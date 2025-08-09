# План реализации системы адаптивной генерации изображений

## Общий план проекта

### Временные рамки
- **Общая продолжительность**: 2-3 недели
- **Этапы**: 6 основных этапов
- **Тестирование**: Параллельно с разработкой

### Приоритеты
1. **Высокий**: Базовая функциональность адаптивного масштабирования
2. **Средний**: Оптимизация качества и производительности
3. **Низкий**: Расширенные функции и конфигурация

## Этап 1: Подготовка инфраструктуры (2-3 дня)

### 1.1 Создание модульной структуры

```
rust/src/libs/image_generation/
├── mod.rs                    # Публичный интерфейс
├── bounds_analyzer.rs        # Анализ координатных границ
├── dimension_calculator.rs   # Вычисление оптимальных размеров
├── coordinate_scaler.rs      # Масштабирование координат
├── adaptive_renderer.rs      # Адаптивная отрисовка
├── a4_optimizer.rs          # Оптимизация под A4
├── config.rs                # Конфигурация и константы
├── types.rs                 # Общие типы данных
├── utils.rs                 # Вспомогательные функции
└── tests/                   # Модульные тесты
    ├── mod.rs
    ├── bounds_tests.rs
    ├── dimension_tests.rs
    └── integration_tests.rs
```

### 1.2 Определение базовых типов (types.rs)

```rust
// Основные структуры данных
pub struct CoordinateBounds {
    pub min_x: f64,
    pub max_x: f64,
    pub min_y: f64,
    pub max_y: f64,
    pub width: f64,
    pub height: f64,
    pub aspect_ratio: f64,
    pub center_x: f64,
    pub center_y: f64,
    pub area: f64,
    pub entity_count: usize,
    pub density: f64,
}

pub struct OptimalDimensions {
    pub image_width: u32,
    pub image_height: u32,
    pub docx_width_twips: u32,
    pub docx_height_twips: u32,
    pub page_width_twips: u32,
    pub page_height_twips: u32,
    pub scale_factor: f64,
    pub font_size: f32,
    pub dpi: u32,
    pub orientation: PageOrientation,
    pub quality_level: QualityLevel,
}

pub struct ScalingParams {
    pub scale_factor: f64,
    pub offset_x: f64,
    pub offset_y: f64,
    pub margin_left: f64,
    pub margin_top: f64,
    pub margin_right: f64,
    pub margin_bottom: f64,
    pub usable_width: f64,
    pub usable_height: f64,
    pub center_offset_x: f64,
    pub center_offset_y: f64,
}

pub struct Margins {
    pub left: f64,
    pub right: f64,
    pub top: f64,
    pub bottom: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PageOrientation {
    Portrait,
    Landscape,
    Auto,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum QualityLevel {
    Draft,    // 150 DPI, быстрая генерация
    Standard, // 300 DPI, баланс качества и размера
    High,     // 600 DPI, высокое качество
    Ultra,    // 1200 DPI, максимальное качество
}
```

### 1.3 Конфигурация (config.rs)

```rust
pub struct A4Config {
    pub orientation: PageOrientation,
    pub margins: Margins,
    pub quality: QualityLevel,
    pub min_font_size: f32,
    pub max_font_size: f32,
    pub target_dpi_range: (u32, u32),
    pub max_image_size: (u32, u32),
    pub min_image_size: (u32, u32),
    pub antialiasing: bool,
    pub compression_level: u8,
    pub outlier_threshold: f64,
    pub density_threshold: f64,
}

impl Default for A4Config {
    fn default() -> Self {
        Self {
            orientation: PageOrientation::Auto,
            margins: Margins {
                left: 400.0,
                right: 400.0,
                top: 300.0,
                bottom: 300.0,
            },
            quality: QualityLevel::Standard,
            min_font_size: 8.0,
            max_font_size: 24.0,
            target_dpi_range: (150, 600),
            max_image_size: (8000, 6000),
            min_image_size: (1000, 800),
            antialiasing: true,
            compression_level: 6,
            outlier_threshold: 3.0, // 3 стандартных отклонения
            density_threshold: 0.1,  // Минимальная плотность элементов
        }
    }
}

// Константы A4
pub const A4_WIDTH_MM: f64 = 210.0;
pub const A4_HEIGHT_MM: f64 = 297.0;
pub const A4_WIDTH_TWIPS: u32 = 11906;
pub const A4_HEIGHT_TWIPS: u32 = 16838;
pub const TWIPS_PER_INCH: f64 = 1440.0;
pub const MM_PER_INCH: f64 = 25.4;
```

