use image::Rgb;
/// Генерирует цветовую палитру на основе количества комбинаций в итоговой шкале
pub fn generate_color_palette(scale_count: usize) -> Vec<Rgb<u8>> {
    match scale_count {
        0 => vec![Rgb([128, 128, 128])], // Серый для пустых данных
        1 => vec![Rgb([255, 255, 0])], // Светло-желтый
        2 => vec![
            Rgb([255, 255, 230]), // Белый с желтым оттенком
            Rgb([175, 238, 238]), // Светлый aqua
        ],
        3 => vec![
            Rgb([141, 251, 246]), // #8DFBF6 - первый цвет
            Rgb([220, 254, 225]), // #DCFEE1 - второй цвет
            Rgb([252, 255, 107]), // #FCFF6B - третий цвет
        ],
        4 => vec![
            Rgb([141, 251, 246]), // #8DFBF6 - первый цвет
            Rgb([220, 254, 225]), // #DCFEE1 - второй цвет
            Rgb([252, 255, 107]), // #FCFF6B - третий цвет
            Rgb([248, 221, 46]),  // #F8DD2E - четвертый цвет
        ],
        5 => vec![
            Rgb([141, 251, 246]), // #8DFBF6 - первый цвет
            Rgb([220, 254, 225]), // #DCFEE1 - второй цвет
            Rgb([252, 255, 107]), // #FCFF6B - третий цвет
            Rgb([248, 221, 46]),  // #F8DD2E - четвертый цвет
            Rgb([252, 166, 26]),  // #FCA61A - пятый цвет
        ],
        6 => vec![
            Rgb([141, 251, 246]), // #8DFBF6 - первый цвет
            Rgb([220, 254, 225]), // #DCFEE1 - второй цвет
            Rgb([252, 255, 107]), // #FCFF6B - третий цвет
            Rgb([248, 221, 46]),  // #F8DD2E - четвертый цвет
            Rgb([252, 166, 26]),  // #FCA61A - пятый цвет
            Rgb([247, 101, 0]),   // #F76500 - шестой цвет
        ],
        7 => vec![
            Rgb([141, 251, 246]), // #8DFBF6 - первый цвет
            Rgb([220, 254, 225]), // #DCFEE1 - второй цвет
            Rgb([252, 255, 107]), // #FCFF6B - третий цвет
            Rgb([248, 221, 46]),  // #F8DD2E - четвертый цвет
            Rgb([252, 166, 26]),  // #FCA61A - пятый цвет
            Rgb([247, 101, 0]),   // #F76500 - шестой цвет
            Rgb([197, 44, 0]),    // #C52C00 - седьмой цвет
        ],
        8 => vec![
            Rgb([141, 251, 246]), // #8DFBF6 - первый цвет
            Rgb([220, 254, 225]), // #DCFEE1 - второй цвет
            Rgb([252, 255, 107]), // #FCFF6B - третий цвет
            Rgb([248, 221, 46]),  // #F8DD2E - четвертый цвет
            Rgb([252, 166, 26]),  // #FCA61A - пятый цвет
            Rgb([247, 101, 0]),   // #F76500 - шестой цвет
            Rgb([197, 44, 0]),    // #C52C00 - седьмой цвет
            Rgb([121, 4, 2]),     // #790402 - восьмой цвет
        ],
        9 => vec![
            Rgb([141, 251, 246]), // #8DFBF6 - первый цвет
            Rgb([220, 254, 225]), // #DCFEE1 - второй цвет
            Rgb([252, 255, 107]), // #FCFF6B - третий цвет
            Rgb([248, 221, 46]),  // #F8DD2E - четвертый цвет
            Rgb([252, 166, 26]),  // #FCA61A - пятый цвет
            Rgb([247, 101, 0]),   // #F76500 - шестой цвет
            Rgb([197, 44, 0]),    // #C52C00 - седьмой цвет
            Rgb([121, 4, 2]),     // #790402 - восьмой цвет (темно-бордовый)
            Rgb([209, 195, 247]), // #d1c3f7 - девятый цвет (светло-фиолетовый)
        ],
        10 => vec![
            Rgb([141, 251, 246]), // #8DFBF6 - первый цвет
            Rgb([220, 254, 225]), // #DCFEE1 - второй цвет
            Rgb([252, 255, 107]), // #FCFF6B - третий цвет
            Rgb([248, 221, 46]),  // #F8DD2E - четвертый цвет
            Rgb([252, 166, 26]),  // #FCA61A - пятый цвет
            Rgb([247, 101, 0]),   // #F76500 - шестой цвет
            Rgb([197, 44, 0]),    // #C52C00 - седьмой цвет
            Rgb([121, 4, 2]),     // #790402 - восьмой цвет (темно-бордовый)
            Rgb([209, 195, 247]), // #d1c3f7 - девятый цвет (светло-фиолетовый)
            Rgb([141, 117, 206]), // #8d75ce - десятый цвет
        ],
        11 => vec![
            Rgb([141, 251, 246]), // #8DFBF6 - первый цвет
            Rgb([220, 254, 225]), // #DCFEE1 - второй цвет
            Rgb([252, 255, 107]), // #FCFF6B - третий цвет
            Rgb([248, 221, 46]),  // #F8DD2E - четвертый цвет
            Rgb([252, 166, 26]),  // #FCA61A - пятый цвет
            Rgb([247, 101, 0]),   // #F76500 - шестой цвет
            Rgb([197, 44, 0]),    // #C52C00 - седьмой цвет
            Rgb([121, 4, 2]),     // #790402 - восьмой цвет (темно-бордовый)
            Rgb([209, 195, 247]), // #d1c3f7 - девятый цвет (светло-фиолетовый)
            Rgb([141, 117, 206]), // #8d75ce - десятый цвет
            Rgb([97, 67, 178]),   // #6143b2 - одиннадцатый цвет
        ],
        12 => vec![
            Rgb([141, 251, 246]), // #8DFBF6 - первый цвет
            Rgb([220, 254, 225]), // #DCFEE1 - второй цвет
            Rgb([252, 255, 107]), // #FCFF6B - третий цвет
            Rgb([248, 221, 46]),  // #F8DD2E - четвертый цвет
            Rgb([252, 166, 26]),  // #FCA61A - пятый цвет
            Rgb([247, 101, 0]),   // #F76500 - шестой цвет
            Rgb([197, 44, 0]),    // #C52C00 - седьмой цвет
            Rgb([121, 4, 2]),     // #790402 - восьмой цвет (темно-бордовый)
            Rgb([209, 195, 247]), // #d1c3f7 - девятый цвет
            Rgb([141, 117, 206]), // #8d75ce - десятый цвет
            Rgb([97, 67, 178]),   // #6143b2 - одиннадцатый цвет
            Rgb([54, 24, 137]),   // #361889 - двенадцатый цвет
        ],
        _ => {
            if scale_count <= 20 {
                // Для 13-20 цветов используем расширенную палитру
                generate_extended_palette(scale_count)
            } else {
                // Для очень больших количеств используем улучшенный градиент
                generate_improved_gradient_palette(scale_count)
            }
        }
    }
}

