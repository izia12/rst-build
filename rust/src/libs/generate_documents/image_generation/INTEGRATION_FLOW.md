# Диаграмма интеграции с существующим кодом

## Текущая архитектура vs Новая архитектура

```mermaid
graph TB
    subgraph "Текущая система (До изменений)"
        A1[EntityWithXlsx Data] --> B1[DrawItemZ::new()]
        B1 --> C1[draw_image_as1() - Фиксированные размеры]
        C1 --> D1[Фиксированное масштабирование 40.0]
        D1 --> E1[Фиксированные размеры изображения 2500x2000]
        E1 --> F1[Фиксированные размеры DOCX 20M x 15M twips]
        F1 --> G1[create_docx_document()]
        G1 --> H1[DOCX с фиксированными размерами]
    end
    
    subgraph "Новая система (После изменений)"
        A2[EntityWithXlsx Data] --> B2[DrawItemZ::new()]
        B2 --> C2[calculate_adaptive_params()]
        C2 --> D2[BoundsAnalyzer]
        D2 --> E2[DimensionCalculator]
        E2 --> F2[CoordinateScaler]
        F2 --> G2[draw_image_adaptive()]
        G2 --> H2[Адаптивные размеры изображения]
        H2 --> I2[Адаптивные размеры DOCX]
        I2 --> J2[create_docx_document_adaptive()]
        J2 --> K2[DOCX с адаптивными размерами]
    end
    
    style A1 fill:#ffebee
    style H1 fill:#ffebee
    style A2 fill:#e8f5e8
    style K2 fill:#e8f5e8
```

## Детальная схема интеграции

```mermaid
sequenceDiagram
    participant Client as Клиент
    participant DG as docx_generator.rs
    participant DI as DrawItemZ (drawItem.rs)
    participant BA as BoundsAnalyzer
    participant DC as DimensionCalculator
    participant CS as CoordinateScaler
    participant AR as AdaptiveRenderer
    
    Note over Client,AR: Новый адаптивный процесс
    
    Client->>DG: create_docx_document_adaptive()
    DG->>DI: Создание DrawItemZ для каждого Z-уровня
    
    loop Для каждого DrawItemZ
        DI->>DI: calculate_adaptive_params()
        DI->>BA: analyze_bounds(self.data)
        BA-->>DI: CoordinateBounds
        
        DI->>DC: calculate_optimal_dimensions(bounds, config)
        DC->>DC: determine_orientation()
        DC->>DC: calculate_complexity()
        DC->>DC: calculate_target_dpi()
        DC-->>DI: OptimalDimensions
        
        DI->>CS: calculate_scaling_params(bounds, dimensions)
        CS-->>DI: ScalingParams
        
        Note over DI: Сохранение параметров в структуре
        DI->>DI: self.bounds = Some(bounds)
        DI->>DI: self.dimensions = Some(dimensions)
        DI->>DI: self.scaling = Some(scaling)
    end
    
    loop Для каждого поля (as1, as2, as3, as4)
        DG->>DI: draw_image_adaptive(field)
        DI->>AR: Создание адаптивного рендерера
        AR->>AR: Создание буфера с оптимальными размерами
        AR->>AR: Отрисовка с адаптивным масштабированием
        AR-->>DI: PNG bytes
        DI-->>DG: PNG bytes
        
        DG->>DG: Создание Pic с адаптивными размерами
        Note over DG: dimensions.docx_width_twips, dimensions.docx_height_twips
    end
    
    DG->>DG: Установка адаптивных размеров страницы
    Note over DG: dimensions.page_width_twips, dimensions.page_height_twips
    
    DG-->>Client: DOCX с адаптивными размерами
```

## Изменения в структуре DrawItemZ

