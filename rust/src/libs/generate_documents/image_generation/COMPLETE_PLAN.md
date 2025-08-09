# Полный план реализации адаптивной системы генерации изображений

## Обзор проекта

### Цель
Создать адаптивную систему генерации изображений для DOCX документов, которая автоматически подстраивает размеры изображений и страниц под координаты фигур в каждом проекте, обеспечивая оптимальное размещение на формате A4.

### Ключевые требования
- ✅ Автоматическое определение границ координат
- ✅ Адаптивное масштабирование под A4 формат
- ✅ 4 изображения на этаж (as1, as2, as3, as4)
- ✅ Оптимальное качество и читаемость
- ✅ Обратная совместимость с существующим кодом
- ✅ Производительность и стабильность

## Структура проекта

```
image_generation/
├── mod.rs                    # Главный модуль
├── types.rs                  # Базовые типы данных
├── config.rs                 # Конфигурация A4 и настройки
├── bounds_analyzer.rs        # Анализ границ координат
├── dimension_calculator.rs   # Расчет оптимальных размеров
├── coordinate_scaler.rs      # Масштабирование координат
├── adaptive_renderer.rs      # Адаптивная отрисовка
├── a4_optimizer.rs          # Оптимизация под A4
├── utils.rs                 # Вспомогательные функции
├── tests/                   # Тесты
│   ├── mod.rs
│   ├── bounds_tests.rs
│   ├── dimension_tests.rs
│   ├── scaling_tests.rs
│   ├── rendering_tests.rs
│   └── integration_tests.rs
└── docs/                    # Документация
    ├── README.md
    ├── ARCHITECTURE.md
    ├── IMPLEMENTATION_PLAN.md
    ├── TECHNICAL_DOCUMENTATION.md
    ├── INTEGRATION_FLOW.md
    └── COMPLETE_PLAN.md
```

## Детальный план реализации

### Этап 1: Подготовка инфраструктуры (2-3 дня)

#### 1.1 Создание базовых типов данных

**Файл: `types.rs`**
```rust
// Основные структуры данных
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

pub struct OptimalDimensions {
    pub image_width: u32,
    pub image_height: u32,
    pub page_width_twips: u32,
    pub page_height_twips: u32,
    pub docx_width_twips: u32,
    pub docx_height_twips: u32,
    pub orientation: PageOrientation,
    pub target_dpi: f64,
    pub font_size: f32,
}

pub struct ScalingParams {
    pub scale_x: f64,
    pub scale_y: f64,
    pub offset_x: f64,
    pub offset_y: f64,
    pub uniform_scale: f64,
    pub margin_pixels: u32,
}

pub struct Margins {
    pub top: u32,
    pub bottom: u32,
    pub left: u32,
    pub right: u32,
}

pub enum PageOrientation {
    Portrait,
    Landscape,
}

pub enum QualityLevel {
    Draft,    // Быстро, низкое качество
    Standard, // Баланс качества и скорости
    High,     // Высокое качество, медленнее
}
```

#### 1.2 Создание конфигурации

**Файл: `config.rs`**
```rust
pub struct A4Config {
    // A4 размеры в twips (1 inch = 1440 twips)
    pub page_width_twips: u32,   // 11906 для portrait, 16838 для landscape
    pub page_height_twips: u32,  // 16838 для portrait, 11906 для landscape
    
    // Поля страницы в twips
    pub margins: Margins,
    
    // Настройки качества
    pub min_dpi: f64,
    pub max_dpi: f64,
    pub default_dpi: f64,
    
    // Настройки шрифта
    pub min_font_size: f32,
    pub max_font_size: f32,
    pub default_font_size: f32,
    
    // Настройки изображения
    pub min_image_width: u32,
    pub max_image_width: u32,
    pub min_image_height: u32,
    pub max_image_height: u32,
    
    // Настройки производительности
    pub enable_caching: bool,
    pub enable_parallel_processing: bool,
    pub quality_level: QualityLevel,
}

impl Default for A4Config {
    fn default() -> Self {
        Self {
            page_width_twips: 11906,  // A4 portrait
            page_height_twips: 16838,
            margins: Margins {
                top: 1440,    // 1 inch
                bottom: 1440,
                left: 1440,
                right: 1440,
            },
            min_dpi: 150.0,
            max_dpi: 600.0,
            default_dpi: 300.0,
            min_font_size: 8.0,
            max_font_size: 48.0,
            default_font_size: 12.0,
            min_image_width: 800,
            max_image_width: 8000,
            min_image_height: 600,
            max_image_height: 6000,
            enable_caching: true,
            enable_parallel_processing: false, // Для начала отключено
            quality_level: QualityLevel::Standard,
        }
    }
}
```