/// Генерирует расширенную палитру для 13-20 цветов
fn generate_extended_palette(count: usize) -> Vec<Rgb<u8>> {
    // Базовая унифицированная палитра с правильными цветами
    let base_palette = vec![
        Rgb([141, 251, 246]), // #8DFBF6 - первый цвет (светло-aqua)
        Rgb([220, 254, 225]), // #DCFEE1 - второй цвет
        Rgb([252, 255, 107]), // #FCFF6B - третий цвет
        Rgb([248, 221, 46]),  // #F8DD2E - четвертый цвет
        Rgb([252, 166, 26]),  // #FCA61A - пятый цвет
        Rgb([247, 101, 0]),   // #F76500 - шестой цвет
        Rgb([197, 44, 0]),    // #C52C00 - седьмой цвет
        Rgb([121, 4, 2]),     // #790402 - восьмой цвет (темно-бордовый)
        Rgb([209, 195, 247]), // #d1c3f7 - девятый цвет (светло-фиолетовый)
        Rgb([141, 117, 206]), // #8d75ce - десятый цвет
        Rgb([97, 67, 178]),   // #6143b2 - одиннадцатый цвет
        Rgb([54, 24, 137]),   // #361889 - двенадцатый цвет
    ];
    
    if count <= 12 {
        base_palette.into_iter().take(count).collect()
    } else {
        // Для 13-20 цветов добавляем дополнительные оттенки
        let mut extended = base_palette;
        
        // Добавляем дополнительные цвета после основной палитры
        let additional_colors = vec![
            Rgb([40, 15, 100]),   // Еще более темно-фиолетовый
            Rgb([30, 10, 80]),    // Очень темно-фиолетовый
            Rgb([20, 5, 60]),     // Почти черно-фиолетовый
            Rgb([15, 3, 40]),     // Темно-синий
            Rgb([10, 2, 30]),     // Очень темно-синий
            Rgb([5, 1, 20]),      // Почти черный
            Rgb([3, 0, 15]),      // Черно-синий
            Rgb([1, 0, 10]),      // Почти черный
        ];
        
        for color in additional_colors {
            if extended.len() >= count {
                break;
            }
            extended.push(color);
        }
        
        extended.into_iter().take(count).collect()
    }
}

