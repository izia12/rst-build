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

/// ЕДИНСТВЕННАЯ ФУНКЦИЯ СОЗДАНИЯ DOCX - ПРОСТАЯ И БЫСТРАЯ
/// Автоматически применяет цвета если есть selected_combinations
pub async fn create_docx(
    entities: Vec<EntityWithXlsx>,
    selected_floors: Option<Vec<f32>>,
    selected_combinations: Option<Vec<SelectedCombination>>,
    title: &str
) -> Vec<u8> {
    // === STEP 3: DOCX GENERATOR ===
    web_sys::console::log_1(&format!("[STEP 3] create_docx() called with title: '{}'", title).into());
    web_sys::console::log_1(&format!("[STEP 3] selected_floors: {:?}", selected_floors).into());
    web_sys::console::log_1(&format!("[STEP 3] selected_combinations: {:?}", selected_combinations.as_ref().map(|c| c.len())).into());
    
    match selected_floors {
        Some(floors) => {
              web_sys::console::log_1(&format!("[STEP 3] Calling create_docx_for_selected_floors() with {} floors", floors.len()).into());
              web_sys::console::log_1(&"[STEP 3] ✅ FIXED: Now passing combinations to create_docx_for_selected_floors!".into());
              create_docx_for_selected_floors(entities, floors, selected_combinations, title).await
          },
         None => {
             web_sys::console::log_1(&"[STEP 3] Calling create_docx_document() - no floors specified".into());
             create_docx_document(entities, title).await
         }
    }
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
    // 🔍 PERFORMANCE: Засекаем общее время создания DOCX
    let docx_start = web_sys::window().unwrap().performance().unwrap().now();
    
    let mut monitor = PerformanceMonitor::new(config);
    let elements_count = entities.len();
    
    web_sys::console::log_1(&format!(
        "🚀 [DOCX-CREATE] Starting DOCX creation for {} elements", 
        elements_count
    ).into());
    
    let mut doc = Docx::new()
        // Устанавливаем минимальные отступы страницы для максимального использования пространства
        .page_margin(docx_rs::PageMargin {
            top: 200,    // 0.2 см (минимальный отступ)
            right: 200,  // 0.2 см
            bottom: 200, // 0.2 см  
            left: 200,   // 0.2 см
            header: 0,   // Без отступа для заголовка
            footer: 0,   // Без отступа для подвала
            gutter: 0,   // Без переплета
        })
        .add_paragraph(
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
        let images = item_z.draw_all_images_with_colors(None).await;
        
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
        Ok(_) => web_sys::console::log_1(&"✅ [DOCX-CREATE] DOCX document built successfully".into()),
        Err(e) => web_sys::console::log_1(&format!("❌ [DOCX-CREATE] Error creating document: {}", e).into()),
    }
    
    let docx_time = monitor.end_docx_creation();
    
    // 🔍 PERFORMANCE: Финальные метрики DOCX создания
    let total_docx_time = web_sys::window().unwrap().performance().unwrap().now() - docx_start;
    let buffer_size = buffer.get_ref().len();
    
    web_sys::console::log_1(&format!(
        "✅ [DOCX-CREATE] DOCX creation completed: Total={:.1}ms, Images={:.1}ms, Assembly={:.1}ms, Size={:.1}MB", 
        total_docx_time, image_time.as_secs_f64() * 1000.0, docx_time.as_secs_f64() * 1000.0, 
        buffer_size as f64 / 1024.0 / 1024.0
    ).into());
    
    // Логируем метрики производительности (упрощенно)
    let metrics = monitor.finish(image_time, docx_time, elements_count, total_images);
    web_sys::console::log_1(&format!(
        "📊 [PERFORMANCE] Elements: {}, Images: {}, Avg per image: {:.1}ms", 
        metrics.elements_count, metrics.images_count, 
        metrics.avg_time_per_image.as_secs_f64() * 1000.0
    ).into());
    
    buffer.into_inner()
}

