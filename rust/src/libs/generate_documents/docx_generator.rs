use std::{ collections::HashMap, io::Cursor};
use docx_rs::{Docx, Paragraph, Pic, Run};
use ordered_float::OrderedFloat;
use crate::libs::drawItem::DrawItemZ;
use crate::libs::parse::EntityWithXlsx;
use crate::libs::generate_documents::performance::{
    PerformanceMonitor, PerformanceConfig};

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
    pub is_default_checked: bool,
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

    
    match selected_floors {
        Some(floors) => {
              create_docx_for_selected_floors(entities, floors, selected_combinations, title).await
          },
         None => {
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
    let mut monitor = PerformanceMonitor::new(config);

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
    for (floor_index, (key, item_z)) in hash.iter().enumerate() {
        web_sys::console::log_1(&format!("📊 Processing floor {} of {} (height: {})", 
            floor_index + 1, floors_count, key.to_string()).into());
        
        // Используем GPU-оптимизированную батчевую генерацию изображений
        let images = item_z.draw_all_images_with_colors(None).await;
        

        
        // Добавляем заголовок этажа
        let run = Run::new()
            .add_text(format!("Высота {}", &key.to_string()))
            .bold()
            .size(22);
        doc = doc.add_paragraph(Paragraph::new().add_run(run));
        
        // Добавляем изображения в документ
        for (_, img) in images.iter().enumerate() {

                
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
    
    monitor.start_docx_creation();
    
    // Создаем буфер и записываем документ
    let mut buffer = Cursor::new(Vec::new());
    match doc.build().pack(&mut buffer) {
        Ok(_) => web_sys::console::log_1(&"✅ [DOCX-CREATE] DOCX document built successfully".into()),
        Err(e) => web_sys::console::log_1(&format!("❌ [DOCX-CREATE] Error creating document: {}", e).into()),
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

/// Создает DOCX документ для выбранных этажей с тройным циклом генерации
pub async fn create_docx_for_selected_floors(
    entities: Vec<EntityWithXlsx>,
    selected_floors: Vec<f32>,
    selected_combinations: Option<Vec<SelectedCombination>>,
    title: &str
) -> Vec<u8> {
    web_sys::console::log_1(&format!("[STEP 4] create_docx_for_selected_floors() called with {} entities, {} floors", entities.len(), selected_floors.len()).into());
    web_sys::console::log_1(&format!("[STEP 4] selected_combinations: {:?}", selected_combinations.as_ref().map(|c| c.len())).into());
    
    let total_start = match web_sys::window() {
        Some(window) => match window.performance() {
            Some(performance) => performance.now(),
            None => {
                web_sys::console::warn_1(&"Performance API недоступен, используем 0.0".into());
                0.0
            }
        },
        None => {
            web_sys::console::warn_1(&"Window объект недоступен в воркере, используем 0.0".into());
            0.0
        }
    };
    
    let mut doc = Docx::new()
        .page_margin(docx_rs::PageMargin {
            top: 200, right: 200, bottom: 200, left: 200,
            header: 0, footer: 0, gutter: 0,
        })
        .add_paragraph(
            Paragraph::new().add_run(
                Run::new().add_text(title)
            )
        );
    
    // Настраиваем размер и ориентацию страницы
    use crate::libs::drawItem::{DOCX_IMAGE_WIDTH_EMU, DOCX_IMAGE_HEIGHT_EMU, DOCX_PAGE_WIDTH_TWIPS, DOCX_PAGE_HEIGHT_TWIPS};
    doc = doc
        .page_size(DOCX_PAGE_WIDTH_TWIPS, DOCX_PAGE_HEIGHT_TWIPS)
        .page_orient(docx_rs::PageOrientationType::Landscape);
    
    // Группируем сущности по Z-координате (этажам)
    let hash = sort_by_z(entities);
    
    // Фильтруем только выбранные комбинации с default_checked = true
    let filtered_combinations: Vec<&SelectedCombination> = if let Some(combinations) = &selected_combinations {
        combinations.iter()
            .filter(|combo| combo.combination.is_default_checked)
            .collect()
    } else {
        Vec::new()
    };    
    // Тройной цикл: этажи × as_функции × комбинации
    for selected_floor in selected_floors.iter() {
        let z_key = OrderedFloat(*selected_floor);
        
        if let Some(item_z) = hash.get(&z_key) {
            // Добавляем заголовок этажа
            let floor_title_paragraph = Paragraph::new().add_run(
                Run::new()
                    .add_text(format!("Высота {}", selected_floor))
                    .size(40)
            );
            doc = doc.add_paragraph(floor_title_paragraph);
            
            // Группируем комбинации по as_функциям для данного этажа
            let floor_combinations: Vec<&SelectedCombination> = filtered_combinations.iter()
                .filter(|combo| combo.floor_level == selected_floor.to_string())
                .cloned()
                .collect();
            
            // Цикл по as_функциям
            for as_function in ["as1", "as2", "as3", "as4"].iter() {
                let as_combinations: Vec<&SelectedCombination> = floor_combinations.iter()
                    .filter(|combo| combo.function_name == *as_function)
                    .cloned()
                    .collect();
                
                if !as_combinations.is_empty() {
                    // Добавляем заголовок as_функции
                    let as_title_paragraph = Paragraph::new().add_run(
                        Run::new()
                            .add_text(format!("Функция {}", as_function))
                            .size(32)
                    );
                    doc = doc.add_paragraph(as_title_paragraph);
                    
                    // Цикл по комбинациям
                    for combination in as_combinations {
                        // Создаем result_scale для конкретной комбинации
                        let result_scale = combination.combination.result_scale.as_deref();
                        let result_scales = vec![result_scale];
                        
                        // Генерируем изображение для данной комбинации
                        let img = item_z.draw_single_image_with_combination(
                            &result_scales, 
                            *selected_floor, 
                            as_function,
                            &combination.combination
                        ).await;
                        
                        // Добавляем заголовок комбинации
                        let combo_title = format!(
                            "Комбинация: Ø{} мм + Ø{} мм, Площадь: {:.3} см²",
                            combination.combination.main_diameter,
                            combination.combination.additional_diameter,
                            combination.combination.total_area
                        );
                        let combo_title_paragraph = Paragraph::new().add_run(
                            Run::new()
                                .add_text(combo_title)
                                .size(24)
                        );
                        doc = doc.add_paragraph(combo_title_paragraph);
                        
                        // Добавляем изображение
                        let image_paragraph = Paragraph::new().add_run(
                            Run::new().add_image(
                                Pic::new(img.as_slice())
                                    .size(DOCX_IMAGE_WIDTH_EMU, DOCX_IMAGE_HEIGHT_EMU)
                            )
                        );
                        doc = doc.add_paragraph(image_paragraph);
                    }
                }
            }
        }
    }
    
    // Создаем буфер и записываем документ
    let mut buffer = Cursor::new(Vec::new());
    match doc.build().pack(&mut buffer) {
        Ok(_) => {
            let total_time = match web_sys::window() {
        Some(window) => match window.performance() {
            Some(performance) => performance.now() - total_start,
            None => 0.0
        },
        None => 0.0
    };
    web_sys::console::log_1(&format!("⏱️ [DOCX-TIME] Генерация завершена за {:.2}мс", total_time).into());
            web_sys::console::log_1(&format!("Total DOCX generation time: {:.2}ms", total_time).into());
        },
        Err(e) => {
            web_sys::console::log_1(&format!("Error creating document: {}", e).into());
        }
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