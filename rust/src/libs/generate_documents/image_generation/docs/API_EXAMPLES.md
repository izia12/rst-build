# API документация и примеры использования

## Обзор API

Адаптивная система генерации изображений предоставляет простой и мощный API для автоматического создания оптимизированных DOCX документов с изображениями, адаптированными под формат A4.

## Основные функции API

### 1. Создание адаптивного DOCX документа

```rust
// Основная функция для создания DOCX с адаптивными изображениями
pub fn create_docx_document_adaptive(
    hash_grouped_by_z: &HashMap<String, DrawItemZ>
) -> Result<Vec<u8>, Box<dyn std::error::Error>>
```

**Описание**: Создает DOCX документ с автоматически адаптированными размерами изображений и страниц.

**Параметры**:
- `hash_grouped_by_z`: HashMap с данными, сгруппированными по Z-координате (этажам)

**Возвращает**: 
- `Ok(Vec<u8>)`: Бинарные данные DOCX документа
- `Err(...)`: Ошибка при создании документа

### 2. Создание DOCX для выбранных этажей

```rust
// Функция для создания DOCX только для определенных этажей
pub fn create_docx_for_selected_floors_adaptive(
    hash_grouped_by_z: &HashMap<String, DrawItemZ>,
    selected_floors: &[String]
) -> Result<Vec<u8>, Box<dyn std::error::Error>>
```

**Описание**: Создает DOCX документ только для указанных этажей.

**Параметры**:
- `hash_grouped_by_z`: HashMap с данными по этажам
- `selected_floors`: Список этажей для включения в документ

### 3. Анализ границ координат

```rust
// Анализ координатных границ для набора entities
pub fn analyze_coordinate_bounds(
    entities: &[EntityWithXlsx]
) -> Result<CoordinateBounds, BoundsError>
```

**Описание**: Анализирует координаты всех фигур и определяет границы.

**Возвращает**:
```rust
pub struct CoordinateBounds {
    pub min_x: f64,
    pub max_x: f64,
    pub min_y: f64,
    pub max_y: f64,
    pub width: f64,
    pub height: f64,
    pub center_x: f64,
    pub center_y: f64,
    pub aspect_ratio: f64,
}
```

### 4. Расчет оптимальных размеров

```rust
// Расчет оптимальных размеров для A4 формата
pub fn calculate_optimal_dimensions(
    bounds: &CoordinateBounds,
    config: &A4Config
) -> Result<OptimalDimensions, DimensionError>
```

**Возвращает**:
```rust
pub struct OptimalDimensions {
    pub image_width: u32,        // Ширина изображения в пикселях
    pub image_height: u32,       // Высота изображения в пикселях
    pub page_width_twips: u32,   // Ширина страницы в twips
    pub page_height_twips: u32,  // Высота страницы в twips
    pub docx_width_twips: u32,   // Ширина изображения в DOCX (twips)
    pub docx_height_twips: u32,  // Высота изображения в DOCX (twips)
    pub orientation: PageOrientation,
    pub target_dpi: f64,
    pub font_size: f32,
}
```

## Примеры использования

### Пример 1: Базовое использование

```rust
use crate::libs::image_generation::*;
use std::collections::HashMap;

// Подготовка данных
let mut hash_grouped_by_z: HashMap<String, DrawItemZ> = HashMap::new();

// Добавление данных для этажа "Floor_1"
let mut floor_1 = DrawItemZ::new();
floor_1.add_entity(entity_1);
floor_1.add_entity(entity_2);
hash_grouped_by_z.insert("Floor_1".to_string(), floor_1);

// Создание адаптивного DOCX документа
match create_docx_document_adaptive(&hash_grouped_by_z) {
    Ok(docx_data) => {
        // Сохранение файла
        std::fs::write("output_adaptive.docx", docx_data)?;
        println!("DOCX документ успешно создан!");
    },
    Err(e) => {
        eprintln!("Ошибка создания DOCX: {}", e);
    }
}
```

### Пример 2: Создание документа для выбранных этажей

