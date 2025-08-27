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
    let mut monitor = PerformanceMonitor::new(config);
    let elements_count = entities.len();
    
    web_sys::console::log_1(&format!("🚀 Starting DOCX generation for {} elements", elements_count).into());
    
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
    
    // Группируем по Z-координате (этажам)
    let hash = sort_by_z(entities);
    
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
    
    // Обрабатываем только выбранные этажи
    for (floor_idx, selected_floor) in selected_floors.iter().enumerate() {
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
             
             let imgs = item_z.draw_all_images_with_colors(Some(&result_scales)).await;
             
             web_sys::console::log_1(&format!("[STEP 8] draw_all_images_with_colors() returned {} images", imgs.len()).into());
            
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