### 1.4 Задачи этапа 1
- [ ] Создать структуру папок и файлов
- [ ] Определить все базовые типы данных
- [ ] Создать конфигурационную систему
- [ ] Написать базовые тесты для типов
- [ ] Обновить mod.rs в libs для подключения нового модуля

## Этап 2: Анализ координатных границ (2-3 дня)

### 2.1 Реализация BoundsAnalyzer (bounds_analyzer.rs)

```rust
use crate::libs::parse::EntityWithXlsx;
use super::types::*;

pub struct BoundsAnalyzer;

impl BoundsAnalyzer {
    /// Анализирует координатные границы всех сущностей
    pub fn analyze_bounds(entities: &[EntityWithXlsx]) -> Result<CoordinateBounds, String> {
        if entities.is_empty() {
            return Err("No entities provided".to_string());
        }

        let mut min_x = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_y = f64::NEG_INFINITY;
        let mut total_vertices = 0;

        for entity in entities {
            for vertex in &entity.vertices {
                min_x = min_x.min(vertex.x as f64);
                max_x = max_x.max(vertex.x as f64);
                min_y = min_y.min(vertex.y as f64);
                max_y = max_y.max(vertex.y as f64);
                total_vertices += 1;
            }
        }

        let width = max_x - min_x;
        let height = max_y - min_y;
        let aspect_ratio = if height != 0.0 { width / height } else { 1.0 };
        let center_x = (min_x + max_x) / 2.0;
        let center_y = (min_y + max_y) / 2.0;
        let area = width * height;
        let density = total_vertices as f64 / area.max(1.0);

        Ok(CoordinateBounds {
            min_x,
            max_x,
            min_y,
            max_y,
            width,
            height,
            aspect_ratio,
            center_x,
            center_y,
            area,
            entity_count: entities.len(),
            density,
        })
    }

    /// Обнаруживает выбросы в координатах
    pub fn detect_outliers(entities: &[EntityWithXlsx], threshold: f64) -> Vec<usize> {
        // Реализация алгоритма обнаружения выбросов
        // на основе стандартного отклонения
        Vec::new() // Заглушка
    }

    /// Фильтрует выбросы из набора данных
    pub fn filter_outliers(
        entities: &[EntityWithXlsx], 
        outlier_indices: &[usize]
    ) -> Vec<EntityWithXlsx> {
        entities
            .iter()
            .enumerate()
            .filter(|(i, _)| !outlier_indices.contains(i))
            .map(|(_, entity)| entity.clone())
            .collect()
    }

    /// Вычисляет статистики для координат
    pub fn calculate_statistics(entities: &[EntityWithXlsx]) -> CoordinateStatistics {
        // Реализация расчета среднего, медианы, стандартного отклонения
        CoordinateStatistics::default() // Заглушка
    }
}

#[derive(Debug, Default)]
pub struct CoordinateStatistics {
    pub mean_x: f64,
    pub mean_y: f64,
    pub std_dev_x: f64,
    pub std_dev_y: f64,
    pub median_x: f64,
    pub median_y: f64,
}
```

### 2.2 Задачи этапа 2
- [ ] Реализовать базовый анализ границ
- [ ] Добавить обнаружение выбросов
- [ ] Реализовать статистический анализ
- [ ] Создать тесты для всех функций
- [ ] Оптимизировать производительность для больших наборов данных