#### 1.3 Создание главного модуля

**Файл: `mod.rs`**
```rust
pub mod types;
pub mod config;
pub mod bounds_analyzer;
pub mod dimension_calculator;
pub mod coordinate_scaler;
pub mod adaptive_renderer;
pub mod a4_optimizer;
pub mod utils;

#[cfg(test)]
mod tests;

// Публичные экспорты
pub use types::*;
pub use config::*;
pub use bounds_analyzer::BoundsAnalyzer;
pub use dimension_calculator::DimensionCalculator;
pub use coordinate_scaler::CoordinateScaler;
pub use adaptive_renderer::AdaptiveRenderer;
pub use a4_optimizer::A4Optimizer;

// Главная функция для адаптивной генерации
pub fn create_adaptive_image(
    entities: &[crate::parse::EntityWithXlsx],
    field: &str,
    config: &A4Config,
) -> Result<(Vec<u8>, OptimalDimensions), String> {
    // Анализ границ
    let bounds = BoundsAnalyzer::analyze_bounds(entities)?;
    
    // Расчет оптимальных размеров
    let dimensions = DimensionCalculator::calculate_optimal_dimensions(&bounds, config)?;
    
    // Расчет параметров масштабирования
    let scaling = CoordinateScaler::calculate_scaling_params(&bounds, &dimensions)?;
    
    // Адаптивная отрисовка
    let image_bytes = AdaptiveRenderer::render_image(
        entities,
        field,
        &dimensions,
        &scaling,
        config,
    )?;
    
    Ok((image_bytes, dimensions))
}
```

### Этап 2: Анализ границ координат (1-2 дня)

#### 2.1 Реализация анализатора границ

**Файл: `bounds_analyzer.rs`**
```rust
use crate::parse::EntityWithXlsx;
use super::types::CoordinateBounds;

pub struct BoundsAnalyzer;

impl BoundsAnalyzer {
    pub fn analyze_bounds(entities: &[EntityWithXlsx]) -> Result<CoordinateBounds, String> {
        if entities.is_empty() {
            return Err("No entities provided for bounds analysis".to_string());
        }
        
        let mut min_x = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_y = f64::NEG_INFINITY;
        
        let mut point_count = 0;
        
        // Анализ всех вершин
        for entity in entities {
            for vertex in &entity.vertices {
                min_x = min_x.min(vertex.x);
                max_x = max_x.max(vertex.x);
                min_y = min_y.min(vertex.y);
                max_y = max_y.max(vertex.y);
                point_count += 1;
            }
        }
        
        if point_count == 0 {
            return Err("No vertices found in entities".to_string());
        }
        
        // Проверка на валидность границ
        if !min_x.is_finite() || !max_x.is_finite() || !min_y.is_finite() || !max_y.is_finite() {
            return Err("Invalid coordinate values detected".to_string());
        }
        
        let width = max_x - min_x;
        let height = max_y - min_y;
        
        if width <= 0.0 || height <= 0.0 {
            return Err("Invalid coordinate range detected".to_string());
        }
        
        let center_x = (min_x + max_x) / 2.0;
        let center_y = (min_y + max_y) / 2.0;
        let aspect_ratio = width / height;
        
        Ok(CoordinateBounds {
            min_x,
            max_x,
            min_y,
            max_y,
            width,
            height,
            center_x,
            center_y,
            aspect_ratio,
        })
    }
    
    pub fn detect_outliers(entities: &[EntityWithXlsx]) -> Result<Vec<usize>, String> {
        // Реализация обнаружения выбросов (опционально)
        // Может быть полезно для фильтрации аномальных координат
        Ok(Vec::new())
    }
    
    pub fn calculate_complexity(entities: &[EntityWithXlsx]) -> f64 {
        // Оценка сложности для определения качества отрисовки
        let total_vertices: usize = entities.iter()
            .map(|e| e.vertices.len())
            .sum();
        
        // Простая метрика сложности
        (total_vertices as f64).log10().max(1.0)
    }
}
```

### Этап 3: Расчет оптимальных размеров (2-3 дня)

