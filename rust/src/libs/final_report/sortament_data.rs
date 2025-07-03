use std::collections::HashMap;
use map_macro::hash_map;
use lazy_static::lazy_static;

/// Стандартный сортамент арматуры согласно ГОСТ
/// Диаметр (мм) -> Площадь поперечного сечения (см²)
pub fn get_standard_sortament() -> HashMap<u32, f32> {
    hash_map! {
        3 => 0.071,
        4 => 0.126,
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
    }
}

//Ленивая инициализация для глобального доступа
lazy_static! {
    pub static ref STANDARD_SORTAMENT: HashMap<u32, f32> = get_standard_sortament();
}

/// Получить площадь для конкретного диаметра
pub fn get_area_for_diameter(diameter: u32) -> Option<f32> {
    STANDARD_SORTAMENT.get(&diameter).copied()
}

/// Получить все доступные диаметры
pub fn get_all_diameters() -> Vec<u32> {
    let mut diameters: Vec<u32> = STANDARD_SORTAMENT.keys().copied().collect();
    diameters.sort();
    diameters
}

/// Проверить, существует ли диаметр в сортаменте
pub fn is_diameter_valid(diameter: u32) -> bool {
    STANDARD_SORTAMENT.contains_key(&diameter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sortament_data() {
        assert_eq!(get_area_for_diameter(12), Some(1.131));
        assert_eq!(get_area_for_diameter(16), Some(2.01));
        assert_eq!(get_area_for_diameter(999), None);
        
        assert!(is_diameter_valid(20));
        assert!(!is_diameter_valid(999));
        
        let diameters = get_all_diameters();
        assert!(diameters.len() > 0);
        assert!(diameters.is_sorted());
    }
}