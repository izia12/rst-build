use calamine::XlsxError;
use lazy_static::lazy_static;
use map_macro::hash_map;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
// static SORTAMENT: phf::Map<u32, f32>=phf_map!{
// 	6=>1.0,
// };
// #[derive(Clone, Debug, Serialize, Deserialize)]

// Структура для работы с сортаментом арматуры
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DiameterInfo {
    pub diameter: u32,
    pub area: f32,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Sortament {
    data: HashMap<u32, f32>,
}

impl Sortament {
    // Создание нового экземпляра
    pub fn new() -> Self {
        Self {
            data: hash_map! {
				3=>0.071,
				4=>0.126,
                6 => 0.283,
                8 => 0.503,
                10 => 0.785,
                12 => 1.131,
                14 => 1.54,
                16 => 2.01,
                18 => 2.54,
                20 => 3.14,
                22 => 3.8,
                25 => 4.91,
                28 => 6.16,
                32 => 8.01,
                36 => 10.18,
                40 => 12.57,
                45 => 15.0,
                50 => 19.63,
                55 => 23.76,
                60 => 28.27,
                70 => 38.48,
                80 => 50.27
            },
        }
    }
    pub fn to_array(&self) -> Vec<DiameterInfo> {
        let mut result = Vec::new();
        let mut diameters: Vec<u32> = self.data.keys().copied().collect();
        diameters.sort(); // Сортируем для предсказуемого порядка
        
        for &diameter in &diameters {
            if let Some(area) = self.get_area(diameter) {
                result.push(DiameterInfo {
                    diameter,
                    area,
                });
            }
        }
        
        result
    }
    // Получение площади по диаметру
    pub fn get_area(&self, diameter: u32) -> Option<f32> {
        self.data.get(&diameter).copied()
    }

    // Получение всех доступных диаметров
    pub fn get_diameters(&self) -> Vec<u32> {
        let mut diameters: Vec<u32> = self.data.keys().copied().collect();
        diameters.sort(); // Сортируем для предсказуемого порядка
        diameters
    }

    // Поиск комбинаций диаметров, сумма площадей которых >= target_area, но не более чем на 10%
// ... existing code ...
pub fn find_combinations_for_area(
    &self,
    target_area: f32,
    main_step: f32,
    secondary_step: f32,
) -> Vec<(u32, u32, f32)> { // Изменен возвращаемый тип, добавлено total_area
    let max_area = target_area * 1.2; // Максимально допустимая площадь (+20%)
    let mut result = Vec::new();
    let diameters = self.get_diameters();
    
    // Количество стержней на 1 метр для каждого шага
    let main_count = 1.0 / main_step;
    let secondary_count = 1.0 / secondary_step;
    
    // Сначала пробуем найти комбинации с учетом обоих шагов
    for &d1 in &diameters {
        let area1 = self.get_area(d1).unwrap_or(0.0);
        for &d2 in &diameters {
            let area2 = self.get_area(d2).unwrap_or(0.0);
            // Вычисляем общую площадь с учетом шагов
            let total_area = main_count * area1 + secondary_count * area2;
            
            // Сохраняем все комбинации для последующей сортировки
            result.push((d1, d2, total_area));
        }
    }
    
    // Сортируем по отклонению от целевой площади (по абсолютной величине)
    result.sort_by(|&(_, _, area_a), &(_, _, area_b)| {
        let deviation_a = (area_a - target_area).abs();
        let deviation_b = (area_b - target_area).abs();
        deviation_a.partial_cmp(&deviation_b).unwrap_or(std::cmp::Ordering::Equal)
    });
    
    // Проверяем, есть ли комбинации с отклонением менее 20%
    let mut valid_combinations: Vec<(u32, u32, f32)> = result
        .iter()
        .filter(|&&(_, _, area)| area >= target_area && area <= max_area)
        .map(|&(d1, d2, area)| (d1, d2, area))
        .take(8) // Берем 8 лучших комбинаций
        .collect();
    
    // План Б: если не нашли подходящих комбинаций, игнорируем secondary_step
	// ... existing code ...
// План Б: если не нашли подходящих комбинаций, игнорируем secondary_step
if valid_combinations.is_empty() {
    println!("Не найдено комбинаций с отклонением менее 20%. Применяем план Б - игнорируем дополнительную арматуру.");
    // Очищаем предыдущие результаты
    result.clear();
    
    // Ищем комбинации только с основным шагом (secondary_count = 0)
    for &d1 in &diameters {
        let area1 = self.get_area(d1).unwrap_or(0.0);
        // Вычисляем общую площадь только с основным шагом
        let total_area = main_count * area1;
        
        // Добавляем комбинацию с нулевым вторым диаметром
        result.push((d1, 0, total_area));
    }
    
    // Сортируем все результаты по площади (от меньшей к большей)
    result.sort_by(|&(_, _, area_a), &(_, _, area_b)| {
        area_a.partial_cmp(&area_b).unwrap_or(std::cmp::Ordering::Equal)
    });
    
    // Находим индекс первого элемента с положительным отклонением
    let positive_index = result.iter().position(|&(_, _, area)| area >= target_area);
    
    let mut final_combinations = Vec::new();
    
    if let Some(pos_idx) = positive_index {
        // Если есть элемент с отрицательным отклонением перед положительным
        if pos_idx > 0 {
            final_combinations.push(result[pos_idx - 1]); // Ближайший отрицательный
        }
        
        // Добавляем первый положительный
        final_combinations.push(result[pos_idx]);
        
        // Добавляем следующий положительный, если он существует
        if pos_idx + 1 < result.len() {
            final_combinations.push(result[pos_idx + 1]);
        }
    } else {
        // Если все отклонения отрицательные, берем последние два (ближайшие к целевой площади)
        if result.len() >= 2 {
            final_combinations.push(result[result.len() - 2]);
            final_combinations.push(result[result.len() - 1]);
        } else if !result.is_empty() {
            final_combinations.push(result[result.len() - 1]);
        }
    }
    
    valid_combinations = final_combinations;
}
// ... existing code ...
    
    valid_combinations
}
pub fn find_optimal_combination_for_area(
	&self,
    target_area: f32,
    main_step: f32,
    secondary_step: f32,
) -> Vec<(u32, u32, f32)> {
    let max_area = target_area * 1.2; // Максимально допустимая площадь (+20%)
    let mut result = Vec::new();
    
    // Получаем все доступные диаметры из сортамента
    let diameters: Vec<u32> = self.data.keys().copied().collect();
    
    // Количество стержней на 1 метр для каждого шага
    let main_count = 1.0 / main_step;
    let secondary_count = 1.0 / secondary_step;
    
    // Сначала пробуем найти комбинации с учетом обоих шагов
    for &d1 in &diameters {
        let area1 = self.get_area(d1).unwrap_or(0.0);
        for &d2 in &diameters {
            let area2 = self.get_area(d2).unwrap_or(0.0);
            // Вычисляем общую площадь с учетом шагов
            let total_area = main_count * area1 + secondary_count * area2;
            
            // Сохраняем все комбинации для последующей сортировки
            result.push((d1, d2, total_area));
        }
    }
    
    // Сортируем по отклонению от целевой площади (по абсолютной величине)
    result.sort_by(|&(_, _, area_a), &(_, _, area_b)| {
        let deviation_a = (area_a - target_area).abs();
        let deviation_b = (area_b - target_area).abs();
        deviation_a.partial_cmp(&deviation_b).unwrap_or(std::cmp::Ordering::Equal)
    });
    
    // Проверяем, есть ли комбинации с отклонением менее 20%
    let mut valid_combinations: Vec<(u32, u32, f32)> = result
        .iter()
        .filter(|&&(_, _, area)| area >= target_area && area <= max_area)
        .map(|&(d1, d2, area)| (d1, d2, area))
        .collect(); // Берем все подходящие комбинации, без ограничения
    
    // План Б: если не нашли подходящих комбинаций, игнорируем secondary_step
    if valid_combinations.is_empty() {
        println!("Не найдено комбинаций с отклонением менее 20%. Применяем план Б - игнорируем дополнительную арматуру.");
        // Очищаем предыдущие результаты
        result.clear();
        // Ищем комбинации только с основным шагом (secondary_count = 0)
        for &d1 in &diameters {
            let area1 = self.get_area(d1).unwrap_or(0.0);
            // Вычисляем общую площадь только с основным шагом
            let total_area = main_count * area1;
            
            // Добавляем комбинацию с нулевым вторым диаметром
            result.push((d1, 0, total_area));
        }
        
        // Сортируем все результаты по площади (от меньшей к большей)
        result.sort_by(|&(_, _, area_a), &(_, _, area_b)| {
            area_a.partial_cmp(&area_b).unwrap_or(std::cmp::Ordering::Equal)
        });
        
        // Находим индекс первого элемента с положительным отклонением
        let positive_index = result.iter().position(|&(_, _, area)| area >= target_area);
        
        // Выбираем только один оптимальный вариант для Плана Б
        if let Some(pos_idx) = positive_index {
            // Возвращаем только первый положительный (ближайший к нулю)
            valid_combinations = vec![result[pos_idx]];
        } else if !result.is_empty() {
            // Если все отрицательные, возвращаем ближайший к целевой площади
            valid_combinations = vec![result[result.len() - 1]];
        }
    }
    
    valid_combinations
}

	pub fn find_combinations_for_area_with_custom_diameters(
		&self,
		target_area: f32,
		main_step: f32,
		secondary_step: f32,
		available_diameters: &[u32],
	) -> Vec<(u32, u32)> {
		let max_area = target_area * 1.2; // Максимально допустимая площадь (+20%)
		let mut result = Vec::new();
		
		// Используем только доступные диаметры, переданные пользователем
		let diameters: Vec<u32> = available_diameters
			.iter()
			.filter(|&&d| self.get_area(d).is_some()) // Проверяем, что диаметр есть в сортаменте
			.copied()
			.collect();
		
		// Если список диаметров пуст, возвращаем пустой результат
		if diameters.is_empty() {
			return Vec::new();
		}
		
		// Количество стержней на 1 метр для каждого шага
		let main_count = 1.0 / main_step;
		let secondary_count = 1.0 / secondary_step;
		
		// Сначала пробуем найти комбинации с учетом обоих шагов
		for &d1 in &diameters {
			let area1 = self.get_area(d1).unwrap_or(0.0);
			for &d2 in &diameters {
				let area2 = self.get_area(d2).unwrap_or(0.0);
				// Вычисляем общую площадь с учетом шагов
				let total_area = main_count * area1 + secondary_count * area2;
				
				// Сохраняем все комбинации для последующей сортировки
				result.push((d1, d2, total_area));
			}
		}
		
		// Сортируем по отклонению от целевой площади (по абсолютной величине)
		result.sort_by(|&(_, _, area_a), &(_, _, area_b)| {
			let deviation_a = (area_a - target_area).abs();
			let deviation_b = (area_b - target_area).abs();
			deviation_a.partial_cmp(&deviation_b).unwrap_or(std::cmp::Ordering::Equal)
		});
		
		// Проверяем, есть ли комбинации с отклонением менее 20%
		let mut valid_combinations: Vec<(u32, u32)> = result
			.iter()
			.filter(|&&(_, _, area)| area >= target_area && area <= max_area)
			.map(|&(d1, d2, _)| (d1, d2))
			.take(8) // Берем 8 лучших комбинаций
			.collect();
		
		// План Б: если не нашли подходящих комбинаций, игнорируем secondary_step
		if valid_combinations.is_empty() {
			println!("Не найдено комбинаций с отклонением менее 20%. Применяем план Б - игнорируем дополнительную арматуру.");
			
			// Очищаем предыдущие результаты
			result.clear();
			
			// Ищем комбинации только с основным шагом (secondary_count = 0)
			for &d1 in &diameters {
				let area1 = self.get_area(d1).unwrap_or(0.0);
				// Вычисляем общую площадь только с основным шагом
				let total_area = main_count * area1;
				
				// Добавляем комбинацию с нулевым вторым диаметром
				result.push((d1, 0, total_area));
			}
			
			// Сортируем по отклонению от целевой площади
			result.sort_by(|&(_, _, area_a), &(_, _, area_b)| {
				let deviation_a = (area_a - target_area).abs();
				let deviation_b = (area_b - target_area).abs();
				deviation_a.partial_cmp(&deviation_b).unwrap_or(std::cmp::Ordering::Equal)
			});
			
			// Берем 8 лучших комбинаций (или меньше, если их меньше 8)
			valid_combinations = result
				.iter()
				.map(|&(d1, d2, _)| (d1, d2))
				.take(8)
				.collect();
		}
		
		valid_combinations
	}
    // Получение данных в виде HashMap
    pub fn get_data(&self) -> &HashMap<u32, f32> {
        &self.data
    }
	pub fn generate_optimal_test_report(&self, filename: &str) -> std::io::Result<()> {
		let mut file = File::create(filename)?;
	
		// Заголовок таблицы
		writeln!(file, "Целевая площадь;Основной шаг;Доп. шаг;Основная арматура;Доп. арматура;Общая площадь;Отклонение (%)")?;
	
		// Используем те же тестовые данные, что и в оригинальном методе
		let test_cases = [

		(0.1, 0.2, 0.2),
		(0.2, 0.2, 0.2),
		(0.3, 0.2, 0.2),
		(0.4, 0.2, 0.2),
		(0.5, 0.2, 0.2),
		(0.6, 0.2, 0.2),
		(0.7, 0.2, 0.2),
		(0.8, 0.2, 0.2),
		(0.9, 0.2, 0.2),

		(1.0, 0.4, 0.2),
		(1.1, 0.4, 0.2),
		(1.2, 0.4, 0.2),
		(1.3, 0.4, 0.2),
		(1.4, 0.4, 0.2),
		(1.5, 0.4, 0.2),
		(1.6, 0.4, 0.2),
		(1.7, 0.4, 0.2),
		(1.8, 0.4, 0.2),
		(1.9, 0.4, 0.2),

		(2.0, 0.4, 0.1),
		(2.2, 0.4, 0.1),
		(2.4, 0.4, 0.1),
		(2.6, 0.4, 0.1),
		(2.8, 0.4, 0.1),
		
		// Разные шаги при одинаковой площади
		(3.0, 0.2, 0.2),
		(3.2, 0.2, 0.2),
		(3.4, 0.2, 0.2),
		(3.6, 0.2, 0.2),
		(3.8, 0.2, 0.2),

		(4.0, 0.2, 0.2),
		(4.2, 0.2, 0.2),
		(4.4, 0.2, 0.2),
		(4.6, 0.2, 0.2),
		(4.8, 0.2, 0.2),

		(5.0, 0.2, 0.2),
		(5.2, 0.2, 0.2),
		(5.4, 0.2, 0.2),
		(5.6, 0.2, 0.2),
		(5.8, 0.2, 0.2),

		(6.0, 0.2, 0.2),
		(6.2, 0.2, 0.2),
		(6.4, 0.2, 0.2),
		(6.6, 0.2, 0.2),
		(6.8, 0.2, 0.2),

		(7.0, 0.2, 0.2),
		(7.2, 0.2, 0.2),
		(7.4, 0.2, 0.2),
		(7.6, 0.2, 0.2),
		(7.8, 0.2, 0.2),

		(8.0, 0.2, 0.2),
		(8.2, 0.2, 0.2),
		(8.4, 0.2, 0.2),
		(8.6, 0.2, 0.2),
		(8.8, 0.2, 0.2),

		(10.0, 0.4, 0.2),
		(20.0, 0.4, 0.2),
		(30.0, 0.4, 0.2),
		(40.0, 0.4, 0.2),
		(50.0, 0.4, 0.2),
		(60.0, 0.4, 0.2),

		(20.0, 0.2, 0.4),
		(40.0, 0.2, 0.4),
		(60.0, 0.2, 0.4),
		(80.0, 0.2, 0.4),

		(100.0, 0.2, 0.4),
		(120.0, 0.2, 0.4),
		(140.0, 0.2, 0.4),
		(160.0, 0.2, 0.4),
		(180.0, 0.2, 0.4),

		(200.0, 0.2, 0.4),
		(220.0, 0.2, 0.4),
		(240.0, 0.2, 0.4),
		(260.0, 0.2, 0.4),
		(280.0, 0.2, 0.4),

		(300.0, 0.2, 0.4),
		(320.0, 0.2, 0.4),
		(340.0, 0.2, 0.4),
		(360.0, 0.2, 0.4),
		(380.0, 0.2, 0.4),

		(400.0, 0.6, 0.3),
		(420.0, 0.6, 0.3),
		(440.0, 0.6, 0.3),
		(460.0, 0.6, 0.3),
		(480.0, 0.6, 0.3),

		(500.0, 0.2, 0.4),
		(520.0, 0.2, 0.4),
		(540.0, 0.2, 0.4),
		(560.0, 0.2, 0.4),
		(580.0, 0.2, 0.4),
		// Добавьте другие тестовые случаи по необходимости
	];
	
		// Для каждого тестового случая находим оптимальную комбинацию и записываем в файл
		for (target_area, main_step, secondary_step) in &test_cases {
			let combinations = self.find_optimal_combination_for_area(*target_area, *main_step, *secondary_step);
			
			if !combinations.is_empty() {
				// Выводим информацию только для первой комбинации с полными данными
				let (d1, d2, total_area) = combinations[0];
				let deviation = ((total_area / target_area) - 1.0) * 100.0;
				
				if d2 > 0 {
					writeln!(
						file,
						"{};{};{};Ø{} мм;Ø{} мм;{:.3};{:.2}",
						target_area, main_step, secondary_step, 
						d1, d2, total_area, deviation
					)?;
				} else {
					writeln!(
						file,
						"{};{};{};Ø{} мм;Нет;{:.3};{:.2}",
						target_area, main_step, secondary_step, 
						d1, total_area, deviation
					)?;
				}
				
				// Для остальных комбинаций не выводим повторяющиеся значения
				for i in 1..combinations.len() {
					let (d1, d2, total_area) = combinations[i];
					let deviation = ((total_area / target_area) - 1.0) * 100.0;
					
					if d2 > 0 {
						writeln!(
							file,
							";;;Ø{} мм;Ø{} мм;{:.3};{:.2}",
							d1, d2, total_area, deviation
						)?;
					} else {
						writeln!(
							file,
							";;;Ø{} мм;Нет;{:.3};{:.2}",
							d1, total_area, deviation
						)?;
					}
				}
			} else {
				writeln!(
					file,
					"{};{};{};Комбинации не найдены;;;;",
					target_area, main_step, secondary_step
				)?;
			}
			
			// Пустая строка между разными тестовыми случаями для лучшей читаемости
			writeln!(file, "")?;
		}
	
		Ok(())
	}
    pub fn generate_test_report(&self, filename: &str) -> std::io::Result<()> {
        let mut file = File::create(filename)?;

        // Заголовок таблицы
        writeln!(file, 
			"Целевая площадь;Основной шаг;Доп. шаг;Основная арматура;
			Доп.арматура;Общая площадь;Отклонение (%);
			Шкала. Диам осн; Шкала. Шаг осн; Шкала. Диам доп; Шкала. Шаг доп; Шкала площадь"
		)?;

        // Тестовые данные - различные комбинации целевой площади и шагов
        let test_cases = [

            (0.1, 0.2, 0.2),
            (0.2, 0.2, 0.2),
            (0.3, 0.2, 0.2),
            (0.4, 0.2, 0.2),
            (0.5, 0.2, 0.2),
            (0.6, 0.2, 0.2),
            (0.7, 0.2, 0.2),
            (0.8, 0.2, 0.2),
            (0.9, 0.2, 0.2),

			(1.0, 0.4, 0.2),
			(1.1, 0.4, 0.2),
			(1.2, 0.4, 0.2),
			(1.3, 0.4, 0.2),
			(1.4, 0.4, 0.2),
			(1.5, 0.4, 0.2),
			(1.6, 0.4, 0.2),
			(1.7, 0.4, 0.2),
			(1.8, 0.4, 0.2),
			(1.9, 0.4, 0.2),

            (2.0, 0.4, 0.1),
            (2.2, 0.4, 0.1),
            (2.4, 0.4, 0.1),
            (2.6, 0.4, 0.1),
            (2.8, 0.4, 0.1),
            
            // Разные шаги при одинаковой площади
            (3.0, 0.2, 0.2),
            (3.2, 0.2, 0.2),
            (3.4, 0.2, 0.2),
            (3.6, 0.2, 0.2),
            (3.8, 0.2, 0.2),

            (4.0, 0.2, 0.2),
            (4.2, 0.2, 0.2),
            (4.4, 0.2, 0.2),
            (4.6, 0.2, 0.2),
            (4.8, 0.2, 0.2),

            (5.0, 0.2, 0.2),
            (5.2, 0.2, 0.2),
            (5.4, 0.2, 0.2),
            (5.6, 0.2, 0.2),
            (5.8, 0.2, 0.2),

            (6.0, 0.2, 0.2),
            (6.2, 0.2, 0.2),
            (6.4, 0.2, 0.2),
            (6.6, 0.2, 0.2),
            (6.8, 0.2, 0.2),

            (7.0, 0.2, 0.2),
            (7.2, 0.2, 0.2),
            (7.4, 0.2, 0.2),
            (7.6, 0.2, 0.2),
            (7.8, 0.2, 0.2),

            (8.0, 0.2, 0.2),
            (8.2, 0.2, 0.2),
            (8.4, 0.2, 0.2),
            (8.6, 0.2, 0.2),
            (8.8, 0.2, 0.2),

            (10.0, 0.4, 0.2),
            (20.0, 0.4, 0.2),
            (30.0, 0.4, 0.2),
            (40.0, 0.4, 0.2),
            (50.0, 0.4, 0.2),
            (60.0, 0.4, 0.2),

            (20.0, 0.2, 0.4),
            (40.0, 0.2, 0.4),
            (60.0, 0.2, 0.4),
            (80.0, 0.2, 0.4),

            (100.0, 0.2, 0.4),
            (120.0, 0.2, 0.4),
            (140.0, 0.2, 0.4),
            (160.0, 0.2, 0.4),
            (180.0, 0.2, 0.4),

            (200.0, 0.2, 0.4),
            (220.0, 0.2, 0.4),
            (240.0, 0.2, 0.4),
            (260.0, 0.2, 0.4),
            (280.0, 0.2, 0.4),

            (300.0, 0.2, 0.4),
            (320.0, 0.2, 0.4),
            (340.0, 0.2, 0.4),
            (360.0, 0.2, 0.4),
            (380.0, 0.2, 0.4),

            (400.0, 0.6, 0.3),
            (420.0, 0.6, 0.3),
            (440.0, 0.6, 0.3),
            (460.0, 0.6, 0.3),
            (480.0, 0.6, 0.3),

            (500.0, 0.2, 0.4),
            (520.0, 0.2, 0.4),
            (540.0, 0.2, 0.4),
            (560.0, 0.2, 0.4),
            (580.0, 0.2, 0.4),
            // Добавьте другие тестовые случаи по необходимости
        ];

        // Для каждого тестового случая находим комбинации и записываем в файл
        for (target_area, main_step, secondary_step) in &test_cases {
			let combinations =
				self.find_combinations_for_area(*target_area, *main_step, *secondary_step);
	
			if combinations.is_empty() {
				writeln!(
					file,
					"{};{};{};Нет;;;;",
					target_area, main_step, secondary_step
				)?;
			} else {
				let limit = combinations.len();
				let plan_b_active = combinations[0].1 == 0; // Проверяем, активен ли план Б
	
				for i in 0..limit {
					let (d1, d2, total_area) = combinations[i];
					let deviation = ((total_area / target_area) - 1.0) * 100.0;
					// Записываем строку в формате CSV с разделителем ;
					if i == 0 {
						if d2 > 0 {
							writeln!(
								file,
								"{};{};{};Ø{} мм;Ø{} мм;{:.3};{:.2}",
								target_area, main_step, secondary_step, 
								d1, d2, total_area, deviation
							)?;
						} else {
							writeln!(
								file,
								"{};{};{};Ø{} мм;Нет;{:.3};{:.2}",
								target_area, main_step, secondary_step, 
								d1, total_area, deviation
							)?;
						}
					} else {
						if d2 > 0 {
							writeln!(
								file,
								";;;Ø{} мм;Ø{} мм;{:.3};{:.2}",
								d1, d2, total_area, deviation
							)?;
						} else {
							writeln!(
								file,
								";;;Ø{} мм;Нет;{:.3};{:.2}",
								d1, total_area, deviation
							)?;
						}
					}
				}
			}
	
			// Пустая строка между разными тестовыми случаями для лучшей читаемости
			writeln!(file, "")?;
		}
        Ok(())
    }
	pub fn generate_excel_report(&self, filename: &str) -> Result<(), XlsxError> {
		use rust_xlsxwriter::{Workbook, Format, Color, XlsxError};
		// Создаем новый файл Excel
		let mut workbook = Workbook::new();
		// Добавляем лист
		let worksheet = workbook.add_worksheet();
		// Создаем форматы для заголовков и данных
		let header_format = Format::new().set_bold().set_align(rust_xlsxwriter::FormatAlign::Center);
		let yellow_fill = Format::new().set_background_color(Color::RGB(0xFFFF00));
		// Форматы для чисел с фиксированным количеством десятичных знаков
		let step_format = Format::new().set_num_format("0.0");
		let area_format = Format::new().set_num_format("0.000");
		let deviation_format = Format::new().set_num_format("0.0");
		// Задаем ширину колонок
		worksheet.set_column_width(0, 15);
		worksheet.set_column_width(1, 15);
		worksheet.set_column_width(2, 15);
		worksheet.set_column_width(3, 15);
		worksheet.set_column_width(4, 15);
		worksheet.set_column_width(5, 15);
		worksheet.set_column_width(6, 15);
		worksheet.set_column_width(7, 15);
		worksheet.set_column_width(8, 15);
		worksheet.set_column_width(9, 15);
		worksheet.set_column_width(10, 15);
		worksheet.set_column_width(11, 15);
		// Записываем заголовки
		worksheet.write_with_format(0, 0, "Целевая площадь", &header_format);
		worksheet.write_with_format(0, 1, "Основной шаг", &header_format);
		worksheet.write_with_format(0, 2, "Доп. шаг", &header_format);
		worksheet.write_with_format(0, 3, "Основная арматура", &header_format);
		worksheet.write_with_format(0, 4, "Доп. арматура", &header_format);
		worksheet.write_with_format(0, 5, "Общая площадь", &header_format);
		worksheet.write_with_format(0, 6, "Отклонение (%)", &header_format);
		worksheet.write_with_format(0, 7, "Шкала. Диам осн", &header_format);
		worksheet.write_with_format(0, 8, "Шкала. Шаг осн", &header_format);
		worksheet.write_with_format(0, 9, "Шкала. Диам доп", &header_format);
		worksheet.write_with_format(0, 10, "Шкала. Шаг доп", &header_format);
		worksheet.write_with_format(0, 11, "Шкала площадь", &header_format);
		// Используем те же тестовые данные, что и в оригинальном методе
		let test_cases = [

		(0.1, 0.2, 0.2),
		(0.2, 0.2, 0.2),
		(0.3, 0.2, 0.2),
		(0.4, 0.2, 0.2),
		(0.5, 0.2, 0.2),
		(0.6, 0.2, 0.2),
		(0.7, 0.2, 0.2),
		(0.8, 0.2, 0.2),
		(0.9, 0.2, 0.2),

		(1.0, 0.4, 0.2),
		(1.1, 0.4, 0.2),
		(1.2, 0.4, 0.2),
		(1.3, 0.4, 0.2),
		(1.4, 0.4, 0.2),
		(1.5, 0.4, 0.2),
		(1.6, 0.4, 0.2),
		(1.7, 0.4, 0.2),
		(1.8, 0.4, 0.2),
		(1.9, 0.4, 0.2),

		(2.0, 0.4, 0.1),
		(2.2, 0.4, 0.1),
		(2.4, 0.4, 0.1),
		(2.6, 0.4, 0.1),
		(2.8, 0.4, 0.1),
		
		// Разные шаги при одинаковой площади
		(3.0, 0.2, 0.2),
		(3.2, 0.2, 0.2),
		(3.4, 0.2, 0.2),
		(3.6, 0.2, 0.2),
		(3.8, 0.2, 0.2),

		(4.0, 0.2, 0.2),
		(4.2, 0.2, 0.2),
		(4.4, 0.2, 0.2),
		(4.6, 0.2, 0.2),
		(4.8, 0.2, 0.2),

		(5.0, 0.2, 0.2),
		(5.2, 0.2, 0.2),
		(5.4, 0.2, 0.2),
		(5.6, 0.2, 0.2),
		(5.8, 0.2, 0.2),

		(6.0, 0.2, 0.2),
		(6.2, 0.2, 0.2),
		(6.4, 0.2, 0.2),
		(6.6, 0.2, 0.2),
		(6.8, 0.2, 0.2),

		(7.0, 0.2, 0.2),
		(7.2, 0.2, 0.2),
		(7.4, 0.2, 0.2),
		(7.6, 0.2, 0.2),
		(7.8, 0.2, 0.2),

		(8.0, 0.2, 0.2),
		(8.2, 0.2, 0.2),
		(8.4, 0.2, 0.2),
		(8.6, 0.2, 0.2),
		(8.8, 0.2, 0.2),

		(10.0, 0.4, 0.2),
		(20.0, 0.4, 0.2),
		(30.0, 0.4, 0.2),
		(40.0, 0.4, 0.2),
		(50.0, 0.4, 0.2),
		(60.0, 0.4, 0.2),

		(20.0, 0.2, 0.4),
		(40.0, 0.2, 0.4),
		(60.0, 0.2, 0.4),
		(80.0, 0.2, 0.4),

		(100.0, 0.2, 0.4),
		(120.0, 0.2, 0.4),
		(140.0, 0.2, 0.4),
		(160.0, 0.2, 0.4),
		(180.0, 0.2, 0.4),

		(200.0, 0.2, 0.4),
		(220.0, 0.2, 0.4),
		(240.0, 0.2, 0.4),
		(260.0, 0.2, 0.4),
		(280.0, 0.2, 0.4),

		(300.0, 0.2, 0.4),
		(320.0, 0.2, 0.4),
		(340.0, 0.2, 0.4),
		(360.0, 0.2, 0.4),
		(380.0, 0.2, 0.4),

		(400.0, 0.6, 0.3),
		(420.0, 0.6, 0.3),
		(440.0, 0.6, 0.3),
		(460.0, 0.6, 0.3),
		(480.0, 0.6, 0.3),

		(500.0, 0.2, 0.4),
		(520.0, 0.2, 0.4),
		(540.0, 0.2, 0.4),
		(560.0, 0.2, 0.4),
		(580.0, 0.2, 0.4),
		// Добавьте другие тестовые случаи по необходимости
	];
		// Получаем все доступные диаметры из сортамента
		let mut diameters = self.get_diameters();
		let mut row = 1; // Начинаем с первой строки (после заголовков)
		// Для каждого тестового случая находим комбинации и записываем в файл
		for (target_area, main_step, secondary_step) in &test_cases {
			let combinations = self.find_combinations_for_area(*target_area, *main_step, *secondary_step);
			if combinations.is_empty() {
				// Случай А: Комбинация не найдена, используем только основную арматуру
				// Находим ближайшую основную арматуру к target_area
				let main_count = 1.0 / main_step;
				// Находим диаметр, который дает площадь, ближайшую к target_area
				let mut best_d = 0;
				let mut best_area_diff = f32::MAX;
				for &d in &diameters {
					if let Some(area) = self.get_area(d) {
						let total_area = main_count * area;
						let area_diff = (total_area - target_area).abs();
						if area_diff < best_area_diff {
							best_area_diff = area_diff;
							best_d = d;
						}
					}
				}
				if best_d > 0 {
					let area = self.get_area(best_d).unwrap_or(0.0);
					let total_area = main_count * area;
					let deviation = ((total_area / target_area) - 1.0) * 100.0;
					// Записываем основную строку
					worksheet.write_with_format(row, 0, *target_area, &area_format);
					worksheet.write_with_format(row, 1, *main_step, &step_format);
					worksheet.write_with_format(row, 2, *secondary_step, &step_format);
					worksheet.write(row, 3, format!("Ø{} мм", best_d));
					worksheet.write(row, 4, "Нет");
					worksheet.write_with_format(row, 5, total_area, &area_format);
					worksheet.write_with_format(row, 6, deviation, &deviation_format);
					// Переходим к следующей строке для шкал
					row += 1;
					// Заполняем ячейки с желтым фоном для случая без доп. арматуры
					worksheet.write_with_format(row, 7, format!("Ø{} мм", best_d), &yellow_fill);
					worksheet.write_with_format(row, 8, *main_step, &yellow_fill);
					worksheet.write_with_format(row, 9, "Нет", &yellow_fill);
					worksheet.write_with_format(row, 10, *secondary_step, &yellow_fill);
					worksheet.write_with_format(row, 11, total_area, &yellow_fill);
					// Переходим к следующей строке для следующего тестового случая
					row += 1;
				} else {
					// Если не нашли подходящий диаметр
					worksheet.write_with_format(row, 0, *target_area, &area_format);
					worksheet.write_with_format(row, 1, *main_step, &step_format);
					worksheet.write_with_format(row, 2, *secondary_step, &step_format);
					worksheet.write(row, 3, "Нет");
					worksheet.write(row, 4, "Нет");
					worksheet.write_with_format(row, 5, 0.0, &area_format);
					worksheet.write_with_format(row, 6, 0.0, &deviation_format);
					// Переходим к следующей строке для следующего тестового случая
					row += 1;
				}
			} else {
				// Случай Б: Нашлись комбинации
				let limit = combinations.len();
				for i in 0..limit {
					let (d1, d2, total_area) = combinations[i];
					let deviation = ((total_area / target_area) - 1.0) * 100.0;
					// Записываем строку с информацией о комбинации
					if i == 0 {
						// Первая строка с полной информацией
						worksheet.write_with_format(row, 0, *target_area, &area_format);
						worksheet.write_with_format(row, 1, *main_step, &step_format);
						worksheet.write_with_format(row, 2, *secondary_step, &step_format);
					} else {
						// Для последующих комбинаций не заполняем первые три колонки
						worksheet.write(row, 0, "");
						worksheet.write(row, 1, "");
						worksheet.write(row, 2, "");
					}
					if d2 > 0 {
						worksheet.write(row, 3, format!("Ø{} мм", d1));
						worksheet.write(row, 4, format!("Ø{} мм", d2));
					} else {
						worksheet.write(row, 3, format!("Ø{} мм", d1));
						worksheet.write(row, 4, "Нет");
					}
					// Используем значение total_area из комбинации, которое уже рассчитано правильно
					worksheet.write_with_format(row, 5, total_area, &area_format);
					worksheet.write_with_format(row, 6, deviation, &deviation_format);
					// Переходим к следующей строке для шкал
					row += 1;
					// Заполняем шкалу диаметров по новым требованиям
					if d2 > 0 {
						// Случай с дополнительной арматурой
						let main_count = 1.0 / main_step;
						let secondary_count = 1.0 / secondary_step;
						let area1 = self.get_area(d1).unwrap_or(0.0);
						// Сначала добавляем случай без дополнительной арматуры (0)
						let main_only_area = main_count * area1;
						worksheet.write_with_format(row, 7, format!("Ø{} мм", d1), &yellow_fill);
						worksheet.write_with_format(row, 8, *main_step, &step_format);
						worksheet.write_with_format(row, 9, "Нет", &yellow_fill);
						worksheet.write_with_format(row, 10, *secondary_step, &step_format);
						worksheet.write_with_format(row, 11, main_only_area, &area_format);
						row += 1;
						// Теперь перебираем все диаметры от минимального до d2
						for &curr_d in &diameters {
							// Пропускаем диаметры больше d2
							if curr_d > d2 {
								break;
							}
							// Пропускаем 0 (уже обработали случай без доп. арматуры)
							if curr_d == 0 {
								continue;
							}
							let area_curr = self.get_area(curr_d).unwrap_or(0.0);
							let combined_area = main_count * area1 + secondary_count * area_curr;
							worksheet.write_with_format(row, 7, format!("Ø{} мм", d1), &yellow_fill);
							worksheet.write_with_format(row, 8, *main_step, &step_format);
							worksheet.write_with_format(row, 9, format!("Ø{} мм", curr_d), &yellow_fill);
							worksheet.write_with_format(row, 10, *secondary_step, &step_format);
							worksheet.write_with_format(row, 11, combined_area, &area_format);
							row += 1;
						}
					} else {
						// Случай без дополнительной арматуры
						let main_count = 1.0 / main_step;
						let area1 = self.get_area(d1).unwrap_or(0.0);
						let main_only_area = main_count * area1;
						worksheet.write_with_format(row, 7, format!("Ø{} мм", d1), &yellow_fill);
						worksheet.write_with_format(row, 8, *main_step, &step_format);
						worksheet.write_with_format(row, 9, "Нет", &yellow_fill);
						worksheet.write_with_format(row, 10, *secondary_step, &step_format);
						worksheet.write_with_format(row, 11, main_only_area, &area_format);
						row += 1;
					}
				}
			}
			// Добавляем пустую строку между разными тестовыми случаями для лучшей читаемости
			row += 1;
		}
		// Сохраняем файл
		workbook.save(filename);
		Ok(())
	}
}

// Создаем глобальный экземпляр для использования в проекте

// Создаем глобальный экземпляр для использования в проекте
lazy_static! {
    pub static ref SORTAMENT: Sortament = Sortament::new();
}
// #[cfg(not(test))]
// fn main() {
//     println!("Тестирование функции find_combinations_for_area");

//     // Тестируем для площади 3.1 с шагами 0.2 и 0.25
//     let target_area = 5.1;
//     let main_step = 0.25; // 5 штук на метр
//     let secondary_step = 0.25; // 4 штуки на метр

//     let combinations = SORTAMENT.find_combinations_for_area(target_area, main_step, secondary_step);

//     println!("Комбинации для площади {}: найдено {} вариантов", target_area, combinations.len());
//     println!("Основной шаг: {} м ({} шт/м), дополнительный шаг: {} м ({} шт/м)",
//              main_step, 1.0/main_step, secondary_step, 1.0/secondary_step);

//     for (i, (d1, d2)) in combinations.iter().enumerate() {
//         let area1 = SORTAMENT.get_area(*d1).unwrap_or(0.0);
//         let area2 = SORTAMENT.get_area(*d2).unwrap_or(0.0);

//         let main_count = 1.0 / main_step;
//         let secondary_count = 1.0 / secondary_step;

//         let total_area = main_count * area1 + secondary_count * area2;

//         println!("Вариант {}: основная Ø{} мм, дополнительная Ø{} мм, общая площадь: {:.3}, отклонение: {:.2}%",
//                  i + 1, d1, d2, total_area, (total_area / target_area - 1.0) * 100.0);
//     }
// }
fn main1() {
    println!("Тестирование функции find_combinations_for_area");

    // Тестируем для площади 3.1 с шагами 0.2 и 0.25
    let target_area = 3.1;
    let main_step = 0.2; // 5 штук на метр
    let secondary_step = 0.25; // 4 штуки на метр

    let combinations = SORTAMENT.find_combinations_for_area(target_area, main_step, secondary_step);

    println!(
        "Комбинации для площади {}: найдено {} вариантов",
        target_area,
        combinations.len()
    );
    println!(
        "Основной шаг: {} м ({} шт/м), дополнительный шаг: {} м ({} шт/м)",
        main_step,
        1.0 / main_step,
        secondary_step,
        1.0 / secondary_step
    );

    for (i, (d1, d2,_)) in combinations.iter().enumerate() {
        let area1 = SORTAMENT.get_area(*d1).unwrap_or(0.0);
        let area2 = SORTAMENT.get_area(*d2).unwrap_or(0.0);

        let main_count = 1.0 / main_step;
        let secondary_count = 1.0 / secondary_step;

        let total_area = main_count * area1 + secondary_count * area2;

        println!("Вариант {}: основная Ø{} мм, дополнительная Ø{} мм, общая площадь: {:.3}, отклонение: {:.2}%", 
                 i + 1, d1, d2, total_area, (total_area / target_area - 1.0) * 100.0);
    }

    // Генерируем тестовый файл с таблицей результатов
    match SORTAMENT.generate_optimal_test_report("armature_combinations6.csv") {
        Ok(_) => println!("Тестовый файл успешно создан: armature_combinations.csv"),
        Err(e) => println!("Ошибка при создании тестового файла: {}", e),
    }
}
fn main() {
    println!("Тестирование функции find_combinations_for_area");

    // Тестируем для площади 3.1 с шагами 0.2 и 0.25
    let target_area = 3.1;
    let main_step = 0.2; // 5 штук на метр
    let secondary_step = 0.25; // 4 штуки на метр

    let combinations = SORTAMENT.find_combinations_for_area(target_area, main_step, secondary_step);

    println!(
        "Комбинации для площади {}: найдено {} вариантов",
        target_area,
        combinations.len()
    );
    println!(
        "Основной шаг: {} м ({} шт/м), дополнительный шаг: {} м ({} шт/м)",
        main_step,
        1.0 / main_step,
        secondary_step,
        1.0 / secondary_step
    );

    for (i, (d1, d2, _)) in combinations.iter().enumerate() {
        let area1 = SORTAMENT.get_area(*d1).unwrap_or(0.0);
        let area2 = SORTAMENT.get_area(*d2).unwrap_or(0.0);

        let main_count = 1.0 / main_step;
        let secondary_count = 1.0 / secondary_step;

        let total_area = main_count * area1 + secondary_count * area2;

        println!("Вариант {}: основная Ø{} мм, дополнительная Ø{} мм, общая площадь: {:.3}, отклонение: {:.2}%", 
                 i + 1, d1, d2, total_area, (total_area / target_area - 1.0) * 100.0);
    }

    // Генерируем тестовый файл с таблицей результатов в формате CSV
    match SORTAMENT.generate_optimal_test_report("armature_combinations6.csv") {
        Ok(_) => println!("Тестовый CSV файл успешно создан: armature_combinations6.csv"),
        Err(e) => println!("Ошибка при создании тестового CSV файла: {}", e),
    }
    
    // Генерируем тестовый файл с таблицей результатов в формате Excel
    match SORTAMENT.generate_excel_report("armature_combinations6565.xlsx") {
        Ok(_) => println!("Тестовый Excel файл успешно создан: armature_combinations.xlsx"),
        Err(e) => println!("Ошибка при создании тестового Excel файла: {}", e),
    }
}
// Модульные тесты
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_area() {
        assert_eq!(SORTAMENT.get_area(6), Some(0.283));
        assert_eq!(SORTAMENT.get_area(999), None); // Несуществующий диаметр
    }

    #[test]
    fn test_find_combinations_with_steps() {
        let main_step = 0.2;
        let secondary_step = 0.25;
        let target_area = 5.1;
        let combinations =
            SORTAMENT.find_combinations_for_area(target_area, main_step, secondary_step);
        assert!(!combinations.is_empty(), "Должны быть найдены комбинации");
        // Проверяем, что все комбинации соответствуют условиям
        for &(d1, d2,_) in &combinations {
            let area1 = SORTAMENT.get_area(d1).unwrap_or(0.0);
            let area2 = SORTAMENT.get_area(d2).unwrap_or(0.0);
            let main_count = 1.0 / main_step;
            let secondary_count = 1.0 / secondary_step;
            let total_area = main_count * area1 + secondary_count * area2;
            assert!(
                total_area >= target_area,
                "Площадь должна быть >= {}",
                target_area
            );
            assert!(
                total_area < target_area * 1.2,
                "Площадь не должна превышать {} * 1.2",
                target_area
            );
        }
    }
}