## Этап 3: Вычисление оптимальных размеров (3-4 дня)

### 3.1 Реализация DimensionCalculator (dimension_calculator.rs)

```rust
use super::types::*;
use super::config::*;

pub struct DimensionCalculator;

impl DimensionCalculator {
    /// Вычисляет оптимальные размеры изображения и страницы
    pub fn calculate_optimal_dimensions(
        bounds: &CoordinateBounds,
        config: &A4Config,
    ) -> Result<OptimalDimensions, String> {
        // 1. Определить ориентацию страницы
        let orientation = Self::determine_orientation(bounds.aspect_ratio, config.orientation);
        
        // 2. Получить размеры рабочей области A4
        let (page_width, page_height) = Self::get_a4_dimensions(orientation);
        let (usable_width, usable_height) = Self::apply_margins(
            page_width as f64, 
            page_height as f64, 
            &config.margins
        );
        
        // 3. Вычислить целевой DPI на основе сложности
        let complexity = Self::calculate_complexity(bounds);
        let target_dpi = Self::calculate_target_dpi(complexity, config.quality, &config.target_dpi_range);
        
        // 4. Вычислить размеры изображения
        let (image_width, image_height) = Self::calculate_image_dimensions(
            usable_width,
            usable_height,
            target_dpi,
            &config.max_image_size,
            &config.min_image_size,
        );
        
        // 5. Вычислить масштабный коэффициент
        let scale_factor = Self::calculate_scale_factor(bounds, image_width as f64, image_height as f64);
        
        // 6. Вычислить размер шрифта
        let font_size = Self::calculate_font_size(
            scale_factor,
            config.min_font_size,
            config.max_font_size,
        );
        
        // 7. Вычислить размеры в твипах для DOCX
        let (docx_width_twips, docx_height_twips) = Self::calculate_docx_dimensions(
            image_width,
            image_height,
            target_dpi,
        );
        
        Ok(OptimalDimensions {
            image_width,
            image_height,
            docx_width_twips,
            docx_height_twips,
            page_width_twips: page_width,
            page_height_twips: page_height,
            scale_factor,
            font_size,
            dpi: target_dpi,
            orientation,
            quality_level: config.quality,
        })
    }
    
    fn determine_orientation(aspect_ratio: f64, config_orientation: PageOrientation) -> PageOrientation {
        match config_orientation {
            PageOrientation::Auto => {
                if aspect_ratio > 1.4 {
                    PageOrientation::Landscape
                } else {
                    PageOrientation::Portrait
                }
            }
            other => other,
        }
    }
    
    fn get_a4_dimensions(orientation: PageOrientation) -> (u32, u32) {
        match orientation {
            PageOrientation::Portrait => (A4_WIDTH_TWIPS, A4_HEIGHT_TWIPS),
            PageOrientation::Landscape => (A4_HEIGHT_TWIPS, A4_WIDTH_TWIPS),
            PageOrientation::Auto => (A4_WIDTH_TWIPS, A4_HEIGHT_TWIPS), // Fallback
        }
    }
    
    fn calculate_complexity(bounds: &CoordinateBounds) -> f64 {
        // Комплексность на основе плотности элементов и размера области
        let area_factor = (bounds.area / 1000000.0).min(10.0); // Нормализация
        let density_factor = (bounds.density * 1000.0).min(10.0);
        let entity_factor = (bounds.entity_count as f64 / 1000.0).min(10.0);
        
        (area_factor + density_factor + entity_factor) / 3.0
    }
    
    fn calculate_target_dpi(
        complexity: f64,
        quality: QualityLevel,
        dpi_range: &(u32, u32),
    ) -> u32 {
        let base_dpi = match quality {
            QualityLevel::Draft => 150,
            QualityLevel::Standard => 300,
            QualityLevel::High => 600,
            QualityLevel::Ultra => 1200,
        };
        
        // Корректировка на основе сложности
        let complexity_multiplier = 1.0 + (complexity - 5.0) * 0.1;
        let adjusted_dpi = (base_dpi as f64 * complexity_multiplier) as u32;
        
        adjusted_dpi.clamp(dpi_range.0, dpi_range.1)
    }
    
    // Дополнительные вспомогательные методы...
}
```

