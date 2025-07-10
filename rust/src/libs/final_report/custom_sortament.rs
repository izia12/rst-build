use std::collections::HashMap;
use rust_xlsxwriter::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use crate::libs::final_report::sortament_data::{get_area_for_diameter};
// #[]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FloorData {
    pub title: Option<String>,
    pub level: String,
    pub max_as1: f32,
    pub max_as2: f32,
    pub max_as3: f32,
    pub max_as4: f32,
    pub steps: [f32; 2], // [mainStep, secondaryStep]
    pub color: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExcelView {
    pub level: String,
    pub title: Option<String>,
    pub main_step: f32,
    pub additional_step: f32,
    pub values: Vec<ArmatureCombination>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArmatureCombination {
    pub function_name: String, // "as1", "as2", "as3", "as4"
    pub as_target_value: f32,
    pub combinations: Vec<CombinationItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombinationItem {
    pub main_diameter: u32,
    pub additional_diameter: u32, // 0 если нет дополнительной арматуры
    pub total_area: f32,
    pub deviation: f32,
}

#[derive(Debug, Clone)]
pub struct CustomSortament {
     diameter_area_map: HashMap<u32, f32>,
}

#[derive(Debug, Clone)]
pub struct CombinationResult {
    pub main_diameter: u32,
    pub main_count: u32,
    pub secondary_diameter: u32,
    pub secondary_count: u32,
    pub total_area: f32,     // Изменить f64 на f32
    pub deviation: f32,   
}

impl CustomSortament {
    /// Создает новый CustomSortament из JavaScript данных о доступных диаметрах
    pub fn from_js_data(available_diameters: Vec<u32>) -> Self {
        let mut diameter_area_map = HashMap::new();
        // Заполняем карту только для доступных диаметров из стандартного сортамента
	     for diameter in available_diameters {
	        if let Some(area) = get_area_for_diameter(diameter) {
	            diameter_area_map.insert(diameter, area); // Убрать "as f32", так как функция уже возвращает f32
	        } else {
	            eprintln!("Предупреждение: диаметр {} не найден в стандартном сортаменте", diameter);
	        }
	    }
	    CustomSortament {
	        diameter_area_map,
	    }
    }
    pub fn calculate_area_for_diameter(diameter: u32) -> f64 {
        let radius = diameter as f64 / 2.0;
        std::f64::consts::PI * radius * radius
    }
    /// Получает все доступные диаметры
    pub fn get_available_diameters(&self) -> Vec<u32> {
        let mut diameters: Vec<u32> = self.diameter_area_map.keys().cloned().collect();
        diameters.sort();
        diameters
    }
    /// Получает площадь для заданного диаметра
	pub fn get_area(&self, diameter: u32) -> Option<f32> {  // Изменить f64 на f32
	    self.diameter_area_map.get(&diameter).copied()
	}
    /// Находит оптимальную комбинацию арматуры для заданной площади
	pub fn find_combinations_for_area(
	    &self,
	    target_area: f32,
	    main_step: f32,
	    secondary_step: f32,
	) -> Vec<(u32, u32, f32)> {
	    let mut combinations = Vec::new();
	    let mut diameters = self.get_available_diameters();
	    
	    // Для малых площадей (меньше 5.0 см²) исключаем большие диаметры (больше 20 мм)
	    if target_area < 5.0 {
	        diameters.retain(|&d| d <= 20);
	    }
	    
	    let main_count = 1.0 / main_step;
	    let secondary_count = 1.0 / secondary_step;
	    
	    for &d1 in &diameters {
	        if let Some(area1) = self.get_area(d1) {
	            // Проверяем комбинацию только с основной арматурой
	            let main_only_area = main_count * area1;
	            combinations.push((d1, 0, main_only_area));
	            
	            // Проверяем комбинации с дополнительной арматурой
	            for &d2 in &diameters {
	                if let Some(area2) = self.get_area(d2) {
	                    let combined_area = main_count * area1 + secondary_count * area2;
	                    combinations.push((d1, d2, combined_area));
	                }
	            }
	        }
	    }
	    
	    // Сортируем по отклонению от целевой площади
	    combinations.sort_by(|a, b| {
	        let dev_a = (a.2 - target_area).abs();
	        let dev_b = (b.2 - target_area).abs();
	        dev_a.partial_cmp(&dev_b).unwrap_or(std::cmp::Ordering::Equal)
	    });
	    
	    // Возвращаем только лучшие комбинации (например, первые 10)
	    combinations.into_iter().take(10).collect()
	}
	pub fn generate_floors_excel_report_to_wasm(
	    &self,
	    floors_data: Vec<FloorData>,
	) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
	    let mut workbook = Workbook::new();
	    let worksheet = workbook.add_worksheet();
	    
	    // Создаем форматы для заголовков и данных
	    let header_format = Format::new().set_bold().set_align(FormatAlign::Center);
	    let yellow_fill = Format::new().set_background_color(Color::RGB(0xFFFF00));
	    
	    // Форматы для чисел с фиксированным количеством десятичных знаков
	    let step_format = Format::new().set_num_format("0.0");
	    let area_format = Format::new().set_num_format("0.000");
	    let deviation_format = Format::new().set_num_format("0.0");
	    
	    // Задаем ширину колонок (добавляем одну колонку)
	    for col in 0..17 {
	        worksheet.set_column_width(col, 15)?;
	    }
	    
	    // Записываем заголовки (разделяем "Целевая площадь" на две колонки)
	    worksheet.write_with_format(0, 0, "Отметка", &header_format)?;
	    worksheet.write_with_format(0, 1, "название", &header_format)?;
	    worksheet.write_with_format(0, 2, "Функция", &header_format)?;  // Новая колонка для as1, as2, as3, as4
	    worksheet.write_with_format(0, 3, "Целевая площадь", &header_format)?;  // Колонка только для значения
	    worksheet.write_with_format(0, 4, "Основной шаг", &header_format)?;
	    worksheet.write_with_format(0, 5, "Доп. шаг", &header_format)?;
	    worksheet.write_with_format(0, 6, "Основная арматура", &header_format)?;
	    worksheet.write_with_format(0, 7, "Доп. арматура", &header_format)?;
	    worksheet.write_with_format(0, 8, "Общая площадь", &header_format)?;
	    worksheet.write_with_format(0, 9, "Отклонение (%)", &header_format)?;
	    worksheet.write_with_format(0, 10, "Шкала. Диам осн", &header_format)?;
	    worksheet.write_with_format(0, 11, "Шкала. Шаг осн", &header_format)?;
	    worksheet.write_with_format(0, 12, "Шкала. Диам доп", &header_format)?;
	    worksheet.write_with_format(0, 13, "Шкала. Шаг доп", &header_format)?;
	    worksheet.write_with_format(0, 14, "Шкала площадь", &header_format)?;
	    
	    let mut row = 1;
	    
	    // Обрабатываем каждый этаж
	    for floor in floors_data {
	        let areas = [floor.max_as1, floor.max_as2, floor.max_as3, floor.max_as4];
	        let main_step = floor.steps[0] / 1000.0; // Преобразуем из мм в м
	        let secondary_step = floor.steps[1] / 1000.0; // Преобразуем из мм в м
	        
	        // Для каждой области (maxAs1-maxAs4) создаем записи
	        for (area_index, &target_area) in areas.iter().enumerate() {
	            if target_area > 0.0 {
	                // Определяем название области
	                let area_name = match area_index {
	                    0 => "as1",
	                    1 => "as2", 
	                    2 => "as3",
	                    3 => "as4",
	                    _ => "as",
	                };
	                
	                // Находим комбинации для данной площади с учетом ограничений
	                let combinations = self.find_combinations_for_area_with_limits(target_area, floor.steps[0], floor.steps[1]);
	                
	                if combinations.is_empty() {
	                    // Случай А: Комбинация не найдена, используем только основную арматуру
	                    let main_count = 1.0 / main_step;
	                    let mut diameters = self.get_available_diameters();
	                    
	                    // Для малых площадей исключаем большие диаметры
	                    if target_area < 5.0 {
	                        diameters.retain(|&d| d <= 20);
	                    }
	                    
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
	                        worksheet.write_string_with_format(row, 0, &floor.level, &area_format)?;
	                        if let Some(ref title) = floor.title {
	                            worksheet.write_string_with_format(row, 1, title, &area_format)?;
	                        } else {
	                            worksheet.write_string_with_format(row, 1, "", &area_format)?;
	                        }
	                        // Записываем функцию и целевую площадь в разные колонки
	                        worksheet.write_string(row, 2, area_name)?;  // Колонка "Функция"
	                        worksheet.write_with_format(row, 3, target_area, &area_format)?;  // Колонка "Целевая площадь"
	                        worksheet.write_with_format(row, 4, floor.steps[0], &step_format)?;
	                        worksheet.write_with_format(row, 5, floor.steps[1], &step_format)?;
	                        worksheet.write_string(row, 6, &format!("Ø{} мм", best_d))?;
	                        worksheet.write_string(row, 7, "Нет")?;
	                        worksheet.write_with_format(row, 8, total_area, &area_format)?;
	                        worksheet.write_with_format(row, 9, deviation, &deviation_format)?;
	                        
	                        row += 1;
	                        
	                        // Заполняем ячейки с желтым фоном
	                        worksheet.write_string_with_format(row, 10, &format!("Ø{} мм", best_d), &yellow_fill)?;
	                        worksheet.write_with_format(row, 11, floor.steps[0], &yellow_fill)?;
	                        worksheet.write_string_with_format(row, 12, "Нет", &yellow_fill)?;
	                        worksheet.write_with_format(row, 13, floor.steps[1], &yellow_fill)?;
	                        worksheet.write_with_format(row, 14, total_area, &yellow_fill)?;
	                        
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
	                            worksheet.write_string_with_format(row, 0, &floor.level, &area_format)?;
	                            if let Some(ref title) = floor.title {
	                                worksheet.write_string_with_format(row, 1, title, &area_format)?;
	                            } else {
	                                worksheet.write_string_with_format(row, 1, "", &area_format)?;
	                            }
	                            // Записываем функцию и целевую площадь в разные колонки
	                            worksheet.write_string(row, 2, area_name)?;  // Колонка "Функция"
	                            worksheet.write_with_format(row, 3, target_area, &area_format)?;  // Колонка "Целевая площадь"
	                            worksheet.write_with_format(row, 4, floor.steps[0], &step_format)?;
	                            worksheet.write_with_format(row, 5, floor.steps[1], &step_format)?;
	                        } else {
	                            // Для последующих комбинаций не заполняем первые колонки
	                            for col in 0..6 {
	                                worksheet.write_string(row, col, "")?;
	                            }
	                        }
	                        
	                        if d2 > 0 {
	                            worksheet.write_string(row, 6, &format!("Ø{} мм", d1))?;
	                            worksheet.write_string(row, 7, &format!("Ø{} мм", d2))?;
	                        } else {
	                            worksheet.write_string(row, 6, &format!("Ø{} мм", d1))?;
	                            worksheet.write_string(row, 7, "Нет")?;
	                        }
	                        
	                        worksheet.write_with_format(row, 8, total_area, &area_format)?;
	                        worksheet.write_with_format(row, 9, deviation, &deviation_format)?;
	                        
	                        row += 1;
	                        
	                        // Заполняем шкалу диаметров
	                        if d2 > 0 {
	                            // Случай с дополнительной арматурой
	                            let main_count = 1.0 / main_step;
	                            let secondary_count = 1.0 / secondary_step;
	                            let area1 = self.get_area(d1).unwrap_or(0.0);
	                            let mut diameters = self.get_available_diameters();
	                            
	                            // Для малых площадей ограничиваем диаметры
	                            if target_area < 5.0 {
	                                diameters.retain(|&d| d <= 20);
	                            }
	                            
	                            // Сначала добавляем случай без дополнительной арматуры
	                            let main_only_area = main_count * area1;
	                            worksheet.write_string_with_format(row, 10, &format!("Ø{} мм", d1), &yellow_fill)?;
	                            worksheet.write_with_format(row, 11, floor.steps[0], &step_format)?;
	                            worksheet.write_string_with_format(row, 12, "Нет", &yellow_fill)?;
	                            worksheet.write_with_format(row, 13, floor.steps[1], &step_format)?;
	                            worksheet.write_with_format(row, 14, main_only_area, &area_format)?;
	                            row += 1;
	                            
	                            // Теперь перебираем все диаметры от минимального до d2
	                            for &curr_d in &diameters {
	                                if curr_d > d2 || curr_d == 0 {
	                                    continue;
	                                }
	                                
	                                let area_curr = self.get_area(curr_d).unwrap_or(0.0);
	                                let combined_area = main_count * area1 + secondary_count * area_curr;
	                                
	                                worksheet.write_string_with_format(row, 10, &format!("Ø{} мм", d1), &yellow_fill)?;
	                                worksheet.write_with_format(row, 11, floor.steps[0], &step_format)?;
	                                worksheet.write_string_with_format(row, 12, &format!("Ø{} мм", curr_d), &yellow_fill)?;
	                                worksheet.write_with_format(row, 13, floor.steps[1], &step_format)?;
	                                worksheet.write_with_format(row, 14, combined_area, &area_format)?;
	                                row += 1;
	                            }
	                        } else {
	                            // Случай без дополнительной арматуры
	                            let main_count = 1.0 / main_step;
	                            let area1 = self.get_area(d1).unwrap_or(0.0);
	                            let main_only_area = main_count * area1;
	                            
	                            worksheet.write_string_with_format(row, 10, &format!("Ø{} мм", d1), &yellow_fill)?;
	                            worksheet.write_with_format(row, 11, floor.steps[0], &step_format)?;
	                            worksheet.write_string_with_format(row, 12, "Нет", &yellow_fill)?;
	                            worksheet.write_with_format(row, 13, floor.steps[1], &step_format)?;
	                            worksheet.write_with_format(row, 14, main_only_area, &area_format)?;
	                            row += 1;
	                        }
	                    }
	                }
	                row += 1;
	            }
	        }
	    }
	    
	    // Сохраняем в буфер
	    let buffer = workbook.save_to_buffer()?;
	    Ok(buffer)
	}
	// Вспомогательный метод для поиска комбинаций с ограничениями
	fn find_combinations_for_area_with_limits(
	    &self,
	    target_area: f32,
	    main_step: f32,
	    secondary_step: f32,
	) -> Vec<(u32, u32, f32)> {
	    let max_area = target_area * 1.2; // Максимально допустимая площадь (+20%)
	    let mut result = Vec::new();
	    let mut diameters = self.get_available_diameters();
	    
	    // Для малых площадей (меньше 5.0 см²/м) исключаем большие диаметры (больше 20 мм)
	    if target_area < 5.0 {
	        diameters.retain(|&d| d <= 20);
	    }
	    
	    // Преобразуем шаги из миллиметров в метры
	    let main_step_m = main_step / 1000.0;
	    let secondary_step_m = secondary_step / 1000.0;
	    
	    // Количество стержней на 1 метр для каждого шага
	    let main_count = 1.0 / main_step_m;
	    let secondary_count = 1.0 / secondary_step_m;
	    
	    // Сначала пробуем найти комбинации с учетом обоих шагов
	    for &d1 in &diameters {
	        let area1 = self.get_area(d1).unwrap_or(0.0);
	        for &d2 in &diameters {
	            let area2 = self.get_area(d2).unwrap_or(0.0);
	            // Вычисляем общую площадь с учетом шагов (см²/м)
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
	    if valid_combinations.is_empty() {
	        // Очищаем предыдущие результаты
	        result.clear();
	        
	        // Ищем комбинации только с основным шагом (secondary_count = 0)
	        for &d1 in &diameters {
	            let area1 = self.get_area(d1).unwrap_or(0.0);
	            // Вычисляем общую площадь только с основным шагом (см²/м)
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
	    
	    valid_combinations
	}
	pub fn get_simple_numbers() -> Vec<i32> {
    	vec![1, 2, 3, 4, 5, 10, 15, 20, 25, 30]
	}
pub fn generate_excel_data_for_js(
    &self,
    floors_data: Vec<FloorData>,
) -> Vec<ExcelView> {
    let mut result = Vec::new();
    
    // Обрабатываем каждый этаж
    for floor in floors_data {
        let areas = [floor.max_as1, floor.max_as2, floor.max_as3, floor.max_as4];
        let main_step = floor.steps[0] / 1000.0; // Преобразуем из мм в м
        
        let mut armature_combinations = Vec::new();
        
        // Для каждой области (maxAs1-maxAs4) создаем ArmatureCombination
        for (area_index, &target_area) in areas.iter().enumerate() {
            if target_area > 0.0 {
                // Определяем название области
                let function_name = match area_index {
                    0 => "as1",
                    1 => "as2", 
                    2 => "as3",
                    3 => "as4",
                    _ => "as",
                };
                
                // Находим комбинации для данной площади с учетом ограничений
                let combinations = self.find_combinations_for_area_with_limits(
                    target_area, 
                    floor.steps[0], 
                    floor.steps[1]
                );
                
                let mut combination_items = Vec::new();
                
                if combinations.is_empty() {
                    // Случай А: Комбинация не найдена, используем только основную арматуру
                    let main_count = 1.0 / main_step;
                    let mut diameters = self.get_available_diameters();
                    
                    // Для малых площадей исключаем большие диаметры
                    if target_area < 5.0 {
                        diameters.retain(|&d| d <= 20);
                    }
                    
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
                        combination_items.push(CombinationItem {
                            main_diameter: best_d,
                            additional_diameter: 0, // Нет дополнительной арматуры
                            total_area,
                            deviation,
                        });
                    }
                } else {
                    // Случай Б: Нашлись комбинации
                    for &(d1, d2, total_area) in combinations.iter() {
                        let deviation = ((total_area / target_area) - 1.0) * 100.0;
                        
                        combination_items.push(CombinationItem {
                            main_diameter: d1,
                            additional_diameter: d2, // 0 если нет дополнительной арматуры
                            total_area,
                            deviation,
                        });
                    }
                }
                
                // Создаем ArmatureCombination для данной функции
                armature_combinations.push(ArmatureCombination {
                    function_name: function_name.to_string(),
                    as_target_value: target_area,
                    combinations: combination_items,
                });
            }
        }
        
        // Создаем ExcelView для этажа
        if !armature_combinations.is_empty() {
            result.push(ExcelView {
                level: floor.level.clone(),
                title: floor.title.clone(),
                main_step: floor.steps[0],
                additional_step: floor.steps[1],
                values: armature_combinations,
            });
        }
    }
    
    result
}
}

// WASM биндинги
#[wasm_bindgen]
pub fn create_custom_sortament_report(
    available_diameters: Vec<u32>,
    floors_data_json: &str,
) -> Result<Vec<u8>, JsValue> {
    let floors_data: Vec<FloorData> = serde_json::from_str(floors_data_json)
        .map_err(|e| JsValue::from_str(&format!("JSON parsing error: {}", e)))?;
    let sortament = CustomSortament::from_js_data(available_diameters);
    sortament
        .generate_floors_excel_report_to_wasm(floors_data)
        .map_err(|e| JsValue::from_str(&format!("Excel generation error: {}", e)))
}
