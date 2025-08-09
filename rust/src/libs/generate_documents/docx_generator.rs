use std::{cell::RefCell, collections::HashMap, io::Cursor};
use docx_rs::{Docx, Paragraph, Pic, Run};
use ordered_float::OrderedFloat;
use crate::libs::drawItem::DrawItemZ;
use crate::libs::parse::EntityWithXlsx;
use crate::libs::generate_documents::performance::{
    PerformanceMonitor, PerformanceConfig, PerformanceMetrics,
    log_performance_metrics, get_optimization_recommendations
};

/// Создает DOCX документ для всех этажей с мониторингом производительности
pub async fn create_docx_document(
    entities: Vec<EntityWithXlsx>,
    title: &str
) -> Vec<u8> {
    create_docx_document_optimized(entities, title, PerformanceConfig::default()).await
}

/// Оптимизированная версия создания DOCX документа с настройками производительности
pub async fn create_docx_document_optimized(
    entities: Vec<EntityWithXlsx>,
    title: &str,
    config: PerformanceConfig
) -> Vec<u8> {
    let mut monitor = PerformanceMonitor::new(config);
    let elements_count = entities.len();
    
    web_sys::console::log_1(&format!("🚀 Starting DOCX generation for {} elements", elements_count).into());
    
    let mut doc = Docx::new().add_paragraph(
        Paragraph::new().add_run(
            Run::new().add_text(title)
        )
    );
    
    let hash = sort_by_z(entities);
    let floors_count = hash.len();
    
    // Устанавливаем размеры страницы в твипах (1/20 точки)
    // A4 landscape: 297mm x 210mm = 11.69" x 8.27" = 16838 x 11906 твипов
    let page_width = 140;  // A4 landscape width
    let page_height = 105; // A4 landscape height
    
    // Размеры изображения должны быть равны размерам страницы с небольшими отступами
    let img_width = (page_width * 290 * 90 / 100) as u32; // 90% от размера страницы
    let img_height = (page_height * 280 * 90 / 100) as u32; // 90% от размера страницы
    
    monitor.start_image_generation();
    let mut total_images = 0;
    
    for (floor_index, (key, item_z)) in hash.iter().enumerate() {
        web_sys::console::log_1(&format!("📊 Processing floor {} of {} (height: {})", 
            floor_index + 1, floors_count, key.to_string()).into());
        
        // Используем GPU-оптимизированную батчевую генерацию изображений
        let images = item_z.draw_all_images_gpu_batch(monitor.get_config()).await;
        
        total_images += images.len();
        
        // Добавляем заголовок этажа
        let run = Run::new()
            .add_text(format!("Высота {}", &key.to_string()))
            .bold()
            .size(22);
        doc = doc.add_paragraph(Paragraph::new().add_run(run));
        
        // Добавляем изображения в документ
        for (img_index, img) in images.iter().enumerate() {
            web_sys::console::log_1(&format!("🖼️ Adding image {} of {} for floor {}", 
                img_index + 1, images.len(), key.to_string()).into());
                
            doc = doc
                .page_size(page_width*290, page_height*280)
                .page_orient(docx_rs::PageOrientationType::Landscape)
                .add_paragraph(
                    Paragraph::new().add_run(
                        Run::new().add_image(
                            Pic::new(img)
                                .size(img_width, img_height)
                        )
                    )
                );
        }
    }
    
    let image_time = monitor.end_image_generation();
    monitor.start_docx_creation();
    
    // Создаем буфер и записываем документ
    let mut buffer = Cursor::new(Vec::new());
    match doc.build().pack(&mut buffer) {
        Ok(_) => web_sys::console::log_1(&"✅ DOCX document created successfully".into()),
        Err(e) => web_sys::console::log_1(&format!("❌ Error creating document: {}", e).into()),
    }
    
    let docx_time = monitor.end_docx_creation();
    
    // Логируем метрики производительности
    let metrics = monitor.finish(image_time, docx_time, elements_count, total_images);
    log_performance_metrics(&metrics);
    
    // Выводим рекомендации по оптимизации
    let recommendations = get_optimization_recommendations(&metrics);
    for rec in recommendations {
        web_sys::console::log_1(&rec.into());
    }
    
    buffer.into_inner()
}