### 3.2 Задачи этапа 3
- [ ] Реализовать алгоритм определения ориентации
- [ ] Создать систему вычисления сложности
- [ ] Реализовать адаптивный расчет DPI
- [ ] Добавить валидацию размеров
- [ ] Создать комплексные тесты

## Этап 4: Масштабирование координат (2-3 дня)

### 4.1 Реализация CoordinateScaler (coordinate_scaler.rs)

```rust
use super::types::*;
use crate::libs::parse::Vertex;

pub struct CoordinateScaler;

impl CoordinateScaler {
    /// Вычисляет параметры масштабирования
    pub fn calculate_scaling_params(
        bounds: &CoordinateBounds,
        dimensions: &OptimalDimensions,
    ) -> ScalingParams {
        // Вычисление рабочей области с учетом отступов
        let margin_left = 50.0; // Пиксели
        let margin_top = 50.0;
        let margin_right = 50.0;
        let margin_bottom = 50.0;
        
        let usable_width = dimensions.image_width as f64 - margin_left - margin_right;
        let usable_height = dimensions.image_height as f64 - margin_top - margin_bottom;
        
        // Вычисление масштабного коэффициента
        let scale_x = usable_width / bounds.width;
        let scale_y = usable_height / bounds.height;
        let scale_factor = scale_x.min(scale_y); // Сохранение пропорций
        
        // Вычисление смещений для центрирования
        let scaled_width = bounds.width * scale_factor;
        let scaled_height = bounds.height * scale_factor;
        
        let center_offset_x = (usable_width - scaled_width) / 2.0;
        let center_offset_y = (usable_height - scaled_height) / 2.0;
        
        let offset_x = margin_left + center_offset_x - bounds.min_x * scale_factor;
        let offset_y = margin_top + center_offset_y - bounds.min_y * scale_factor;
        
        ScalingParams {
            scale_factor,
            offset_x,
            offset_y,
            margin_left,
            margin_top,
            margin_right,
            margin_bottom,
            usable_width,
            usable_height,
            center_offset_x,
            center_offset_y,
        }
    }
    
    /// Масштабирует точку в координаты изображения
    pub fn scale_point(vertex: &Vertex, params: &ScalingParams) -> (f32, f32) {
        let x = (vertex.x as f64 * params.scale_factor + params.offset_x) as f32;
        let y = (vertex.y as f64 * params.scale_factor + params.offset_y) as f32;
        (x, y)
    }
    
    /// Масштабирует размер шрифта
    pub fn scale_font_size(base_size: f32, params: &ScalingParams) -> f32 {
        (base_size * params.scale_factor as f32).max(8.0).min(72.0)
    }
    
    /// Проверяет, находится ли точка в пределах изображения
    pub fn is_point_in_bounds(x: f32, y: f32, width: u32, height: u32) -> bool {
        x >= 0.0 && y >= 0.0 && x < width as f32 && y < height as f32
    }
}
```

### 4.2 Задачи этапа 4
- [ ] Реализовать точное масштабирование координат
- [ ] Добавить центрирование контента
- [ ] Реализовать адаптивные отступы
- [ ] Создать валидацию координат
- [ ] Оптимизировать для производительности

## Этап 5: Адаптивная отрисовка (4-5 дней)

### 5.1 Модификация DrawItemZ