#### 3.1 Реализация калькулятора размеров

**Файл: `dimension_calculator.rs`**
```rust
use super::types::*;
use super::config::A4Config;

pub struct DimensionCalculator;

impl DimensionCalculator {
    pub fn calculate_optimal_dimensions(
        bounds: &CoordinateBounds,
        config: &A4Config,
    ) -> Result<OptimalDimensions, String> {
        // 1. Определение ориентации страницы
        let orientation = Self::determine_orientation(bounds, config);
        
        // 2. Расчет доступной области на странице
        let (page_width, page_height) = Self::get_page_dimensions(&orientation, config);
        let usable_width = page_width - config.margins.left - config.margins.right;
        let usable_height = page_height - config.margins.top - config.margins.bottom;
        
        // 3. Расчет целевого DPI на основе сложности
        let complexity = crate::bounds_analyzer::BoundsAnalyzer::calculate_complexity(&[]);
        let target_dpi = Self::calculate_target_dpi(complexity, config);
        
        // 4. Расчет размеров изображения
        let (image_width, image_height) = Self::calculate_image_dimensions(
            bounds,
            usable_width,
            usable_height,
            target_dpi,
            config,
        )?;
        
        // 5. Расчет размеров для DOCX (в twips)
        let (docx_width_twips, docx_height_twips) = Self::calculate_docx_dimensions(
            image_width,
            image_height,
            target_dpi,
        );
        
        // 6. Расчет размера шрифта
        let font_size = Self::calculate_font_size(image_width, image_height, config);
        
        Ok(OptimalDimensions {
            image_width,
            image_height,
            page_width_twips: page_width,
            page_height_twips: page_height,
            docx_width_twips,
            docx_height_twips,
            orientation,
            target_dpi,
            font_size,
        })
    }
    
    fn determine_orientation(bounds: &CoordinateBounds, _config: &A4Config) -> PageOrientation {
        // Если ширина больше высоты, используем landscape
        if bounds.aspect_ratio > 1.2 {
            PageOrientation::Landscape
        } else {
            PageOrientation::Portrait
        }
    }
    
    fn get_page_dimensions(orientation: &PageOrientation, config: &A4Config) -> (u32, u32) {
        match orientation {
            PageOrientation::Portrait => (11906, 16838),   // A4 portrait в twips
            PageOrientation::Landscape => (16838, 11906),  // A4 landscape в twips
        }
    }
    
    fn calculate_target_dpi(complexity: f64, config: &A4Config) -> f64 {
        // Адаптивный DPI на основе сложности
        let base_dpi = config.default_dpi;
        let complexity_factor = (complexity / 3.0).min(2.0); // Ограничиваем фактор
        
        let target_dpi = base_dpi * (1.0 + complexity_factor * 0.5);
        target_dpi.clamp(config.min_dpi, config.max_dpi)
    }
    
    fn calculate_image_dimensions(
        bounds: &CoordinateBounds,
        usable_width_twips: u32,
        usable_height_twips: u32,
        target_dpi: f64,
        config: &A4Config,
    ) -> Result<(u32, u32), String> {
        // Конвертация twips в дюймы (1 inch = 1440 twips)
        let usable_width_inches = usable_width_twips as f64 / 1440.0;
        let usable_height_inches = usable_height_twips as f64 / 1440.0;
        
        // Расчет размеров изображения в пикселях
        let max_width_pixels = (usable_width_inches * target_dpi) as u32;
        let max_height_pixels = (usable_height_inches * target_dpi) as u32;
        
        // Подгонка под соотношение сторон координат
        let (image_width, image_height) = if bounds.aspect_ratio > (max_width_pixels as f64 / max_height_pixels as f64) {
            // Ограничиваем по ширине
            let width = max_width_pixels;
            let height = (width as f64 / bounds.aspect_ratio) as u32;
            (width, height)
        } else {
            // Ограничиваем по высоте
            let height = max_height_pixels;
            let width = (height as f64 * bounds.aspect_ratio) as u32;
            (width, height)
        };
        
        // Проверка ограничений
        let final_width = image_width.clamp(config.min_image_width, config.max_image_width);
        let final_height = image_height.clamp(config.min_image_height, config.max_image_height);
        
        Ok((final_width, final_height))
    }
    
    fn calculate_docx_dimensions(image_width: u32, image_height: u32, target_dpi: f64) -> (u32, u32) {
        // Конвертация размеров изображения в twips для DOCX
        let width_inches = image_width as f64 / target_dpi;
        let height_inches = image_height as f64 / target_dpi;
        
        let width_twips = (width_inches * 1440.0) as u32;
        let height_twips = (height_inches * 1440.0) as u32;
        
        (width_twips, height_twips)
    }
    
    fn calculate_font_size(image_width: u32, image_height: u32, config: &A4Config) -> f32 {
        // Адаптивный размер шрифта на основе размеров изображения
        let base_size = config.default_font_size;
        let scale_factor = ((image_width * image_height) as f64 / (2000.0 * 1500.0)).sqrt();
        
        let font_size = base_size * scale_factor as f32;
        font_size.clamp(config.min_font_size, config.max_font_size)
    }
}
```

