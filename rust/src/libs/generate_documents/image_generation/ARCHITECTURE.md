# Архитектура системы адаптивной генерации изображений

## Общая архитектура

```mermaid
flowchart TD
    subgraph "Входные данные"
        A[EntityWithXlsx Vector]
        A1[Vertices: Vec&lt;Vertex&gt;]
        A2[Material, Layer, etc.]
    end
    
    subgraph "Анализ данных"
        B[BoundsAnalyzer]
        B1[calculate_bounds]
        B2[analyze_density]
        B3[detect_outliers]
    end
    
    subgraph "Вычисление параметров"
        C[DimensionCalculator]
        C1[calculate_optimal_size]
        C2[determine_orientation]
        C3[calculate_dpi]
        C4[calculate_font_size]
    end
    
    subgraph "Масштабирование"
        D[CoordinateScaler]
        D1[calculate_scale_factor]
        D2[apply_margins]
        D3[center_content]
    end
    
    subgraph "Отрисовка"
        E[AdaptiveRenderer]
        E1[create_image_buffer]
        E2[draw_shapes]
        E3[draw_text]
        E4[apply_antialiasing]
    end
    
    subgraph "Оптимизация A4"
        F[A4Optimizer]
        F1[validate_printability]
        F2[adjust_for_printer]
        F3[optimize_file_size]
    end
    
    subgraph "Выходные данные"
        G[Optimized Images]
        G1[PNG Buffers]
        G2[DOCX Dimensions]
        G3[Metadata]
    end
    
    A --> B
    A1 --> B1
    A2 --> B2
    
    B --> C
    B1 --> C1
    B2 --> C3
    B3 --> C2
    
    C --> D
    C1 --> D1
    C2 --> D2
    C4 --> D3
    
    D --> E
    D1 --> E1
    D2 --> E2
    D3 --> E3
    
    E --> F
    E1 --> F1
    E4 --> F2
    
    F --> G
    F1 --> G1
    F2 --> G2
    F3 --> G3
```

## Детальная архитектура компонентов

### 1. BoundsAnalyzer

```mermaid
classDiagram
    class BoundsAnalyzer {
        +analyze_bounds(entities: &[EntityWithXlsx]) CoordinateBounds
        +calculate_density(bounds: &CoordinateBounds, count: usize) f64
        +detect_outliers(entities: &[EntityWithXlsx]) Vec~usize~
        +filter_outliers(entities: &[EntityWithXlsx], outliers: &[usize]) Vec~EntityWithXlsx~
    }
    
    class CoordinateBounds {
        +min_x: f64
        +max_x: f64
        +min_y: f64
        +max_y: f64
        +width: f64
        +height: f64
        +aspect_ratio: f64
        +center_x: f64
        +center_y: f64
        +area: f64
    }
    
    BoundsAnalyzer --> CoordinateBounds
```

### 2. DimensionCalculator

```mermaid
classDiagram
    class DimensionCalculator {
        +calculate_optimal_dimensions(bounds: &CoordinateBounds, config: &A4Config) OptimalDimensions
        +determine_orientation(aspect_ratio: f64) PageOrientation
        +calculate_target_dpi(complexity: f64, quality: QualityLevel) u32
        +calculate_font_size(scale_factor: f64, config: &A4Config) f32
    }
    
    class OptimalDimensions {
        +image_width: u32
        +image_height: u32
        +docx_width_twips: u32
        +docx_height_twips: u32
        +page_width_twips: u32
        +page_height_twips: u32
        +scale_factor: f64
        +font_size: f32
        +dpi: u32
        +orientation: PageOrientation
    }
    
    DimensionCalculator --> OptimalDimensions
```

### 3. CoordinateScaler

```mermaid
classDiagram
    class CoordinateScaler {
        +calculate_scaling_params(bounds: &CoordinateBounds, dimensions: &OptimalDimensions) ScalingParams
        +scale_point(point: &Vertex, params: &ScalingParams) (f32, f32)
        +apply_margins(dimensions: &OptimalDimensions, margins: &Margins) (u32, u32)
    }
    
    class ScalingParams {
        +scale_factor: f64
        +offset_x: f64
        +offset_y: f64
        +margin_left: f64
        +margin_top: f64
        +usable_width: f64
        +usable_height: f64
    }
    
    class Margins {
        +left: f64
        +right: f64
        +top: f64
        +bottom: f64
    }
    
    CoordinateScaler --> ScalingParams
    ScalingParams --> Margins
```

## Поток данных

```mermaid
sequenceDiagram
    participant Client
    participant DrawItemZ
    participant BoundsAnalyzer
    participant DimensionCalculator
    participant CoordinateScaler
    participant AdaptiveRenderer
    participant A4Optimizer
    
    Client->>DrawItemZ: add_entities(entities)
    Client->>DrawItemZ: calculate_adaptive_params()
    
    DrawItemZ->>BoundsAnalyzer: analyze_bounds(entities)
    BoundsAnalyzer-->>DrawItemZ: CoordinateBounds
    
    DrawItemZ->>DimensionCalculator: calculate_optimal_dimensions(bounds, config)
    DimensionCalculator-->>DrawItemZ: OptimalDimensions
    
    DrawItemZ->>CoordinateScaler: calculate_scaling_params(bounds, dimensions)
    CoordinateScaler-->>DrawItemZ: ScalingParams
    
    Client->>DrawItemZ: draw_image_adaptive(field)
    DrawItemZ->>AdaptiveRenderer: render(entities, scaling, dimensions)
    AdaptiveRenderer-->>DrawItemZ: ImageBuffer
    
    DrawItemZ->>A4Optimizer: optimize_for_a4(image, dimensions)
    A4Optimizer-->>DrawItemZ: OptimizedImage
    
    DrawItemZ-->>Client: Vec<u8> (PNG)
```

