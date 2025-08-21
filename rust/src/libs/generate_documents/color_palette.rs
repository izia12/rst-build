use image::Rgb;
use std::collections::HashMap;

/// Генерирует цветовую палитру на основе количества комбинаций в итоговой шкале
pub fn generate_color_palette(scale_count: usize) -> Vec<Rgb<u8>> {
    match scale_count {
        0 => vec![Rgb([128, 128, 128])], // Серый для пустых данных
        1 => vec![Rgb([255, 255, 0])], // Светло-желтый
        2 => vec![
            Rgb([255, 255, 0]),   // Желтый
            Rgb([255, 165, 0]),   // Оранжевый
        ],
        3 => vec![
            Rgb([255, 255, 0]),   // Желтый
            Rgb([255, 165, 0]),   // Оранжевый
            Rgb([255, 0, 0]),     // Красный
        ],
        4 => vec![
            Rgb([255, 255, 0]),   // Желтый
            Rgb([255, 165, 0]),   // Оранжевый
            Rgb([255, 0, 0]),     // Красный
            Rgb([139, 0, 0]),     // Темно-красный
        ],
        5 => vec![
            Rgb([255, 255, 0]),   // Желтый
            Rgb([255, 165, 0]),   // Оранжевый
            Rgb([255, 0, 0]),     // Красный
            Rgb([139, 0, 0]),     // Темно-красный
            Rgb([128, 0, 0]),     // Бордовый
        ],
        _ => generate_gradient_palette(scale_count), // Для больших количеств
    }
}

/// Генерирует градиентную палитру для большого количества комбинаций
fn generate_gradient_palette(count: usize) -> Vec<Rgb<u8>> {
    let mut palette = Vec::with_capacity(count);
    
    for i in 0..count {
        let ratio = i as f32 / (count - 1) as f32;
        
        // Градиент от желтого (255,255,0) до бордового (128,0,0)
        let r = (255.0 * (1.0 - ratio * 0.5)) as u8;
        let g = (255.0 * (1.0 - ratio)) as u8;
        let b = 0;
        
        palette.push(Rgb([r, g, b]));
    }
    
    palette
}

/// Парсит итоговую шкалу и извлекает диапазоны площадей
pub fn parse_result_scale(result_scale: &str) -> Vec<(f32, f32)> {
    let mut ranges = Vec::new();
    
    // Ищем все вхождения вида [X.XXXсм2:...]
    let re = regex::Regex::new(r"\[(\d+\.\d+)см2:[^\]]+\]").unwrap();
    
    for cap in re.captures_iter(result_scale) {
        if let Some(area_str) = cap.get(1) {
            if let Ok(area) = area_str.as_str().parse::<f32>() {
                // Создаем диапазон ±5% от значения
                let tolerance = area * 0.05;
                ranges.push((area - tolerance, area + tolerance));
            }
        }
    }
    
    ranges
}

/// Определяет цвет для фигуры на основе ее площади и итоговой шкалы
pub fn get_color_for_area(
    area_value: f32,
    result_scale: &str,
    palette: &[Rgb<u8>]
) -> Rgb<u8> {
    let ranges = parse_result_scale(result_scale);
    
    // Ищем подходящий диапазон
    for (i, &(min_area, max_area)) in ranges.iter().enumerate() {
        if area_value >= min_area && area_value <= max_area {
            return palette.get(i).copied().unwrap_or(palette[0]);
        }
    }
    
    // Если не найден подходящий диапазон, используем первый цвет
    palette.get(0).copied().unwrap_or(Rgb([128, 128, 128]))
}

/// Создает легенду для палитры цветов
pub fn create_color_legend(result_scale: &str, palette: &[Rgb<u8>]) -> String {
    let ranges = parse_result_scale(result_scale);
    let mut legend = String::from("Цветовая палитра:\n");
    
    // Извлекаем описания из итоговой шкалы
    let re = regex::Regex::new(r"\[([^\]]+)\]").unwrap();
    let descriptions: Vec<&str> = re.find_iter(result_scale)
        .map(|m| m.as_str())
        .collect();
    
    for (i, description) in descriptions.iter().enumerate() {
        if let Some(color) = palette.get(i) {
            legend.push_str(&format!(
                "🟨 {} - RGB({}, {}, {})\n",
                description,
                color.0[0],
                color.0[1],
                color.0[2]
            ));
        }
    }
    
    legend
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_generate_color_palette() {
        let palette_1 = generate_color_palette(1);
        assert_eq!(palette_1.len(), 1);
        assert_eq!(palette_1[0], Rgb([255, 255, 0])); // Желтый
        
        let palette_5 = generate_color_palette(5);
        assert_eq!(palette_5.len(), 5);
        assert_eq!(palette_5[0], Rgb([255, 255, 0])); // Желтый
        assert_eq!(palette_5[4], Rgb([128, 0, 0])); // Бордовый
    }
    
    #[test]
    fn test_parse_result_scale() {
        let scale = "[2.515см2:Ø8 мм s=200 мм][3.930см2:Ø8 мм s=200 мм + Ø6 мм s=200 мм]";
        let ranges = parse_result_scale(scale);
        
        assert_eq!(ranges.len(), 2);
        assert!((ranges[0].0 - 2.389).abs() < 0.01); // 2.515 - 5%
        assert!((ranges[0].1 - 2.641).abs() < 0.01); // 2.515 + 5%
    }
    
    #[test]
    fn test_get_color_for_area() {
        let scale = "[2.515см2:Ø8 мм s=200 мм][3.930см2:Ø8 мм s=200 мм + Ø6 мм s=200 мм]";
        let palette = generate_color_palette(2);
        
        let color1 = get_color_for_area(2.5, scale, &palette);
        assert_eq!(color1, palette[0]); // Первый цвет
        
        let color2 = get_color_for_area(3.9, scale, &palette);
        assert_eq!(color2, palette[1]); // Второй цвет
    }
}