### Этап 4: Масштабирование координат (1-2 дня)

#### 4.1 Реализация масштабировщика координат

**Файл: `coordinate_scaler.rs`**
```rust
use super::types::*;

pub struct CoordinateScaler;

impl CoordinateScaler {
    pub fn calculate_scaling_params(
        bounds: &CoordinateBounds,
        dimensions: &OptimalDimensions,
    ) -> Result<ScalingParams, String> {
        // Расчет отступов (10% от размера изображения)
        let margin_pixels = (dimensions.image_width.min(dimensions.image_height) as f64 * 0.1) as u32;
        
        // Доступная область для отрисовки
        let available_width = dimensions.image_width - 2 * margin_pixels;
        let available_height = dimensions.image_height - 2 * margin_pixels;
        
        // Расчет масштабов по осям
        let scale_x = available_width as f64 / bounds.width;
        let scale_y = available_height as f64 / bounds.height;
        
        // Используем единый масштаб для сохранения пропорций
        let uniform_scale = scale_x.min(scale_y);
        
        // Расчет смещений для центрирования
        let scaled_width = bounds.width * uniform_scale;
        let scaled_height = bounds.height * uniform_scale;
        
        let offset_x = (dimensions.image_width as f64 - scaled_width) / 2.0;
        let offset_y = (dimensions.image_height as f64 - scaled_height) / 2.0;
        
        Ok(ScalingParams {
            scale_x: uniform_scale,
            scale_y: uniform_scale,
            offset_x,
            offset_y,
            uniform_scale,
            margin_pixels,
        })
    }
    
    pub fn scale_point(
        x: f64,
        y: f64,
        bounds: &CoordinateBounds,
        scaling: &ScalingParams,
    ) -> (f32, f32) {
        // Нормализация координат относительно минимальных значений
        let normalized_x = x - bounds.min_x;
        let normalized_y = y - bounds.min_y;
        
        // Применение масштабирования и смещения
        let scaled_x = normalized_x * scaling.uniform_scale + scaling.offset_x;
        let scaled_y = normalized_y * scaling.uniform_scale + scaling.offset_y;
        
        (scaled_x as f32, scaled_y as f32)
    }
    
    pub fn scale_font_size(base_font_size: f32, scaling: &ScalingParams) -> f32 {
        // Адаптация размера шрифта к масштабу
        let scale_factor = scaling.uniform_scale.sqrt() as f32;
        (base_font_size * scale_factor).max(8.0).min(48.0)
    }
}
```

### Этап 5: Адаптивная отрисовка (2-3 дня)

#### 5.1 Реализация адаптивного рендерера

