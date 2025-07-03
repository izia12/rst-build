use std::collections::HashMap;
use rust_xlsxwriter::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use crate::libs::final_report::sortament_data::{get_area_for_diameter, is_diameter_valid};

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
    
    /// Вычисляет площадь для заданного диаметра (π * (d/2)²)
    fn calculate_area_for_diameter(diameter: u32) -> f64 {
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
    pub fn find_optimal_combination_for_area(
 	&self,
    target_area: f32,        // Изменить f64 на f32
    main_step: f32,          // Изменить f64 на f32
    secondary_step: f32, 
    ) -> Option<CombinationResult> {
        let mut best_combination: Option<CombinationResult> = None;
        let mut min_deviation = f32::INFINITY;
        
        let diameters = self.get_available_diameters();
        
        // Перебираем все возможные комбинации
        for &main_diameter in &diameters {
            let main_area = self.get_area(main_diameter)?;
            
            for &secondary_diameter in &diameters {
                let secondary_area = self.get_area(secondary_diameter)?;
                
                // Вычисляем максимальное количество основной арматуры
                let max_main_count = (target_area / main_area).floor() as u32;
                
                for main_count in 0..=max_main_count {
                    let remaining_area = target_area - (main_count as f32 * main_area);
                    
                    if remaining_area <= 0.0 {
                        let total_area = main_count as f32 * main_area;
                        let deviation = (total_area - target_area).abs();
                        
                        if deviation < min_deviation {
                            min_deviation = deviation;
                            best_combination = Some(CombinationResult {
                                main_diameter,
                                main_count,
                                secondary_diameter: 0,
                                secondary_count: 0,
                                total_area,
                                deviation,
                            });
                        }
                        continue;
                    }
                    
                    // Вычисляем количество вторичной арматуры
                    let secondary_count = (remaining_area / secondary_area).round() as u32;
                    let total_area = (main_count as f32 * main_area) + (secondary_count as f32 * secondary_area);
                    let deviation = (total_area - target_area).abs();
                    
                    // Проверяем ограничения по шагам
                    let main_step_ok = main_count == 0 || main_step > 0.0;
                    let secondary_step_ok = secondary_count == 0 || secondary_step > 0.0;
                    
                    if main_step_ok && secondary_step_ok && deviation < min_deviation {
                        min_deviation = deviation;
                        best_combination = Some(CombinationResult {
                            main_diameter,
                            main_count,
                            secondary_diameter,
                            secondary_count,
                            total_area,
                            deviation,
                        });
                    }
                }
            }
        }
        
        best_combination
    }
    
    /// Генерирует Excel отчет для массива этажей и возвращает его как Vec<u8>
    pub fn generate_floors_excel_report_to_wasm(
        &self,
        floors_data: Vec<FloorData>,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut workbook = Workbook::new();
        let worksheet = workbook.add_worksheet();
        
        // Настройка форматирования
        let header_format = Format::new()
            .set_bold()
            .set_background_color(Color::RGB(0xD9D9D9))
            .set_border(FormatBorder::Thin)
            .set_align(FormatAlign::Center);
            
        let cell_format = Format::new()
            .set_border(FormatBorder::Thin)
            .set_align(FormatAlign::Center);
            
        let number_format = Format::new()
            .set_border(FormatBorder::Thin)
            .set_align(FormatAlign::Center)
            .set_num_format("0.00");
        
        // Заголовки столбцов
        let headers = [
            "Title", "Level", "Target Area", "Main Diameter", "Main Count", 
            "Secondary Diameter", "Secondary Count", "Total Area", "Deviation", "Status"
        ];
        
        for (col, header) in headers.iter().enumerate() {
            worksheet.write_string_with_format(0, col as u16, *header, &header_format)?;
        }
        
        let mut row = 1;
        
        // Обрабатываем каждый этаж
        for floor in floors_data {
            let areas = [floor.max_as1, floor.max_as2, floor.max_as3, floor.max_as4];
            let main_step = floor.steps[0];
            let secondary_step = floor.steps[1];
            
            // Для каждой области (maxAs1-maxAs4) создаем запись
            for (area_index, &target_area) in areas.iter().enumerate() {
                if target_area > 0.0 {
                    // Находим оптимальную комбинацию
                    let combination = self.find_optimal_combination_for_area(
                        target_area as f32,
                        main_step as f32,
                        secondary_step as f32,
                    );
                    
                    // Title (может быть null)
                    if let Some(ref title) = floor.title {
                        worksheet.write_string_with_format(row, 0, title, &cell_format)?;
                    } else {
                        worksheet.write_string_with_format(row, 0, "", &cell_format)?;
                    }
                    
                    // Level
                    worksheet.write_string_with_format(row, 1, &floor.level, &cell_format)?;
                    
                    // Target Area
                    worksheet.write_number_with_format(row, 2, target_area, &number_format)?;
                    
                    if let Some(combo) = combination {
                        // Main Diameter
                        worksheet.write_number_with_format(row, 3, combo.main_diameter as f64, &cell_format)?;
                        
                        // Main Count
                        worksheet.write_number_with_format(row, 4, combo.main_count as f64, &cell_format)?;
                        
                        // Secondary Diameter
                        if combo.secondary_count > 0 {
                            worksheet.write_number_with_format(row, 5, combo.secondary_diameter as f64, &cell_format)?;
                        } else {
                            worksheet.write_string_with_format(row, 5, "-", &cell_format)?;
                        }
                        
                        // Secondary Count
                        if combo.secondary_count > 0 {
                            worksheet.write_number_with_format(row, 6, combo.secondary_count as f64, &cell_format)?;
                        } else {
                            worksheet.write_string_with_format(row, 6, "-", &cell_format)?;
                        }
                        
                        // Total Area
                        worksheet.write_number_with_format(row, 7, combo.total_area, &number_format)?;
                        
                        // Deviation
                        worksheet.write_number_with_format(row, 8, combo.deviation, &number_format)?;
                        
                        // Status
                        worksheet.write_string_with_format(row, 9, "Found", &cell_format)?;
                    } else {
                        // Если комбинация не найдена
                        for col in 3..9 {
                            worksheet.write_string_with_format(row, col, "-", &cell_format)?;
                        }
                        worksheet.write_string_with_format(row, 9, "Not Found", &cell_format)?;
                    }
                    
                    row += 1;
                }
            }
        }
        
        // Автоподбор ширины столбцов
		worksheet.autofit();  // Изменить на autofit
        
        // Сохраняем в буфер
        // let mut buffer = Vec::new();
       let buffer = workbook.save_to_buffer()?;
		Ok(buffer) 
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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_custom_sortament_creation() {
        let diameters = vec![8, 10, 12, 14, 16, 18, 20, 22, 25, 28, 32];
        let sortament = CustomSortament::from_js_data(diameters.clone());
        
        assert_eq!(sortament.get_available_diameters(), diameters);
        assert!(sortament.get_area(12).is_some());
        assert!(sortament.get_area(100).is_none());
    }
    
    #[test]
    fn test_area_calculation() {
        let area_12 = CustomSortament::calculate_area_for_diameter(12);
        let expected = std::f64::consts::PI * 6.0 * 6.0; // π * r²
        assert!((area_12 - expected).abs() < 0.001);
    }
    
    #[test]
    fn test_combination_finding() {
        let diameters = vec![8, 10, 12, 14, 16];
        let sortament = CustomSortament::from_js_data(diameters);
        
        let result = sortament.find_optimal_combination_for_area(500.0, 200.0, 150.0);
        assert!(result.is_some());
        
        let combo = result.unwrap();
        assert!(combo.total_area > 0.0);
        assert!(combo.deviation >= 0.0);
    }
}