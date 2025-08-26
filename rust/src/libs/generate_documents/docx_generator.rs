use std::{ collections::HashMap, io::Cursor};
use docx_rs::{Docx, Paragraph, Pic, Run};
use ordered_float::OrderedFloat;
use crate::libs::drawItem::DrawItemZ;
use crate::libs::parse::EntityWithXlsx;
use crate::libs::generate_documents::performance::{
    PerformanceMonitor, PerformanceConfig, 
    log_performance_metrics, get_optimization_recommendations
};

#[derive(serde::Deserialize, Debug)]
pub struct SelectedCombination {
    pub floor_level: String,
    pub function_name: String,
    pub as_target_value: f32,
    pub combination: CombinationItem,
}

#[derive(serde::Deserialize, Debug)]
pub struct CombinationItem {
    pub main_diameter: u32,
    pub additional_diameter: u32,
    pub total_area: f32,
    pub deviation: f32,
    pub result_scale: Option<String>,
}

/// НОВАЯ ФУНКЦИЯ: Создает DOCX документ с цветовой палитрой
pub async fn create_docx_with_color_palette(
    entities: Vec<EntityWithXlsx>,
    selected_floors: Vec<f32>,
    selected_combinations: Vec<SelectedCombination>,
    title: &str
) -> Vec<u8> {
    web_sys::console::log_1(&format!("🎨 Creating DOCX with color palette for {} floors", selected_floors.len()).into());
    
    // Пока используем стандартную генерацию, позже добавим цветовую палитру
    create_docx_for_selected_floors(
        entities,
        selected_floors,
        title
    ).await
}

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
    
    // ИСПОЛЬЗУЕМ ЕДИНЫЕ КОНСТАНТЫ - ПРАВИЛЬНЫЕ ПРОПОРЦИИ A4!
    use crate::libs::drawItem::{DOCX_IMAGE_WIDTH_TWIPS, DOCX_IMAGE_HEIGHT_TWIPS, DOCX_PAGE_WIDTH_TWIPS, DOCX_PAGE_HEIGHT_TWIPS};
    
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
                .page_size(DOCX_PAGE_WIDTH_TWIPS, DOCX_PAGE_HEIGHT_TWIPS)
                .page_orient(docx_rs::PageOrientationType::Landscape)
                .add_paragraph(
                    Paragraph::new().add_run(
                        Run::new().add_image(
                            Pic::new(img.as_slice())
                                .size(DOCX_IMAGE_WIDTH_TWIPS, DOCX_IMAGE_HEIGHT_TWIPS)
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
    
    // ИСПОЛЬЗУЕМ ЕДИНЫЕ КОНСТАНТЫ - ПРАВИЛЬНЫЕ ПРОПОРЦИИ A4!
    use crate::libs::drawItem::{DOCX_IMAGE_WIDTH_TWIPS, DOCX_IMAGE_HEIGHT_TWIPS, DOCX_PAGE_WIDTH_TWIPS, DOCX_PAGE_HEIGHT_TWIPS};
    
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
                .page_size(DOCX_PAGE_WIDTH_TWIPS, DOCX_PAGE_HEIGHT_TWIPS)
                .page_orient(docx_rs::PageOrientationType::Landscape)
                .add_paragraph(
                    Paragraph::new().add_run(
                        Run::new().add_image(
                            Pic::new(img.as_slice())
                                .size(DOCX_IMAGE_WIDTH_TWIPS, DOCX_IMAGE_HEIGHT_TWIPS)
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
    
    // ИСПОЛЬЗУЕМ ЕДИНЫЕ КОНСТАНТЫ - ПРАВИЛЬНЫЕ ПРОПОРЦИИ A4!
    use crate::libs::drawItem::{DOCX_IMAGE_WIDTH_EMU, DOCX_IMAGE_HEIGHT_EMU, DOCX_PAGE_WIDTH_TWIPS, DOCX_PAGE_HEIGHT_TWIPS};
    
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
                .size(40);
            doc = doc.add_paragraph(Paragraph::new().add_run(run));
            
            // Добавляем изображения в документ
            for img in imgs.iter() {
                doc = doc
                    .page_size(DOCX_PAGE_WIDTH_TWIPS, DOCX_PAGE_HEIGHT_TWIPS)
                    .page_orient(docx_rs::PageOrientationType::Landscape)
                    .add_paragraph(
                        Paragraph::new().add_run(
                            Run::new().add_image(
                                Pic::new(img.as_slice())
                                    .size(DOCX_IMAGE_WIDTH_EMU, DOCX_IMAGE_HEIGHT_EMU)
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
pub fn sort_by_z(data1: Vec<EntityWithXlsx>) -> HashMap<OrderedFloat<f32>, DrawItemZ> {
    let mut map: HashMap<OrderedFloat<f32>, DrawItemZ> = HashMap::new();
    
    for item in data1.into_iter() {
        let z0 = item.vertices[0].z;
        if item.vertices.iter().all(|v| v.z == z0) {
            let z = OrderedFloat(z0 as f32);
            map.entry(z)
                .or_insert_with(|| DrawItemZ { data: Vec::new() })
                .data.push(item);
        }
    }
    map
}