**Файл: `adaptive_renderer.rs`**
```rust
use image::{ImageBuffer, Rgb, RgbImage};
use imageproc::drawing::{draw_line_segment_mut, draw_text_mut};
use rusttype::{Font, Scale};
use crate::parse::EntityWithXlsx;
use super::types::*;
use super::config::A4Config;
use super::coordinate_scaler::CoordinateScaler;

pub struct AdaptiveRenderer;

impl AdaptiveRenderer {
    pub fn render_image(
        entities: &[EntityWithXlsx],
        field: &str,
        dimensions: &OptimalDimensions,
        scaling: &ScalingParams,
        config: &A4Config,
    ) -> Result<Vec<u8>, String> {
        // Создание буфера изображения
        let mut image: RgbImage = ImageBuffer::new(
            dimensions.image_width,
            dimensions.image_height,
        );
        
        // Заливка белым фоном
        for pixel in image.pixels_mut() {
            *pixel = Rgb([255, 255, 255]);
        }
        
        // Загрузка шрифта
        let font_data = include_bytes!("../../../assets/DejaVuSans.ttf");
        let font = Font::try_from_bytes(font_data as &[u8])
            .ok_or("Failed to load font")?;
        
        // Расчет границ для данного поля
        let bounds = crate::bounds_analyzer::BoundsAnalyzer::analyze_bounds(entities)?;
        
        // Отрисовка сущностей
        for entity in entities {
            Self::draw_entity(&mut image, entity, field, &bounds, scaling, dimensions, &font)?;
        }
        
        // Конвертация в PNG
        let mut png_data = Vec::new();
        {
            let mut cursor = std::io::Cursor::new(&mut png_data);
            image.write_to(&mut cursor, image::ImageOutputFormat::Png)
                .map_err(|e| format!("Failed to encode PNG: {}", e))?;
        }
        
        Ok(png_data)
    }
    
    fn draw_entity(
        image: &mut RgbImage,
        entity: &EntityWithXlsx,
        field: &str,
        bounds: &CoordinateBounds,
        scaling: &ScalingParams,
        dimensions: &OptimalDimensions,
        font: &Font,
    ) -> Result<(), String> {
        let vertices = &entity.vertices;
        
        if vertices.len() < 3 {
            return Ok(()); // Пропускаем сущности с недостаточным количеством вершин
        }
        
        // Получение значения поля
        let field_value = Self::get_field_value(entity, field);
        
        // Масштабирование координат вершин
        let scaled_points: Vec<(f32, f32)> = vertices
            .iter()
            .map(|v| CoordinateScaler::scale_point(v.x, v.y, bounds, scaling))
            .collect();
        
        // Отрисовка контура
        Self::draw_polygon_outline(image, &scaled_points)?;
        
        // Отрисовка текста в центре
        if !field_value.is_empty() {
            let center = Self::calculate_polygon_center(&scaled_points);
            Self::draw_text_at_point(
                image,
                &field_value,
                center,
                dimensions.font_size,
                font,
            )?;
        }
        
        Ok(())
    }
    
    fn get_field_value(entity: &EntityWithXlsx, field: &str) -> String {
        match field {
            "as1" => entity.as1.clone().unwrap_or_default(),
            "as2" => entity.as2.clone().unwrap_or_default(),
            "as3" => entity.as3.clone().unwrap_or_default(),
            "as4" => entity.as4.clone().unwrap_or_default(),
            _ => String::new(),
        }
    }
    
    fn draw_polygon_outline(
        image: &mut RgbImage,
        points: &[(f32, f32)],
    ) -> Result<(), String> {
        if points.len() < 2 {
            return Ok(());
        }
        
        let line_color = Rgb([0, 0, 0]); // Черный цвет
        
        // Рисуем линии между соседними точками
        for i in 0..points.len() {
            let start = points[i];
            let end = points[(i + 1) % points.len()];
            
            draw_line_segment_mut(
                image,
                start,
                end,
                line_color,
            );
        }
        
        Ok(())
    }
    
    fn calculate_polygon_center(points: &[(f32, f32)]) -> (f32, f32) {
        let sum_x: f32 = points.iter().map(|p| p.0).sum();
        let sum_y: f32 = points.iter().map(|p| p.1).sum();
        let count = points.len() as f32;
        
        (sum_x / count, sum_y / count)
    }
    
    fn draw_text_at_point(
        image: &mut RgbImage,
        text: &str,
        position: (f32, f32),
        font_size: f32,
        font: &Font,
    ) -> Result<(), String> {
        let scale = Scale::uniform(font_size);
        let text_color = Rgb([0, 0, 0]); // Черный цвет
        
        draw_text_mut(
            image,
            text_color,
            position.0 as i32,
            position.1 as i32,
            scale,
            font,
            text,
        );
        
        Ok(())
    }
}
```

### Этап 6: Интеграция с DOCX (1-2 дня)

#### 6.1 Обновление DrawItemZ