/// Создает DOCX документ для всех этажей (старая версия для совместимости)
pub async fn create_docx_document_legacy(
    entities: Vec<EntityWithXlsx>,
    title: &str
) -> Vec<u8> {
    let mut doc = Docx::new()
        // Устанавливаем минимальные отступы страницы для максимального использования пространства
        .page_margin(docx_rs::PageMargin {
            top: 200,    // 0.2 см (минимальный отступ)
            right: 200,  // 0.2 см
            bottom: 200, // 0.2 см  
            left: 200,   // 0.2 см
            header: 0,   // Без отступа для заголовка
            footer: 0,   // Без отступа для подвала
            gutter: 0,   // Без переплета
        })
        .add_paragraph(
            Paragraph::new().add_run(
                Run::new().add_text(title)
            )
        );
    
    let hash = sort_by_z(entities);
    
    // ИСПОЛЬЗУЕМ ЕДИНЫЕ КОНСТАНТЫ - ПРАВИЛЬНЫЕ ПРОПОРЦИИ A4!
    use crate::libs::drawItem::{DOCX_IMAGE_WIDTH_TWIPS, DOCX_IMAGE_HEIGHT_TWIPS, DOCX_PAGE_WIDTH_TWIPS, DOCX_PAGE_HEIGHT_TWIPS};
    
    let monitor = PerformanceMonitor::new(PerformanceConfig::default());
    
    for (key, item_z) in hash {
        let imgs = item_z.draw_all_images_with_colors(None).await;
        
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
    selected_combinations: Option<Vec<SelectedCombination>>,
    title: &str
) -> Vec<u8> {
    // === STEP 4: SELECTED FLOORS PROCESSING ===
    web_sys::console::log_1(&format!("[STEP 4] create_docx_for_selected_floors() called with {} entities, {} floors", entities.len(), selected_floors.len()).into());
    web_sys::console::log_1(&format!("[STEP 4] selected_floors: {:?}", selected_floors).into());
    web_sys::console::log_1(&format!("[STEP 4] selected_combinations: {:?}", selected_combinations.as_ref().map(|c| c.len())).into());
    	
    // 🔍 PERFORMANCE: Начало общего таймера
    let total_start = web_sys::window().unwrap().performance().unwrap().now();
    	
    // 🔍 PERFORMANCE: Начало создания DOCX структуры
    let docx_init_start = web_sys::window().unwrap().performance().unwrap().now();
    let mut doc = Docx::new()
        // Устанавливаем минимальные отступы страницы для максимального использования пространства
        .page_margin(docx_rs::PageMargin {
            top: 200,    // 0.2 см (минимальный отступ)
            right: 200,  // 0.2 см
            bottom: 200, // 0.2 см  
            left: 200,   // 0.2 см
            header: 0,   // Без отступа для заголовка
            footer: 0,   // Без отступа для подвала
            gutter: 0,   // Без переплета
        })
        .add_paragraph(
            Paragraph::new().add_run(
                Run::new().add_text(title)
            )
        );
    
    // 🔍 PERFORMANCE: Время инициализации DOCX
    let docx_init_time = web_sys::window().unwrap().performance().unwrap().now() - docx_init_start;
    web_sys::console::log_1(&format!("📄 [DOCX-INIT] DOCX structure created: {:.1}ms", docx_init_time).into());
    
    // ✨ ОПТИМИЗАЦИЯ: Настраиваем page size и orientation ОДИН раз для всего документа
    doc = doc
        .page_size(DOCX_PAGE_WIDTH_TWIPS, DOCX_PAGE_HEIGHT_TWIPS)
        .page_orient(docx_rs::PageOrientationType::Landscape);
    
    web_sys::console::log_1(&"✨ [DOCX-OPT] Global page settings applied".into());
    
    // 🔍 PERFORMANCE: Начало сортировки по Z
    let sort_start = web_sys::window().unwrap().performance().unwrap().now();
    
    // Группируем по Z-координате (этажам)
    let hash = sort_by_z(entities);
    
    // 🔍 PERFORMANCE: Время сортировки
    let sort_time = web_sys::window().unwrap().performance().unwrap().now() - sort_start;
    web_sys::console::log_1(&format!("📋 [SORT-Z] Entities sorted by Z-coordinate: {:.1}ms ({} floors)", sort_time, hash.len()).into());
    
    // ИСПОЛЬЗУЕМ ЕДИНЫЕ КОНСТАНТЫ - ПРАВИЛЬНЫЕ ПРОПОРЦИИ A4!
    use crate::libs::drawItem::{DOCX_IMAGE_WIDTH_EMU, DOCX_IMAGE_HEIGHT_EMU, DOCX_PAGE_WIDTH_TWIPS, DOCX_PAGE_HEIGHT_TWIPS};
    
    let monitor = PerformanceMonitor::new(PerformanceConfig::default());
    
    // === STEP 5: CREATING COMBINATION MAP ===
    let mut combination_map = std::collections::HashMap::new();
    if let Some(combinations) = &selected_combinations {
         web_sys::console::log_1(&format!("[STEP 5] Creating combination map from {} combinations", combinations.len()).into());
         for (idx, combo) in combinations.iter().enumerate() {
             web_sys::console::log_1(&format!("[DEBUG] Combo {}: floor_level='{}', function_name='{}'", idx, combo.floor_level, combo.function_name).into());
             web_sys::console::log_1(&format!("[DEBUG] Combo {}: result_scale={:?}", idx, combo.combination.result_scale).into());
             
             let key = format!("{}-{}", combo.floor_level, combo.function_name);
             if let Some(ref result_scale) = combo.combination.result_scale {
                 combination_map.insert(key.clone(), result_scale.as_str());
                 web_sys::console::log_1(&format!("[STEP 5] ✅ Added mapping: {} -> {}", key, result_scale).into());
             } else {
                 web_sys::console::log_1(&format!("[STEP 5] ❌ No result_scale for combo {}: {}", idx, key).into());
             }
         }
         web_sys::console::log_1(&format!("[STEP 5] Final combination_map size: {}", combination_map.len()).into());
     } else {
         web_sys::console::log_1(&"[STEP 5] No combinations provided, using default colors".into());
     }
    
    // === STEP 6: PROCESSING EACH FLOOR ===
    web_sys::console::log_1(&format!("[STEP 6] Processing {} selected floors", selected_floors.len()).into());
    
    // 🔍 PERFORMANCE: Начало обработки всех этажей
    let floors_start = web_sys::window().unwrap().performance().unwrap().now();
    
    // Обрабатываем только выбранные этажи
    for (floor_idx, selected_floor) in selected_floors.iter().enumerate() {
        // 🔍 PERFORMANCE: Начало обработки одного этажа + точка между этажами
        let floor_start = web_sys::window().unwrap().performance().unwrap().now();
        if floor_idx > 0 {
            web_sys::console::log_1(&format!("⏱️ [GAP] Time since last floor ended: Starting floor {} processing", selected_floor).into());
        }
        
        let z_key = OrderedFloat(*selected_floor);
        
        web_sys::console::log_1(&format!("[STEP 6] Processing floor {}/{}: {}", floor_idx + 1, selected_floors.len(), selected_floor).into());
         
         if let Some(item_z) = hash.get(&z_key) {
             web_sys::console::log_1(&format!("[STEP 6] Found {} entities for floor {}", item_z.data.len(), selected_floor).into());
             
             // === STEP 7: CREATING RESULT_SCALES FOR THIS FLOOR ===
              let result_scales: Vec<Option<&str>> = ["as1", "as2", "as3", "as4"]
                  .iter()
                  .map(|field| {
                      let key = format!("{}-{}", selected_floor, field);
                      let result = combination_map.get(&key).copied();
                      web_sys::console::log_1(&format!("[DEBUG] Looking for key '{}': {:?}", key, result).into());
                      result
                  })
                  .collect();
              
              web_sys::console::log_1(&format!("[STEP 7] Created result_scales for floor {}: {:?}", selected_floor, result_scales).into());
              web_sys::console::log_1(&format!("[DEBUG] Available keys in map: {:?}", combination_map.keys().collect::<Vec<_>>()).into());
             
             // === STEP 8: CALLING DRAW FUNCTIONS ===
             web_sys::console::log_1(&format!("[STEP 8] Calling draw_all_images_with_colors(Some(result_scales)) for floor {}", selected_floor).into());
             web_sys::console::log_1(&"[STEP 8] ✅ FIXED: Now passing result_scales instead of None!".into());
             
             // 🔍 PERFORMANCE: Начало генерации изображений для этажа
             let images_start = web_sys::window().unwrap().performance().unwrap().now();
             
             let imgs = item_z.draw_all_images_with_colors_and_floor(Some(&result_scales), *selected_floor).await;
             
             // 🔍 PERFORMANCE: Время генерации изображений
             let images_time = web_sys::window().unwrap().performance().unwrap().now() - images_start;
             web_sys::console::log_1(&format!("🖼️ [FLOOR-IMAGES] Floor {} images generated: {:.1}ms ({} images)", 
                 selected_floor, images_time, imgs.len()).into());
             
             web_sys::console::log_1(&format!("[STEP 8] draw_all_images_with_colors() returned {} images", imgs.len()).into());
            
            // 🔍 PERFORMANCE: Начало добавления в DOCX
            let docx_add_start = web_sys::window().unwrap().performance().unwrap().now();
            
            // ✨ ОПТИМИЗАЦИЯ: Создаем заголовок этажа
            let floor_title_paragraph = Paragraph::new().add_run(
                Run::new()
                    .add_text(format!("Высота {}", selected_floor))
                    .size(40)
            );
            
            // ✨ ОПТИМИЗАЦИЯ: Предварительно создаем все параграфы с изображениями
            let prep_start = web_sys::window().unwrap().performance().unwrap().now();
            let mut image_paragraphs = Vec::with_capacity(imgs.len());
            for img in imgs.iter() {
                let image_paragraph = Paragraph::new().add_run(
                    Run::new().add_image(
                        Pic::new(img.as_slice())
                            .size(DOCX_IMAGE_WIDTH_EMU, DOCX_IMAGE_HEIGHT_EMU)
                    )
                );
                image_paragraphs.push(image_paragraph);
            }
            let prep_time = web_sys::window().unwrap().performance().unwrap().now() - prep_start;
            
            web_sys::console::log_1(&format!("🚀 [DOCX-PREP] Prepared {} paragraphs for floor {} in {:.1}ms", 
                image_paragraphs.len(), selected_floor, prep_time).into());
            
            // ✨ ОПТИМИЗАЦИЯ: Добавляем заголовок этажа (без повторных настроек)
            doc = doc.add_paragraph(floor_title_paragraph);
            
            // ✨ ОПТИМИЗАЦИЯ: Batch-добавление всех изображений
            let batch_start = web_sys::window().unwrap().performance().unwrap().now();
            for paragraph in image_paragraphs {
                doc = doc.add_paragraph(paragraph);
            }
            let batch_time = web_sys::window().unwrap().performance().unwrap().now() - batch_start;
            
            web_sys::console::log_1(&format!("✨ [DOCX-BATCH] Added {} images to doc in {:.1}ms", 
                imgs.len(), batch_time).into());
            
            // 🔍 PERFORMANCE: Время добавления в DOCX
            let docx_add_time = web_sys::window().unwrap().performance().unwrap().now() - docx_add_start;
            web_sys::console::log_1(&format!("📄 [FLOOR-DOCX] Floor {} added to DOCX: {:.1}ms ({} images) [PREP: {:.1}ms + BATCH: {:.1}ms]", 
                selected_floor, docx_add_time, imgs.len(), prep_time, batch_time).into());
            
            // 🔍 PERFORMANCE: Общее время обработки этажа
            let floor_total_time = web_sys::window().unwrap().performance().unwrap().now() - floor_start;
            web_sys::console::log_1(&format!("🏁 [FLOOR-TOTAL] Floor {} processing complete: {:.1}ms (Images: {:.1}ms, DOCX: {:.1}ms)", 
                selected_floor, floor_total_time, images_time, docx_add_time).into());
            
            // Log completion point for gap measurement
            let floor_end_timestamp = web_sys::window().unwrap().performance().unwrap().now();
            web_sys::console::log_1(&format!("⏹️ [FLOOR-END] Floor {} completed at timestamp: {:.1}ms", 
                selected_floor, floor_end_timestamp).into());
        }
    }
    
    // 🔍 PERFORMANCE: Начало финальной сборки DOCX
    let build_start = web_sys::window().unwrap().performance().unwrap().now();
    
    // Создаем буфер и записываем документ
    let mut buffer = Cursor::new(Vec::new());
    match doc.build().pack(&mut buffer) {
        Ok(_) => {
            let build_time = web_sys::window().unwrap().performance().unwrap().now() - build_start;
            let buffer_size = buffer.get_ref().len();
            web_sys::console::log_1(&format!("✅ [DOCX-BUILD] Document built successfully: {:.1}ms, Size: {:.1}MB", 
                build_time, buffer_size as f64 / 1024.0 / 1024.0).into());
        },
        Err(e) => {
            web_sys::console::log_1(&format!("❌ [DOCX-BUILD] Error creating document: {}", e).into());
        }
    }
    
    // 🔍 PERFORMANCE: Общее время
    let total_time = web_sys::window().unwrap().performance().unwrap().now() - total_start;
    web_sys::console::log_1(&format!("🏁 [DOCX-TOTAL] Total DOCX generation time: {:.1}ms", total_time).into());
    
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