```rust
// В drawItem.rs - расширение существующей структуры
use crate::libs::image_generation::*;

impl DrawItemZ {
    // Новые поля для адаптивности
    pub bounds: Option<CoordinateBounds>,
    pub dimensions: Option<OptimalDimensions>,
    pub scaling: Option<ScalingParams>,
    pub config: A4Config,
    
    /// Вычисляет адаптивные параметры
    pub fn calculate_adaptive_params(&mut self) -> Result<(), String> {
        // Анализ границ
        self.bounds = Some(BoundsAnalyzer::analyze_bounds(&self.data)?);
        
        // Вычисление размеров
        if let Some(bounds) = &self.bounds {
            self.dimensions = Some(DimensionCalculator::calculate_optimal_dimensions(
                bounds, 
                &self.config
            )?);
        }
        
        // Вычисление масштабирования
        if let (Some(bounds), Some(dimensions)) = (&self.bounds, &self.dimensions) {
            self.scaling = Some(CoordinateScaler::calculate_scaling_params(
                bounds, 
                dimensions
            ));
        }
        
        Ok(())
    }
    
    /// Отрисовка с адаптивными параметрами
    pub fn draw_image_adaptive(&self, field: &str) -> Result<Vec<u8>, String> {
        let dimensions = self.dimensions.as_ref()
            .ok_or("Adaptive parameters not calculated")?;
        let scaling = self.scaling.as_ref()
            .ok_or("Scaling parameters not calculated")?;
            
        // Создание буфера изображения
        let mut img = ImageBuffer::new(
            dimensions.image_width, 
            dimensions.image_height
        );
        
        // Заливка фона
        for pixel in img.pixels_mut() {
            *pixel = Rgb([255, 255, 255]); // Белый фон
        }
        
        // Отрисовка элементов
        self.draw_entities_adaptive(&mut img, field, scaling, dimensions)?;
        
        // Конвертация в PNG
        let mut buffer = Vec::new();
        {
            let mut cursor = Cursor::new(&mut buffer);
            img.write_to(&mut cursor, ImageFormat::Png)
                .map_err(|e| format!("PNG encoding error: {}", e))?;
        }
        
        Ok(buffer)
    }
    
    fn draw_entities_adaptive(
        &self,
        img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
        field: &str,
        scaling: &ScalingParams,
        dimensions: &OptimalDimensions,
    ) -> Result<(), String> {
        let font_data = include_bytes!("../../OpenSans-Regular.ttf");
        let font = Font::try_from_bytes(font_data)
            .ok_or("Failed to load font")?;
            
        for entity in &self.data {
            match entity.vertices.len() {
                3 => self.draw_triangle_adaptive(img, entity, scaling, &font, dimensions.font_size)?,
                4 => self.draw_quadrilateral_adaptive(img, entity, scaling, &font, dimensions.font_size)?,
                _ => continue,
            }
        }
        
        Ok(())
    }
    
    // Методы отрисовки треугольников и четырехугольников...
}
```

### 5.2 Задачи этапа 5
- [ ] Модифицировать DrawItemZ для адаптивности
- [ ] Реализовать адаптивную отрисовку фигур
- [ ] Добавить адаптивное масштабирование шрифтов
- [ ] Оптимизировать качество изображений
- [ ] Создать систему fallback для ошибок

## Этап 6: Интеграция с DOCX (2-3 дня)

### 6.1 Модификация docx_generator.rs