/// Создает DOCX документ для всех этажей (старая версия для совместимости)
pub async fn create_docx_document_legacy(
    entities: Vec<EntityWithXlsx>,
    title: &str
) -> Vec<u8> {
    let mut doc = Docx::new().add_paragraph(
        Paragraph::new().add_run(
            Run::new().add_text(title)
        )
    );
    
    let hash = sort_by_z(entities);
    
    // Устанавливаем размеры страницы в твипах (1/20 точки)
    // A4 landscape: 297mm x 210mm = 11.69" x 8.27" = 16838 x 11906 твипов
    let page_width = 140;  // A4 landscape width
    let page_height = 105; // A4 landscape height
    
    // Размеры изображения в твипах (максимально увеличенные)
    let img_width = 20000000;   // Очень большой размер для видимости
    let img_height = 15000000;  // Очень большой размер для видимости
    
    let monitor = PerformanceMonitor::new(PerformanceConfig::default());
    
    for (key, item_z) in hash {
        let imgs = item_z.draw_all_images_gpu_batch(monitor.get_config()).await;
        
        // Добавляем заголовок этажа
        let run = Run::new()
            .add_text(format!("Высота {}", &key.to_string()))
            .bold()
            .size(22);
        doc = doc.add_paragraph(Paragraph::new().add_run(run));
        
        // Добавляем изображения в документ
        for img in imgs.iter() {
            doc = doc
                .page_size(page_width*290, page_height*280)
                .page_orient(docx_rs::PageOrientationType::Landscape)
                .add_paragraph(
                    Paragraph::new().add_run(
                        Run::new().add_image(
                            Pic::new(img.as_slice())
                                .size(img_width, img_height)
                        )
                    )
                );
        }
    }
    
    // Создаем буфер и записываем документ
    let mut buffer = Cursor::new(Vec::new());
    match doc.build().pack(&mut buffer) {
        Ok(_) => (),
        Err(e) => println!("Ошибка создания документа: {}", e),
    }
    
    buffer.into_inner()
}

/// Создает DOCX документ для выбранных этажей
pub async fn create_docx_for_selected_floors(
    entities: Vec<EntityWithXlsx>,
    selected_floors: Vec<f32>,
    title: &str
) -> Vec<u8> {
    let mut doc = Docx::new().add_paragraph(
        Paragraph::new().add_run(
            Run::new().add_text(title)
        )
    );
    
    // Группируем по Z-координате (этажам)
    let hash = sort_by_z(entities);
    
    // Устанавливаем размеры страницы в твипах (1/20 точки)
    // A4 landscape: 297mm x 210mm = 11.69" x 8.27" = 16838 x 11906 твипов
    let page_width = 16838;  // A4 landscape width
    let page_height = 11906; // A4 landscape height
    
    // Размеры изображения в твипах (большие размеры для лучшей видимости)
    let img_width = 30000;   // Большой размер для максимальной видимости
    let img_height = 22000;  // Большой размер для максимальной видимости
    
    let monitor = PerformanceMonitor::new(PerformanceConfig::default());
    
    // Обрабатываем только выбранные этажи
    for selected_floor in selected_floors {
        let z_key = OrderedFloat(selected_floor);
        
        if let Some(item_z) = hash.get(&z_key) {
            // Генерируем изображения для этого этажа
            let imgs = item_z.draw_all_images_gpu_batch(monitor.get_config()).await;
            
            // Добавляем заголовок этажа
            let run = Run::new()
                .add_text(format!("Высота {}", selected_floor))
                .bold()
                .size(22);
            doc = doc.add_paragraph(Paragraph::new().add_run(run));
            
            // Добавляем изображения в документ
            for img in imgs.iter() {
                doc = doc
                    .page_size(page_width, page_height)
                    .page_orient(docx_rs::PageOrientationType::Landscape)
                    .add_paragraph(
                        Paragraph::new().add_run(
                            Run::new().add_image(
                                Pic::new(img.as_slice())
                                    .size(img_width, img_height)
                            )
                        )
                    );
            }
        }
    }
    
    // Создаем буфер и записываем документ
    let mut buffer = Cursor::new(Vec::new());
    match doc.build().pack(&mut buffer) {
        Ok(_) => (),
        Err(e) => println!("Ошибка создания документа: {}", e),
    }
    
    buffer.into_inner()
}

/// Группирует сущности по Z-координате (этажам)
fn sort_by_z(data1: Vec<EntityWithXlsx>) -> HashMap<OrderedFloat<f32>, DrawItemZ> {
    let mut map: HashMap<OrderedFloat<f32>, DrawItemZ> = HashMap::new();
	let mut _map1:HashMap<String, String> = HashMap::new();
	// map1.
    for item in data1.into_iter() {
		let z0 = item.vertices[0].z;
		if item.vertices.iter().all(|v| v.z == z0) {
			let z = OrderedFloat(z0 as f32);
			map.entry(z)
				.or_insert_with(|| DrawItemZ { data: Vec::new() })
				.data.push(item); // Здесь теперь item перемещается, а не заимствуется
		}
	}
    map
}