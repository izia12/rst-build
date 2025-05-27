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
    pub fn find_combinations_for_area(
        &self,
        target_area: f32,
        main_step: f32,
        secondary_step: f32,
    ) -> Vec<(u32, u32)> {
        let max_area = target_area * 1.2; // Максимально допустимая площадь (+20%)
        let mut result = Vec::new();
        let diameters = self.get_diameters();
        // Количество стержней на 1 метр для каждого шага
        let main_count = 1.0 / main_step;
        let secondary_count = 1.0 / secondary_step;
        for &d1 in &diameters {
            let area1 = self.get_area(d1).unwrap_or(0.0);
            for &d2 in &diameters {
                let area2 = self.get_area(d2).unwrap_or(0.0);
                // Вычисляем общую площадь с учетом шагов
                let total_area = main_count * area1 + secondary_count * area2;
                // Проверяем условие: target_area <= total_area < target_area * 1.2
				result.push((d1, d2));
                // if total_area >= target_area && total_area < max_area {
                // }
            }
        }

        // Сортируем результаты по общей площади (от меньшей к большей)
        result.sort_by(|&(d1a, d2a), &(d1b, d2b)| {
            let area_a = main_count * self.get_area(d1a).unwrap_or(0.0)
                + secondary_count * self.get_area(d2a).unwrap_or(0.0);
            let area_b = main_count * self.get_area(d1b).unwrap_or(0.0)
                + secondary_count * self.get_area(d2b).unwrap_or(0.0);
            area_a
                .partial_cmp(&area_b)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        result
    }

    // Получение данных в виде HashMap
    pub fn get_data(&self) -> &HashMap<u32, f32> {
        &self.data
    }
    pub fn generate_test_report(&self, filename: &str) -> std::io::Result<()> {
        let mut file = File::create(filename)?;

        // Заголовок таблицы
        writeln!(file, "Целевая площадь;Основной шаг;Доп. шаг;Основная арматура;Доп. арматура;Общая площадь;Отклонение (%)")?;

        // Тестовые данные - различные комбинации целевой площади и шагов
        let test_cases = [

            (0.1, 0.1, 0.1),
            (0.2, 0.1, 0.1),
            (0.3, 0.1, 0.1),
            (0.4, 0.1, 0.1),
            (0.5, 0.1, 0.1),
            (0.6, 0.1, 0.1),
            (0.7, 0.1, 0.1),
            (0.8, 0.1, 0.1),
            (0.9, 0.1, 0.1),

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

            // Если комбинации найдены, записываем первые 5 (или меньше) в файл
            let limit = combinations.len().min(20); // Ограничиваем количество выводимых комбинаций

            if combinations.is_empty() {
                writeln!(
                    file,
                    "{};{};{};Комбинации не найдены;;;;",
                    target_area, main_step, secondary_step
                )?;
            } else {
                for i in 0..limit {
                    let (d1, d2) = combinations[i];
                    let area1 = self.get_area(d1).unwrap_or(0.0);
                    let area2 = self.get_area(d2).unwrap_or(0.0);

                    let main_count = 1.0 / main_step;
                    let secondary_count = 1.0 / secondary_step;

                    let total_area = main_count * area1 + secondary_count * area2;
                    let deviation = (total_area / target_area - 1.0) * 100.0;

                    // Записываем строку в формате CSV с разделителем ;
                    if i == 0 {
                        writeln!(
                            file,
                            "{};{};{};Ø{} мм;Ø{} мм;{:.3};{:.2}",
                            target_area, main_step, secondary_step, d1, d2, total_area, deviation
                        )?;
                    } else {
                        writeln!(
                            file,
                            ";;;Ø{} мм;Ø{} мм;{:.3};{:.2}",
                            d1, d2, total_area, deviation
                        )?;
                    }
                }
            }

            // Пустая строка между разными тестовыми случаями для лучшей читаемости
            writeln!(file, "")?;
        }

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

    for (i, (d1, d2)) in combinations.iter().enumerate() {
        let area1 = SORTAMENT.get_area(*d1).unwrap_or(0.0);
        let area2 = SORTAMENT.get_area(*d2).unwrap_or(0.0);

        let main_count = 1.0 / main_step;
        let secondary_count = 1.0 / secondary_step;

        let total_area = main_count * area1 + secondary_count * area2;

        println!("Вариант {}: основная Ø{} мм, дополнительная Ø{} мм, общая площадь: {:.3}, отклонение: {:.2}%", 
                 i + 1, d1, d2, total_area, (total_area / target_area - 1.0) * 100.0);
    }

    // Генерируем тестовый файл с таблицей результатов
    match SORTAMENT.generate_test_report("armature_combinations2.csv") {
        Ok(_) => println!("Тестовый файл успешно создан: armature_combinations.csv"),
        Err(e) => println!("Ошибка при создании тестового файла: {}", e),
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
        for &(d1, d2) in &combinations {
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