```rust
// Обновление функций генерации DOCX
pub fn create_docx_document_adaptive(
    item_z: HashMap<OrderedFloat<f32>, DrawItemZ>,
    config: Option<A4Config>,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let config = config.unwrap_or_default();
    let mut doc = Document::new();
    
    for (z_coord, mut item_z_data) in item_z {
        // Вычисление адаптивных параметров
        item_z_data.calculate_adaptive_params()
            .map_err(|e| format!("Failed to calculate adaptive params: {}", e))?;
            
        // Получение оптимальных размеров страницы
        let dimensions = item_z_data.dimensions.as_ref().unwrap();
        
        // Настройка страницы
        let page_size = PageSize::new()
            .width(dimensions.page_width_twips)
            .height(dimensions.page_height_twips);
            
        let page_orient = match dimensions.orientation {
            PageOrientation::Portrait => PageOrient::Portrait,
            PageOrientation::Landscape => PageOrient::Landscape,
            PageOrientation::Auto => PageOrient::Portrait,
        };
        
        // Генерация изображений
        let images = vec![
            item_z_data.draw_image_adaptive("as1")?,
            item_z_data.draw_image_adaptive("as2")?,
            item_z_data.draw_image_adaptive("as3")?,
            item_z_data.draw_image_adaptive("as4")?,
        ];
        
        // Добавление изображений в документ
        for (i, img_data) in images.iter().enumerate() {
            let img = Pic::new(img_data)
                .size(dimensions.docx_width_twips, dimensions.docx_height_twips);
                
            doc = doc
                .add_paragraph(Paragraph::new().add_run(Run::new().add_image(img)))
                .page_size(page_size.clone())
                .page_orient(page_orient);
                
            // Добавление заголовка
            if i == 0 {
                doc = doc.add_paragraph(
                    Paragraph::new().add_run(
                        Run::new().add_text(&format!("Floor Z: {}", z_coord))
                    )
                );
            }
        }
    }
    
    let mut buf = Vec::new();
    doc.build().pack(&mut buf)?;
    Ok(buf)
}
```

### 6.2 Задачи этапа 6
- [ ] Обновить функции генерации DOCX
- [ ] Добавить поддержку адаптивных размеров страниц
- [ ] Реализовать автоматическую ориентацию
- [ ] Создать fallback для совместимости
- [ ] Провести интеграционное тестирование

## Тестирование и валидация

### Модульные тесты

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_bounds_analysis() {
        // Тест анализа границ
    }
    
    #[test]
    fn test_dimension_calculation() {
        // Тест вычисления размеров
    }
    
    #[test]
    fn test_coordinate_scaling() {
        // Тест масштабирования
    }
    
    #[test]
    fn test_adaptive_rendering() {
        // Тест адаптивной отрисовки
    }
    
    #[test]
    fn test_a4_optimization() {
        // Тест оптимизации под A4
    }
}
```

### Интеграционные тесты

```rust
#[test]
fn test_full_pipeline() {
    // Тест полного пайплайна от данных до DOCX
}

#[test]
fn test_edge_cases() {
    // Тест граничных случаев
}

#[test]
fn test_performance() {
    // Тест производительности
}
```

## Критерии готовности

### Функциональные требования
- [ ] Автоматический анализ координатных границ
- [ ] Адаптивное вычисление размеров изображений
- [ ] Корректное масштабирование координат
- [ ] Качественная отрисовка с адаптивными шрифтами
- [ ] Оптимизация под формат A4
- [ ] Обратная совместимость с существующим API

### Качественные требования
- [ ] Покрытие тестами > 80%
- [ ] Время генерации < 5 сек для 10000 элементов
- [ ] Размер изображений оптимизирован для A4
- [ ] Читаемость текста при печати
- [ ] Корректная обработка ошибок

### Документация
- [ ] API документация
- [ ] Примеры использования
- [ ] Руководство по миграции
- [ ] Техническая документация

## Риски и митигация

### Технические риски
1. **Производительность**: Большие наборы данных могут замедлить обработку
   - *Митигация*: Оптимизация алгоритмов, параллельная обработка

2. **Качество изображений**: Автоматическое масштабирование может ухудшить качество
   - *Митигация*: Адаптивные алгоритмы, настройки качества

3. **Совместимость**: Изменения могут нарушить существующий функционал
   - *Митигация*: Обратная совместимость, тщательное тестирование

### Временные риски
1. **Сложность интеграции**: Интеграция может занять больше времени
   - *Митигация*: Поэтапная интеграция, раннее тестирование

2. **Неожиданные проблемы**: Могут возникнуть непредвиденные технические сложности
   - *Митигация*: Буферное время, альтернативные подходы

## Заключение

Данный план обеспечивает структурированный подход к реализации системы адаптивной генерации изображений. Каждый этап имеет четкие цели, задачи и критерии готовности, что позволяет контролировать прогресс и качество реализации.