```mermaid
classDiagram
    class DrawItemZ_Old {
        +data: Vec~EntityWithXlsx~
        +new() DrawItemZ
        +add_entity(entity: EntityWithXlsx)
        +draw_image_as1(field: &str) Vec~u8~
        +draw_all_images() Vec~Vec~u8~~
    }
    
    class DrawItemZ_New {
        +data: Vec~EntityWithXlsx~
        +bounds: Option~CoordinateBounds~
        +dimensions: Option~OptimalDimensions~
        +scaling: Option~ScalingParams~
        +config: A4Config
        +new() DrawItemZ
        +add_entity(entity: EntityWithXlsx)
        +calculate_adaptive_params() Result~(), String~
        +draw_image_as1(field: &str) Vec~u8~
        +draw_image_adaptive(field: &str) Vec~u8~
        +draw_all_images() Vec~Vec~u8~~
        +draw_all_images_adaptive() Vec~Vec~u8~~
    }
    
    DrawItemZ_Old --|> DrawItemZ_New : Расширение
    
    style DrawItemZ_Old fill:#ffebee
    style DrawItemZ_New fill:#e8f5e8
```

## Миграционная стратегия

```mermaid
flowchart TD
    A[Этап 1: Создание новых модулей] --> B[Этап 2: Расширение DrawItemZ]
    B --> C[Этап 3: Добавление адаптивных методов]
    C --> D[Этап 4: Обновление docx_generator]
    D --> E[Этап 5: Тестирование совместимости]
    E --> F{Все тесты пройдены?}
    F -->|Нет| G[Исправление ошибок]
    G --> E
    F -->|Да| H[Этап 6: Постепенная замена]
    H --> I[Этап 7: Удаление старого кода]
    
    subgraph "Детали этапов"
        A1["• bounds_analyzer.rs<br/>• dimension_calculator.rs<br/>• coordinate_scaler.rs<br/>• adaptive_renderer.rs<br/>• config.rs<br/>• types.rs"]
        B1["• Добавление новых полей<br/>• Сохранение обратной совместимости"]
        C1["• draw_image_adaptive()<br/>• calculate_adaptive_params()<br/>• draw_all_images_adaptive()"]
        D1["• create_docx_document_adaptive()<br/>• create_docx_for_selected_floors_adaptive()"]
        E1["• Юнит-тесты<br/>• Интеграционные тесты<br/>• Тесты производительности"]
        H1["• Флаг feature для переключения<br/>• A/B тестирование<br/>• Мониторинг производительности"]
        I1["• Удаление старых методов<br/>• Очистка неиспользуемого кода<br/>• Обновление документации"]
    end
    
    A -.-> A1
    B -.-> B1
    C -.-> C1
    D -.-> D1
    E -.-> E1
    H -.-> H1
    I -.-> I1
    
    style A fill:#e3f2fd
    style I fill:#e8f5e8
    style G fill:#fff3e0
```

## Точки интеграции в существующем коде

```mermaid
graph LR
    subgraph "docx_generator.rs"
        A[create_docx_document] --> A1[create_docx_document_adaptive]
        B[create_docx_for_selected_floors] --> B1[create_docx_for_selected_floors_adaptive]
        C[Фиксированные размеры страницы] --> C1[Адаптивные размеры страницы]
        D[Фиксированные размеры изображения] --> D1[Адаптивные размеры изображения]
    end
    
    subgraph "drawItem.rs"
        E[DrawItemZ::new] --> E1[DrawItemZ::new с конфигом]
        F[draw_image_as1] --> F1[draw_image_adaptive]
        G[draw_all_images] --> G1[draw_all_images_adaptive]
        H[Фиксированные параметры] --> H1[Адаптивные параметры]
    end
    
    subgraph "Новые модули"
        I[image_generation/mod.rs]
        J[bounds_analyzer.rs]
        K[dimension_calculator.rs]
        L[coordinate_scaler.rs]
        M[adaptive_renderer.rs]
        N[config.rs]
        O[types.rs]
    end
    
    E1 --> I
    F1 --> J
    F1 --> K
    F1 --> L
    F1 --> M
    A1 --> I
    B1 --> I
    
    style A1 fill:#e8f5e8
    style B1 fill:#e8f5e8
    style C1 fill:#e8f5e8
    style D1 fill:#e8f5e8
    style E1 fill:#e8f5e8
    style F1 fill:#e8f5e8
    style G1 fill:#e8f5e8
    style H1 fill:#e8f5e8
    style I fill:#c8e6c9
    style J fill:#c8e6c9
    style K fill:#c8e6c9
    style L fill:#c8e6c9
    style M fill:#c8e6c9
    style N fill:#c8e6c9
    style O fill:#c8e6c9
```