```rust
// Выбор конкретных этажей
let selected_floors = vec![
    "Floor_1".to_string(),
    "Floor_3".to_string(),
    "Floor_5".to_string()
];

// Создание DOCX только для выбранных этажей
match create_docx_for_selected_floors_adaptive(&hash_grouped_by_z, &selected_floors) {
    Ok(docx_data) => {
        std::fs::write("selected_floors.docx", docx_data)?;
        println!("DOCX для выбранных этажей создан!");
    },
    Err(e) => {
        eprintln!("Ошибка: {}", e);
    }
}
```

### Пример 3: Настройка конфигурации

```rust
use crate::libs::image_generation::config::*;

// Создание пользовательской конфигурации
let custom_config = A4Config {
    page_width_twips: 11906,  // Portrait A4
    page_height_twips: 16838,
    margins: Margins {
        top: 720,    // 0.5 дюйма
        bottom: 720,
        left: 720,
        right: 720,
    },
    min_dpi: 150.0,
    max_dpi: 600.0,
    default_dpi: 300.0,
    min_font_size: 8.0,
    max_font_size: 24.0,
    default_font_size: 12.0,
    quality_level: QualityLevel::High,
};

// Использование пользовательской конфигурации
let bounds = analyze_coordinate_bounds(&entities)?;
let dimensions = calculate_optimal_dimensions(&bounds, &custom_config)?;
```

### Пример 4: Анализ координат и расчет размеров

```rust
// Анализ границ координат
let entities = vec![entity1, entity2, entity3];

match analyze_coordinate_bounds(&entities) {
    Ok(bounds) => {
        println!("Границы координат:");
        println!("  X: {} - {}", bounds.min_x, bounds.max_x);
        println!("  Y: {} - {}", bounds.min_y, bounds.max_y);
        println!("  Размеры: {}x{}", bounds.width, bounds.height);
        println!("  Соотношение сторон: {:.2}", bounds.aspect_ratio);
        
        // Расчет оптимальных размеров
        let config = A4Config::default();
        match calculate_optimal_dimensions(&bounds, &config) {
            Ok(dimensions) => {
                println!("Оптимальные размеры:");
                println!("  Изображение: {}x{} пикселей", 
                    dimensions.image_width, dimensions.image_height);
                println!("  Страница: {}x{} twips", 
                    dimensions.page_width_twips, dimensions.page_height_twips);
                println!("  Ориентация: {:?}", dimensions.orientation);
                println!("  DPI: {:.1}", dimensions.target_dpi);
                println!("  Размер шрифта: {:.1}pt", dimensions.font_size);
            },
            Err(e) => eprintln!("Ошибка расчета размеров: {}", e),
        }
    },
    Err(e) => eprintln!("Ошибка анализа границ: {}", e),
}
```

### Пример 5: Обработка ошибок и fallback

```rust
use crate::libs::image_generation::errors::*;

// Функция с обработкой ошибок и fallback к legacy методу
fn create_docx_with_fallback(
    hash_grouped_by_z: &HashMap<String, DrawItemZ>
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    
    // Попытка использовать адаптивный метод
    match create_docx_document_adaptive(hash_grouped_by_z) {
        Ok(docx_data) => {
            log::info!("Адаптивная генерация DOCX успешна");
            Ok(docx_data)
        },
        Err(e) => {
            log::warn!("Ошибка адаптивной генерации: {}. Переключение на legacy метод.", e);
            
            // Fallback к legacy методу
            match create_docx_document_legacy(hash_grouped_by_z) {
                Ok(docx_data) => {
                    log::info!("Legacy генерация DOCX успешна");
                    Ok(docx_data)
                },
                Err(legacy_error) => {
                    log::error!("Ошибка legacy генерации: {}", legacy_error);
                    Err(Box::new(legacy_error))
                }
            }
        }
    }
}
```

### Пример 6: Мониторинг производительности

```rust
use std::time::Instant;
use crate::libs::image_generation::metrics::*;

// Функция с мониторингом производительности
fn create_docx_with_metrics(
    hash_grouped_by_z: &HashMap<String, DrawItemZ>
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    
    let mut metrics = PerformanceMetrics::new();
    let start_time = Instant::now();
    
    // Этап 1: Анализ данных
    let analysis_start = Instant::now();
    let total_entities = hash_grouped_by_z.values()
        .map(|draw_item| draw_item.data.len())
        .sum::<usize>();
    metrics.analysis_time_ms = analysis_start.elapsed().as_millis() as u64;
    
    // Этап 2: Создание DOCX
    let docx_start = Instant::now();
    let result = create_docx_document_adaptive(hash_grouped_by_z);
    metrics.docx_generation_time_ms = docx_start.elapsed().as_millis() as u64;
    
    // Общее время
    metrics.total_time_ms = start_time.elapsed().as_millis() as u64;
    metrics.entities_processed = total_entities;
    
    // Логирование метрик
    log::info!("Метрики производительности: {:?}", metrics);
    
    result
}
```