**Изменения в `drawItem.rs`:**
```rust
// Добавить в структуру DrawItemZ
use crate::libs::image_generation::*;

pub struct DrawItemZ {
    pub data: Vec<EntityWithXlsx>,
    // Новые поля для адаптивной генерации
    pub bounds: Option<CoordinateBounds>,
    pub dimensions: Option<OptimalDimensions>,
    pub scaling: Option<ScalingParams>,
    pub config: A4Config,
}

impl DrawItemZ {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            bounds: None,
            dimensions: None,
            scaling: None,
            config: A4Config::default(),
        }
    }
    
    pub fn calculate_adaptive_params(&mut self) -> Result<(), String> {
        // Анализ границ
        let bounds = BoundsAnalyzer::analyze_bounds(&self.data)?;
        
        // Расчет оптимальных размеров
        let dimensions = DimensionCalculator::calculate_optimal_dimensions(&bounds, &self.config)?;
        
        // Расчет параметров масштабирования
        let scaling = CoordinateScaler::calculate_scaling_params(&bounds, &dimensions)?;
        
        // Сохранение параметров
        self.bounds = Some(bounds);
        self.dimensions = Some(dimensions);
        self.scaling = Some(scaling);
        
        Ok(())
    }
    
    pub fn draw_image_adaptive(&self, field: &str) -> Result<Vec<u8>, String> {
        // Проверка наличия параметров
        let dimensions = self.dimensions.as_ref()
            .ok_or("Adaptive parameters not calculated")?;
        let scaling = self.scaling.as_ref()
            .ok_or("Scaling parameters not calculated")?;
        
        // Адаптивная отрисовка
        AdaptiveRenderer::render_image(
            &self.data,
            field,
            dimensions,
            scaling,
            &self.config,
        )
    }
    
    pub fn draw_all_images_adaptive(&self) -> Result<Vec<Vec<u8>>, String> {
        let fields = ["as1", "as2", "as3", "as4"];
        let mut images = Vec::new();
        
        for field in &fields {
            let image = self.draw_image_adaptive(field)?;
            images.push(image);
        }
        
        Ok(images)
    }
    
    // Сохраняем старые методы для обратной совместимости
    pub fn draw_image_as1(&self, field: &str) -> Vec<u8> {
        // Старая реализация...
        // Можно добавить fallback к адаптивному методу
        match self.draw_image_adaptive(field) {
            Ok(image) => image,
            Err(_) => {
                // Fallback к старому методу
                self.draw_image_as1_legacy(field)
            }
        }
    }
}
```

#### 6.2 Обновление docx_generator.rs

**Изменения в `docx_generator.rs`:**
```rust
use crate::libs::image_generation::*;

pub fn create_docx_document_adaptive(
    hash_grouped_by_z: HashMap<String, DrawItemZ>,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut docx = Docx::new();
    
    for (z_key, mut item_z) in hash_grouped_by_z {
        // Расчет адаптивных параметров
        if let Err(e) = item_z.calculate_adaptive_params() {
            eprintln!("Failed to calculate adaptive params for {}: {}", z_key, e);
            continue;
        }
        
        // Получение размеров
        let dimensions = item_z.dimensions.as_ref().unwrap();
        
        // Установка размеров страницы
        let page_size = PageSize::new()
            .width(dimensions.page_width_twips)
            .height(dimensions.page_height_twips);
        
        docx = docx.page_size(page_size);
        
        // Генерация изображений
        match item_z.draw_all_images_adaptive() {
            Ok(images) => {
                let fields = ["as1", "as2", "as3", "as4"];
                
                for (i, image_data) in images.iter().enumerate() {
                    let field_name = fields[i];
                    
                    // Создание изображения с адаптивными размерами
                    let pic = Pic::new(image_data)
                        .size(
                            dimensions.docx_width_twips,
                            dimensions.docx_height_twips,
                        );
                    
                    // Добавление заголовка
                    docx = docx.add_paragraph(
                        Paragraph::new()
                            .add_run(Run::new().add_text(&format!("Floor {} - {}", z_key, field_name)))
                    );
                    
                    // Добавление изображения
                    docx = docx.add_paragraph(
                        Paragraph::new().add_run(Run::new().add_image(pic))
                    );
                }
            }
            Err(e) => {
                eprintln!("Failed to generate adaptive images for {}: {}", z_key, e);
                // Fallback к старому методу
                let images = item_z.draw_all_images();
                // ... обработка старым способом
            }
        }
    }
    
    let mut buf = Vec::new();
    docx.build().pack(&mut buf)?;
    Ok(buf)
}

pub fn create_docx_for_selected_floors_adaptive(
    hash_grouped_by_z: HashMap<String, DrawItemZ>,
    selected_floors: Vec<String>,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut docx = Docx::new();
    
    for floor in selected_floors {
        if let Some(mut item_z) = hash_grouped_by_z.get(&floor).cloned() {
            // Расчет адаптивных параметров
            if let Err(e) = item_z.calculate_adaptive_params() {
                eprintln!("Failed to calculate adaptive params for floor {}: {}", floor, e);
                continue;
            }
            
            // Получение размеров
            let dimensions = item_z.dimensions.as_ref().unwrap();
            
            // Установка размеров страницы для каждого этажа
            let page_size = PageSize::new()
                .width(dimensions.page_width_twips)
                .height(dimensions.page_height_twips);
            
            docx = docx.page_size(page_size);
            
            // Добавление заголовка этажа
            docx = docx.add_paragraph(
                Paragraph::new()
                    .add_run(Run::new().add_text(&format!("Floor: {}", floor)))
                    .style("Heading1")
            );
            
            // Генерация и добавление изображений
            match item_z.draw_all_images_adaptive() {
                Ok(images) => {
                    let fields = ["as1", "as2", "as3", "as4"];
                    
                    for (i, image_data) in images.iter().enumerate() {
                        let field_name = fields[i];
                        
                        let pic = Pic::new(image_data)
                            .size(
                                dimensions.docx_width_twips,
                                dimensions.docx_height_twips,
                            );
                        
                        docx = docx.add_paragraph(
                            Paragraph::new()
                                .add_run(Run::new().add_text(&format!("{}: ", field_name)))
                        );
                        
                        docx = docx.add_paragraph(
                            Paragraph::new().add_run(Run::new().add_image(pic))
                        );
                    }
                }
                Err(e) => {
                    eprintln!("Failed to generate adaptive images for floor {}: {}", floor, e);
                }
            }
            
            // Добавление разрыва страницы между этажами
            docx = docx.add_paragraph(Paragraph::new().page_break_before(true));
        }
    }
    
    let mut buf = Vec::new();
    docx.build().pack(&mut buf)?;
    Ok(buf)
}
```