/// Генерирует улучшенную градиентную палитру для очень больших количеств
fn generate_improved_gradient_palette(count: usize) -> Vec<Rgb<u8>> {
    let mut palette = Vec::with_capacity(count);
    
    for i in 0..count {
        let ratio = i as f32 / (count - 1).max(1) as f32;
        
        // Улучшенный градиент с более разнообразными цветами
        let (r, g, b) = if ratio < 0.2 {
            // Кремовые и светло-желтые тона
            let local_ratio = ratio / 0.2;
            let r = (255.0 - local_ratio * 10.0) as u8;
            let g = 255;
            let b = (204.0 - local_ratio * 102.0) as u8;
            (r, g, b)
        } else if ratio < 0.4 {
            // Желтые тона
            let local_ratio = (ratio - 0.2) / 0.2;
            let r = 255;
            let g = (255.0 - local_ratio * 40.0) as u8;
            let b = 0;
            (r, g, b)
        } else if ratio < 0.6 {
            // Оранжевые тона
            let local_ratio = (ratio - 0.4) / 0.2;
            let r = 255;
            let g = (215.0 - local_ratio * 75.0) as u8;
            let b = 0;
            (r, g, b)
        } else if ratio < 0.8 {
            // Красно-оранжевые тона
            let local_ratio = (ratio - 0.6) / 0.2;
            let r = 255;
            let g = (140.0 - local_ratio * 140.0) as u8;
            let b = 0;
            (r, g, b)
        } else {
            // Красные и бордовые тона
            let local_ratio = (ratio - 0.8) / 0.2;
            let r = (255.0 - local_ratio * 127.0) as u8;
            let g = 0;
            let b = 0;
            (r, g, b)
        };
        
        palette.push(Rgb([r, g, b]));
    }
    
    palette
}

/// Старая градиентная функция (оставлена для совместимости)
fn generate_gradient_palette(count: usize) -> Vec<Rgb<u8>> {
    generate_improved_gradient_palette(count)
}

/// Парсит итоговую шкалу и извлекает диапазоны площадей
pub fn parse_result_scale(result_scale: &str) -> Vec<(f32, f32)> {
    let mut areas = Vec::new();
    
    // Ищем все вхождения вида [X.XXXсм2:...]
    let re = regex::Regex::new(r"\[(\d+\.\d+)см2:[^\]]+\]").unwrap();
    
    for cap in re.captures_iter(result_scale) {
        if let Some(area_str) = cap.get(1) {
            if let Ok(area) = area_str.as_str().parse::<f32>() {
                areas.push(area);
            }
        }
    }
    
    // Сортируем площади
    areas.sort_by(|a, b| a.partial_cmp(b).unwrap());
    
    // ИСПРАВЛЕНИЕ: Создаем равномерные диапазоны на основе значений из result_scale
    let mut ranges = Vec::new();
    if !areas.is_empty() {
        let min_area = 0.0;
        let max_area = areas.last().copied().unwrap_or(0.0) * 1.2; // Добавляем 20% запас
        let step = max_area / areas.len() as f32;
        
        // Создаем равномерные диапазоны
        for i in 0..areas.len() {
            let range_min = min_area + (i as f32 * step);
            let range_max = min_area + ((i + 1) as f32 * step);
            ranges.push((range_min, range_max));
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