## Конфигурация

### Структура A4Config

```rust
#[derive(Debug, Clone)]
pub struct A4Config {
    // Размеры страницы A4 в twips
    pub page_width_twips: u32,   // 11906 (portrait) или 16838 (landscape)
    pub page_height_twips: u32,  // 16838 (portrait) или 11906 (landscape)
    
    // Отступы страницы
    pub margins: Margins,
    
    // Настройки качества изображения
    pub min_dpi: f64,           // Минимальное разрешение (например, 150)
    pub max_dpi: f64,           // Максимальное разрешение (например, 600)
    pub default_dpi: f64,       // Разрешение по умолчанию (например, 300)
    
    // Настройки шрифта
    pub min_font_size: f32,     // Минимальный размер шрифта
    pub max_font_size: f32,     // Максимальный размер шрифта
    pub default_font_size: f32, // Размер шрифта по умолчанию
    
    // Уровень качества
    pub quality_level: QualityLevel,
}

#[derive(Debug, Clone)]
pub struct Margins {
    pub top: u32,     // Верхний отступ в twips
    pub bottom: u32,  // Нижний отступ в twips
    pub left: u32,    // Левый отступ в twips
    pub right: u32,   // Правый отступ в twips
}

#[derive(Debug, Clone)]
pub enum QualityLevel {
    Draft,    // Быстрая генерация, низкое качество
    Standard, // Баланс качества и скорости
    High,     // Высокое качество, медленная генерация
}
```

### Значения по умолчанию

```rust
impl Default for A4Config {
    fn default() -> Self {
        Self {
            page_width_twips: 11906,  // A4 Portrait
            page_height_twips: 16838,
            margins: Margins {
                top: 1440,    // 1 дюйм
                bottom: 1440,
                left: 1440,
                right: 1440,
            },
            min_dpi: 150.0,
            max_dpi: 600.0,
            default_dpi: 300.0,
            min_font_size: 8.0,
            max_font_size: 24.0,
            default_font_size: 12.0,
            quality_level: QualityLevel::Standard,
        }
    }
}
```

## Обработка ошибок

### Типы ошибок

```rust
#[derive(Debug, thiserror::Error)]
pub enum AdaptiveImageError {
    #[error("Ошибка анализа границ: {0}")]
    BoundsAnalysis(#[from] BoundsError),
    
    #[error("Ошибка расчета размеров: {0}")]
    DimensionCalculation(#[from] DimensionError),
    
    #[error("Ошибка масштабирования: {0}")]
    Scaling(#[from] ScalingError),
    
    #[error("Ошибка отрисовки: {0}")]
    Rendering(#[from] RenderingError),
    
    #[error("Ошибка создания DOCX: {0}")]
    DocxGeneration(String),
    
    #[error("Конфигурация невалидна: {0}")]
    InvalidConfiguration(String),
}

#[derive(Debug, thiserror::Error)]
pub enum BoundsError {
    #[error("Нет данных для анализа")]
    NoData,
    
    #[error("Невалидные координаты")]
    InvalidCoordinates,
    
    #[error("Все координаты одинаковые")]
    ZeroDimensions,
}
```

### Обработка ошибок в коде

```rust
// Пример обработки различных типов ошибок
match create_docx_document_adaptive(&hash_grouped_by_z) {
    Ok(docx_data) => {
        // Успешное создание
        Ok(docx_data)
    },
    Err(e) => {
        match e.downcast_ref::<AdaptiveImageError>() {
            Some(AdaptiveImageError::BoundsAnalysis(bounds_err)) => {
                log::warn!("Проблема с анализом границ: {}. Используем значения по умолчанию.", bounds_err);
                // Fallback логика
            },
            Some(AdaptiveImageError::DimensionCalculation(dim_err)) => {
                log::warn!("Проблема с расчетом размеров: {}. Используем стандартные размеры.", dim_err);
                // Fallback логика
            },
            Some(AdaptiveImageError::Rendering(render_err)) => {
                log::error!("Критическая ошибка отрисовки: {}. Переключение на legacy метод.", render_err);
                // Fallback к legacy методу
            },
            _ => {
                log::error!("Неизвестная ошибка: {}", e);
                return Err(e);
            }
        }
    }
}
```