## Тестирование и валидация

### Юнит-тесты

**Файл: `tests/bounds_tests.rs`**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::{EntityWithXlsx, Vertex};
    
    #[test]
    fn test_bounds_analysis_simple() {
        let entities = vec![
            EntityWithXlsx {
                vertices: vec![
                    Vertex { x: 0.0, y: 0.0, z: 0.0 },
                    Vertex { x: 10.0, y: 10.0, z: 0.0 },
                ],
                // ... другие поля
            }
        ];
        
        let bounds = BoundsAnalyzer::analyze_bounds(&entities).unwrap();
        
        assert_eq!(bounds.min_x, 0.0);
        assert_eq!(bounds.max_x, 10.0);
        assert_eq!(bounds.min_y, 0.0);
        assert_eq!(bounds.max_y, 10.0);
        assert_eq!(bounds.width, 10.0);
        assert_eq!(bounds.height, 10.0);
        assert_eq!(bounds.aspect_ratio, 1.0);
    }
    
    #[test]
    fn test_bounds_analysis_empty() {
        let entities = vec![];
        let result = BoundsAnalyzer::analyze_bounds(&entities);
        assert!(result.is_err());
    }
}
```

### Интеграционные тесты

**Файл: `tests/integration_tests.rs`**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_full_adaptive_pipeline() {
        // Создание тестовых данных
        let entities = create_test_entities();
        let config = A4Config::default();
        
        // Тестирование полного пайплайна
        let result = crate::libs::image_generation::create_adaptive_image(
            &entities,
            "as1",
            &config,
        );
        
        assert!(result.is_ok());
        let (image_data, dimensions) = result.unwrap();
        
        // Проверка результатов
        assert!(!image_data.is_empty());
        assert!(dimensions.image_width > 0);
        assert!(dimensions.image_height > 0);
        assert!(dimensions.docx_width_twips > 0);
        assert!(dimensions.docx_height_twips > 0);
    }
    
    fn create_test_entities() -> Vec<EntityWithXlsx> {
        // Создание тестовых сущностей
        vec![
            EntityWithXlsx {
                vertices: vec![
                    Vertex { x: 0.0, y: 0.0, z: 0.0 },
                    Vertex { x: 100.0, y: 0.0, z: 0.0 },
                    Vertex { x: 100.0, y: 100.0, z: 0.0 },
                    Vertex { x: 0.0, y: 100.0, z: 0.0 },
                ],
                as1: Some("Test1".to_string()),
                as2: Some("Test2".to_string()),
                as3: Some("Test3".to_string()),
                as4: Some("Test4".to_string()),
                // ... другие поля
            }
        ]
    }
}
```