## Алгоритмы вычислений

### Расчет оптимальных размеров

```mermaid
flowchart TD
    A[Входные границы] --> B{Aspect Ratio > 1.4?}
    B -->|Да| C[Landscape A4]
    B -->|Нет| D[Portrait A4]
    
    C --> E[Рабочая область: 15000x10000 twips]
    D --> F[Рабочая область: 10000x15000 twips]
    
    E --> G[Расчет scale_factor]
    F --> G
    
    G --> H{Сложность > threshold?}
    H -->|Да| I[High DPI: 600]
    H -->|Нет| J[Standard DPI: 300]
    
    I --> K[Финальные размеры]
    J --> K
    
    K --> L[Валидация размеров]
    L --> M{Размер > MAX?}
    M -->|Да| N[Уменьшить DPI]
    M -->|Нет| O[Готово]
    
    N --> K
```

### Масштабирование координат

```mermaid
flowchart TD
    A[Исходные координаты] --> B[Нормализация к [0,1]]
    B --> C[Применение отступов]
    C --> D[Масштабирование к целевому размеру]
    D --> E[Центрирование]
    E --> F[Финальные координаты]
    
    subgraph "Формулы"
        G["normalized_x = (x - min_x) / width"]
        H["scaled_x = normalized_x * usable_width + margin_left"]
        I["final_x = scaled_x + center_offset_x"]
    end
```

## Конфигурация и настройки

```mermaid
classDiagram
    class A4Config {
        +orientation: PageOrientation
        +margins: Margins
        +quality: QualityLevel
        +min_font_size: f32
        +max_font_size: f32
        +target_dpi_range: (u32, u32)
        +max_image_size: (u32, u32)
        +antialiasing: bool
        +compression_level: u8
    }
    
    class PageOrientation {
        <<enumeration>>
        Portrait
        Landscape
        Auto
    }
    
    class QualityLevel {
        <<enumeration>>
        Draft
        Standard
        High
        Ultra
    }
    
    A4Config --> PageOrientation
    A4Config --> QualityLevel
```

## Оптимизации производительности

### Кэширование

```mermaid
flowchart LR
    A[Входные данные] --> B{Кэш bounds?}
    B -->|Есть| C[Использовать кэш]
    B -->|Нет| D[Вычислить bounds]
    D --> E[Сохранить в кэш]
    C --> F[Продолжить обработку]
    E --> F
```

### Параллельная обработка

```mermaid
flowchart TD
    A[Список entities] --> B[Разделить на chunks]
    B --> C[Параллельная обработка]
    
    subgraph "Parallel Processing"
        D[Thread 1: Chunk 1]
        E[Thread 2: Chunk 2]
        F[Thread 3: Chunk 3]
        G[Thread N: Chunk N]
    end
    
    C --> D
    C --> E
    C --> F
    C --> G
    
    D --> H[Объединение результатов]
    E --> H
    F --> H
    G --> H
    
    H --> I[Финальный результат]
```

## Обработка ошибок

```mermaid
flowchart TD
    A[Начало обработки] --> B{Валидные данные?}
    B -->|Нет| C[Возврат ошибки]
    B -->|Да| D[Анализ границ]
    
    D --> E{Границы валидны?}
    E -->|Нет| F[Fallback к значениям по умолчанию]
    E -->|Да| G[Вычисление размеров]
    
    F --> G
    G --> H{Размеры в пределах лимитов?}
    H -->|Нет| I[Корректировка размеров]
    H -->|Да| J[Отрисовка]
    
    I --> J
    J --> K{Отрисовка успешна?}
    K -->|Нет| L[Упрощенная отрисовка]
    K -->|Да| M[Успех]
    
    L --> M
```

## Интеграционные точки

### С существующим кодом

```mermaid
flowchart LR
    subgraph "Существующий код"
        A[docx_generator.rs]
        B[drawItem.rs]
        C[parse.rs]
    end
    
    subgraph "Новый модуль"
        D[image_generation/]
        E[BoundsAnalyzer]
        F[DimensionCalculator]
        G[AdaptiveRenderer]
    end
    
    A -.->|Использует| D
    B -.->|Расширяется| G
    C -->|Данные| E
    
    D -->|Новые методы| A
    G -->|Заменяет| B
```

### API совместимость

```rust
// Старый API (сохраняется)
impl DrawItemZ {
    pub fn draw_all_images(&self) -> Vec<Vec<u8>> {
        // Fallback к новому API с настройками по умолчанию
        self.draw_all_images_adaptive_with_config(&A4Config::default())
    }
}

// Новый API
impl DrawItemZ {
    pub fn draw_all_images_adaptive(&self) -> Vec<Vec<u8>>
    pub fn draw_all_images_adaptive_with_config(&self, config: &A4Config) -> Vec<Vec<u8>>
    pub fn get_optimal_docx_dimensions(&self) -> (u32, u32)
}
```