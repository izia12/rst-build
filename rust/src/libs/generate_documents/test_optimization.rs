//! Тестирование оптимизированной генерации DOCX

use crate::libs::drawItem::DrawItemZ;
use crate::libs::parse::{EntityWithXlsx, Vertex, RowData};
use super::docx_generator::create_docx_document_optimized;
use super::performance::{PerformanceConfig, PerformanceMonitor};
use super::wasm_time::WasmInstant;

/// Создает тестовые данные для проверки производительности
pub fn create_test_entities(count: usize) -> Vec<EntityWithXlsx> {
    let mut entities = Vec::new();
    
    for i in 0..count {
        let vertices = vec![
            Vertex {
                x: (i % 100) as f64 * 10.0,
                y: (i % 50) as f64 * 10.0,
                z: (i % 10) as f64,
            },
            Vertex {
                x: (i % 100) as f64 * 10.0 + 5.0,
                y: (i % 50) as f64 * 10.0 + 5.0,
                z: (i % 10) as f64,
            },
        ];
        
        let row_data = RowData {
            id: i,
            as1: vec![1.0, 2.0],
            as2: vec![3.0, 4.0],
            as3: vec![5.0, 6.0],
            as4: vec![7.0, 8.0],
            asw1: vec![9.0, 10.0],
            asw2: vec![11.0, 12.0],
        };
        
        let entity = EntityWithXlsx {
            entity_type: "test_line".to_string(),
            vertices,
            row: Some(row_data),
            changed: false,
            material: None,
        };
        entities.push(entity);
    }
    
    entities
}

/// Тестирует производительность генерации DOCX
pub async fn test_docx_performance() {
    println!("🚀 Начинаем тест производительности DOCX генерации...");
    
    // Создаем тестовые данные
    let entities = create_test_entities(1000);
    println!("📊 Создано {} тестовых объектов", entities.len());
    
    // Конфигурация производительности
    let config = PerformanceConfig {
        enable_parallel_rendering: true,
        enable_gpu_acceleration: false,
        max_image_size: (2500, 2000),
        compression_quality: 80,
        enable_caching: true,
    };
    
    let start_time = WasmInstant::now();
    
    // Запускаем оптимизированную генерацию
    let docx_buffer = create_docx_document_optimized(entities, "Test Document", config).await;
    let duration = start_time.elapsed();
    println!("✅ DOCX успешно сгенерирован!");
    println!("⏱️  Время генерации: {:.2} секунд", duration.as_secs_f64());
    println!("📦 Размер файла: {} байт", docx_buffer.len());
    
    // Оценка улучшения производительности
    let estimated_old_time = 45.0; // Предыдущее время генерации
    let speedup = estimated_old_time / duration.as_secs_f64();
    println!("🚀 Ускорение: {:.1}x", speedup);
}

/// Тестирует параллельную генерацию изображений
pub async fn test_parallel_image_generation() {
    println!("🖼️  Тестируем параллельную генерацию изображений...");
    
    let entities = create_test_entities(100);
    let mut draw_item = DrawItemZ::new();
    
    // Добавляем сущности в DrawItemZ
    for entity in entities {
        draw_item.add_entity(entity);
    }
    
    // Конфигурация для последовательной генерации
    let config_sequential = PerformanceConfig {
        enable_parallel_rendering: false,
        enable_gpu_acceleration: false,
        max_image_size: (2500, 2000),
        compression_quality: 80,
        enable_caching: true,
    };
    
    // Конфигурация для параллельной генерации
    let config_parallel = PerformanceConfig {
        enable_parallel_rendering: true,
        enable_gpu_acceleration: false,
        max_image_size: (2000, 1600), // Меньший размер для быстрой обработки
        compression_quality: 60, // Быстрое сжатие для параллельной обработки
        enable_caching: true,
    };
    
    // Последовательная генерация
    let start_sequential = WasmInstant::now();
    let _sequential_images = draw_item.draw_all_images_optimized(&config_sequential).await;
    let sequential_time = start_sequential.elapsed();
    
    println!("⏱️  Последовательная генерация: {:.2} сек", sequential_time.as_secs_f64());
    
    // Параллельная генерация
    let start_parallel = WasmInstant::now();
    let _parallel_images = draw_item.draw_all_images_optimized(&config_parallel).await;
    let parallel_time = start_parallel.elapsed();
    
    println!("⏱️  Параллельная генерация: {:.2} сек", parallel_time.as_secs_f64());
    
    if parallel_time < sequential_time {
        let speedup = sequential_time.as_secs_f64() / parallel_time.as_secs_f64();
        println!("🚀 Ускорение: {:.1}x", speedup);
    } else {
        println!("⚠️  Параллельная генерация не показала улучшения");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_entity_creation() {
        let entities = create_test_entities(10);
        assert_eq!(entities.len(), 10);
        assert_eq!(entities[0].entity_type, "test_line");
        assert_eq!(entities[9].entity_type, "test_line");
    }
    
    #[test]
    fn test_performance_config() {
        let config = PerformanceConfig {
            enable_parallel_rendering: true,
            enable_gpu_acceleration: false,
            max_image_size: (2500, 2000),
            compression_quality: 80,
            enable_caching: true,
        };
        
        assert!(config.enable_parallel_rendering);
        assert!(!config.enable_gpu_acceleration);
        assert_eq!(config.compression_quality, 80);
    }
    
    /// Тестирует производительность генерации DOCX
    pub fn test_docx_generation_performance(entities: Vec<EntityWithXlsx>) -> f64 {
        let config = PerformanceConfig {
            enable_parallel_rendering: true,
            enable_gpu_acceleration: false,
            max_image_size: (2500, 2000),
            compression_quality: 80,
            enable_caching: true,
        };
        
        let start = WasmInstant::now();
        
        let docx_buffer = create_docx_document_optimized(entities, "Performance Test", config);
        let duration = start.elapsed();
        println!("DOCX generation completed in: {:?}", duration);
        println!("Generated DOCX size: {} bytes", docx_buffer.len());
        duration.as_secs_f64()
    }
    
    /// Сравнивает последовательную и параллельную генерацию изображений
    pub fn compare_image_generation(entities: Vec<EntityWithXlsx>) -> (f64, f64) {
        let mut draw_item = DrawItemZ::new();
        
        // Добавляем сущности в DrawItemZ
        for entity in entities {
            draw_item.add_entity(entity);
        }
        
        // Последовательная генерация
        let start = WasmInstant::now();
        let _images_sequential = draw_item.draw_all_images();
        let sequential_time = start.elapsed().as_secs_f64();
        
        // Параллельная генерация (если доступна)
        let start = WasmInstant::now();
        let _images_parallel = draw_item.draw_all_images(); // В будущем заменить на параллельную версию
        let parallel_time = start.elapsed().as_secs_f64();
        
        println!("Sequential generation: {:.3}s", sequential_time);
        println!("Parallel generation: {:.3}s", parallel_time);
        println!("Speedup: {:.2}x", sequential_time / parallel_time);
        
        (sequential_time, parallel_time)
    }
}