## Логирование и мониторинг

### Настройка логирования

```rust
use log::{info, warn, error, debug};

// Пример логирования в процессе создания DOCX
fn create_docx_with_logging(
    hash_grouped_by_z: &HashMap<String, DrawItemZ>
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    
    info!("Начало создания адаптивного DOCX документа");
    debug!("Количество этажей: {}", hash_grouped_by_z.len());
    
    let start_time = std::time::Instant::now();
    
    for (floor_name, draw_item) in hash_grouped_by_z {
        debug!("Обработка этажа '{}' с {} entities", floor_name, draw_item.data.len());
        
        // Анализ границ
        match draw_item.analyze_bounds() {
            Ok(bounds) => {
                debug!("Границы для этажа '{}': {:?}", floor_name, bounds);
            },
            Err(e) => {
                warn!("Не удалось проанализировать границы для этажа '{}': {}", floor_name, e);
            }
        }
    }
    
    let result = create_docx_document_adaptive(hash_grouped_by_z);
    
    match &result {
        Ok(docx_data) => {
            let elapsed = start_time.elapsed();
            info!("DOCX документ успешно создан за {:.2}с, размер: {} байт", 
                elapsed.as_secs_f64(), docx_data.len());
        },
        Err(e) => {
            error!("Ошибка создания DOCX документа: {}", e);
        }
    }
    
    result
}
```

## Тестирование

### Пример юнит-теста

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::libs::parse::{EntityWithXlsx, Vertex};
    
    #[test]
    fn test_bounds_analysis() {
        // Создание тестовых данных
        let entities = vec![
            EntityWithXlsx {
                vertices: vec![
                    Vertex { x: 0.0, y: 0.0, z: 0.0 },
                    Vertex { x: 10.0, y: 0.0, z: 0.0 },
                    Vertex { x: 10.0, y: 10.0, z: 0.0 },
                    Vertex { x: 0.0, y: 10.0, z: 0.0 },
                ],
                // ... другие поля
            },
        ];
        
        // Тестирование анализа границ
        let bounds = analyze_coordinate_bounds(&entities).unwrap();
        
        assert_eq!(bounds.min_x, 0.0);
        assert_eq!(bounds.max_x, 10.0);
        assert_eq!(bounds.min_y, 0.0);
        assert_eq!(bounds.max_y, 10.0);
        assert_eq!(bounds.width, 10.0);
        assert_eq!(bounds.height, 10.0);
        assert_eq!(bounds.aspect_ratio, 1.0);
    }
    
    #[test]
    fn test_dimension_calculation() {
        let bounds = CoordinateBounds {
            min_x: 0.0,
            max_x: 100.0,
            min_y: 0.0,
            max_y: 50.0,
            width: 100.0,
            height: 50.0,
            center_x: 50.0,
            center_y: 25.0,
            aspect_ratio: 2.0,
        };
        
        let config = A4Config::default();
        let dimensions = calculate_optimal_dimensions(&bounds, &config).unwrap();
        
        // Проверка, что выбрана landscape ориентация для широких данных
        assert_eq!(dimensions.orientation, PageOrientation::Landscape);
        assert!(dimensions.image_width > dimensions.image_height);
    }
}
```

## Заключение

Данная API документация предоставляет:

✅ **Полное описание всех публичных функций** и их параметров  
✅ **Практические примеры использования** для различных сценариев  
✅ **Детальную конфигурацию** системы  
✅ **Стратегии обработки ошибок** и fallback механизмы  
✅ **Примеры логирования и мониторинга** производительности  
✅ **Шаблоны для тестирования** функциональности  

Эта документация поможет разработчикам быстро интегрировать адаптивную систему генерации изображений в свои проекты и эффективно использовать все её возможности.