## Конфигурация и настройка

### Переменные окружения
```bash
# Настройки качества
ADAPTIVE_IMAGE_QUALITY=standard  # draft, standard, high
ADAPTIVE_MIN_DPI=150
ADAPTIVE_MAX_DPI=600
ADAPTIVE_DEFAULT_DPI=300

# Настройки производительности
ADAPTIVE_ENABLE_CACHING=true
ADAPTIVE_ENABLE_PARALLEL=false

# Настройки отладки
ADAPTIVE_DEBUG_MODE=false
ADAPTIVE_LOG_LEVEL=info
```

### Флаги компиляции
```toml
# В Cargo.toml
[features]
default = ["adaptive-rendering"]
adaptive-rendering = []
legacy-rendering = []
debug-adaptive = []
```

## Мониторинг и метрики

### Логирование
```rust
// Добавить в каждый модуль
use log::{info, warn, error, debug};

// Пример использования
info!("Analyzing bounds for {} entities", entities.len());
debug!("Calculated bounds: {:?}", bounds);
warn!("Using fallback to legacy rendering due to: {}", error);
error!("Failed to generate adaptive image: {}", error);
```

### Метрики производительности
```rust
use std::time::Instant;

pub struct PerformanceMetrics {
    pub bounds_analysis_time: Duration,
    pub dimension_calculation_time: Duration,
    pub scaling_calculation_time: Duration,
    pub rendering_time: Duration,
    pub total_time: Duration,
    pub memory_usage: usize,
}

impl PerformanceMetrics {
    pub fn measure<F, R>(operation: F) -> (R, Duration)
    where
        F: FnOnce() -> R,
    {
        let start = Instant::now();
        let result = operation();
        let duration = start.elapsed();
        (result, duration)
    }
}
```

## Развертывание и миграция

### Поэтапное развертывание

1. **Этап 1**: Развертывание с флагом `legacy-rendering` (по умолчанию)
2. **Этап 2**: A/B тестирование с частичным включением `adaptive-rendering`
3. **Этап 3**: Полное переключение на `adaptive-rendering`
4. **Этап 4**: Удаление legacy кода

### Скрипт миграции
```bash
#!/bin/bash
# migrate_to_adaptive.sh

echo "Starting migration to adaptive rendering..."

# Резервное копирование
cp -r src/libs/drawItem.rs src/libs/drawItem.rs.backup
cp -r src/libs/docx_generator.rs src/libs/docx_generator.rs.backup

# Компиляция с новыми модулями
cargo build --features adaptive-rendering

if [ $? -eq 0 ]; then
    echo "✅ Compilation successful"
    
    # Запуск тестов
    cargo test --features adaptive-rendering
    
    if [ $? -eq 0 ]; then
        echo "✅ All tests passed"
        echo "Migration completed successfully!"
    else
        echo "❌ Tests failed, rolling back..."
        # Откат изменений
        mv src/libs/drawItem.rs.backup src/libs/drawItem.rs
        mv src/libs/docx_generator.rs.backup src/libs/docx_generator.rs
    fi
else
    echo "❌ Compilation failed, rolling back..."
    # Откат изменений
    mv src/libs/drawItem.rs.backup src/libs/drawItem.rs
    mv src/libs/docx_generator.rs.backup src/libs/docx_generator.rs
fi
```

## Заключение

Данный план предоставляет:

✅ **Полную архитектуру** адаптивной системы генерации изображений  
✅ **Детальную реализацию** всех компонентов  
✅ **Интеграцию** с существующим кодом  
✅ **Обратную совместимость** и безопасную миграцию  
✅ **Комплексное тестирование** и валидацию  
✅ **Мониторинг** и метрики производительности  
✅ **Документацию** и примеры использования  

Система будет автоматически адаптировать размеры изображений и страниц под координаты фигур в каждом проекте, обеспечивая оптимальное размещение на формате A4 с высоким качеством и читаемостью.

**Следующие шаги:**
1. Создание структуры папок и файлов
2. Реализация базовых типов и конфигурации
3. Поэтапная реализация модулей согласно плану
4. Интеграционное тестирование
5. Постепенное развертывание в продакшн

Время реализации: **2-3 недели** при работе одного разработчика.