## Обратная совместимость

```mermaid
flowchart TD
    A[Вызов существующего API] --> B{Используется новый метод?}
    B -->|Нет| C[Вызов старого метода]
    B -->|Да| D[Проверка наличия адаптивных параметров]
    
    C --> E[Возврат результата старым способом]
    
    D --> F{Параметры вычислены?}
    F -->|Нет| G[calculate_adaptive_params()]
    F -->|Да| H[Использование существующих параметров]
    
    G --> I{Успешно?}
    I -->|Нет| J[Fallback к старому методу]
    I -->|Да| H
    
    H --> K[Вызов адаптивного метода]
    J --> C
    K --> L[Возврат адаптивного результата]
    
    style C fill:#fff3e0
    style E fill:#fff3e0
    style J fill:#fff3e0
    style K fill:#e8f5e8
    style L fill:#e8f5e8
```

## Конфигурация переключения

```mermaid
graph TB
    subgraph "Конфигурационные флаги"
        A[use_adaptive_rendering: bool]
        B[fallback_to_fixed: bool]
        C[enable_bounds_analysis: bool]
        D[enable_dimension_optimization: bool]
        E[enable_coordinate_scaling: bool]
    end
    
    subgraph "Логика переключения"
        F{use_adaptive_rendering?}
        F -->|true| G[Адаптивный путь]
        F -->|false| H[Фиксированный путь]
        
        G --> I{Адаптивный метод успешен?}
        I -->|true| J[Возврат адаптивного результата]
        I -->|false| K{fallback_to_fixed?}
        K -->|true| H
        K -->|false| L[Возврат ошибки]
    end
    
    A --> F
    B --> K
    
    style G fill:#e8f5e8
    style H fill:#fff3e0
    style J fill:#e8f5e8
    style L fill:#ffebee
```

## Тестовая стратегия

```mermaid
quadrantChart
    title Стратегия тестирования
    x-axis Простые тесты --> Сложные тесты
    y-axis Быстрые тесты --> Медленные тесты
    
    quadrant-1 Сложные и медленные
    quadrant-2 Простые и медленные
    quadrant-3 Простые и быстрые
    quadrant-4 Сложные и быстрые
    
    Юнит-тесты модулей: [0.2, 0.8]
    Интеграционные тесты: [0.6, 0.6]
    Тесты производительности: [0.8, 0.2]
    Тесты совместимости: [0.4, 0.4]
    E2E тесты: [0.9, 0.1]
    Регрессионные тесты: [0.7, 0.3]
```

## Мониторинг и метрики

```mermaid
graph TB
    subgraph "Метрики производительности"
        A[Время анализа границ]
        B[Время вычисления размеров]
        C[Время масштабирования]
        D[Время отрисовки]
        E[Общее время обработки]
        F[Использование памяти]
    end
    
    subgraph "Метрики качества"
        G[Размер файла DOCX]
        H[Качество изображения]
        I[Читаемость текста]
        J[Соответствие A4 формату]
    end
    
    subgraph "Метрики надежности"
        K[Частота ошибок]
        L[Частота fallback]
        M[Успешность обработки]
        N[Стабильность результатов]
    end
    
    subgraph "Система мониторинга"
        O[Логирование]
        P[Метрики Prometheus]
        Q[Дашборды Grafana]
        R[Алерты]
    end
    
    A --> O
    B --> O
    C --> O
    D --> O
    E --> P
    F --> P
    G --> P
    H --> P
    I --> P
    J --> P
    K --> R
    L --> R
    M --> Q
    N --> Q
    
    style O fill:#e3f2fd
    style P fill:#e8f5e8
    style Q fill:#fff3e0
    style R fill:#ffebee
```

## Заключение

Данная диаграмма интеграции показывает:

1. **Плавный переход** от текущей системы к новой адаптивной
2. **Обратную совместимость** для существующего кода
3. **Поэтапную миграцию** с возможностью отката
4. **Комплексное тестирование** на всех уровнях
5. **Мониторинг и контроль** качества интеграции

Это обеспечивает безопасное внедрение новой функциональности без нарушения работы существующей системы.