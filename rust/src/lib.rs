pub mod libs;
use std::{cell::RefCell, collections::HashMap};
use image::{ImageBuffer, Rgb, ImageOutputFormat};
use imageproc::drawing::{draw_line_segment_mut, draw_text_mut};
use rusttype::{Font, Scale};
use std::io::Cursor;
use web_sys::console;
use imageproc::point::Point;
use wasm_bindgen::prelude::*;
use docx_rs::{Docx, Paragraph, Pic, Run};
use serde::{Serialize, Deserialize};
use crate::libs::drawItem::DrawItemZ;
use ordered_float::OrderedFloat;
use libs::{ 
	arm_combination::SORTAMENT,
	parse::{convert_sli_xsl_to_json, EntityWithXlsx, Vertex}, 
	unification_data::unification_data,
	gpu_renderer::init_gpu_renderer,
};


#[wasm_bindgen]
pub fn log_data(x: f64, y: f64) {
    console::log_1(&format!("Координаты: x = {}, y = {}", x, y).into());
}

#[wasm_bindgen]
pub fn str_log_data(str:&str) {
    console::log_1(&format!("Координаты: x = {}", str).into());
}
// #[wasm_bindgen]
pub fn string_log_data(str:&String) {
    console::log_1(&format!("Координаты: x = {}", str).into());
}
pub fn string_log_two_params(load:&str, str:&String) {
    console::log_1(&format!("{} = {}",load,  str).into());
}


thread_local! {
	static GLOBAL_ENTITIES: RefCell<Option<Vec<EntityWithXlsx>>> = RefCell::new(None);
}

#[wasm_bindgen]//ТОчка входа в и вызызова с JS
pub fn parse_data(sli_data: &str,txt_data:&str, xlsx_data: &[u8]) {
    let parsed = convert_sli_xsl_to_json(sli_data, txt_data, xlsx_data);
    GLOBAL_ENTITIES.with(|cell| *cell.borrow_mut() = Some(parsed));
}

#[wasm_bindgen]
pub async fn initialize_gpu_renderer() -> Result<(), JsValue> {
    init_gpu_renderer().await
        .map_err(|e| JsValue::from_str(&format!("Failed to initialize GPU: {}", e)))
}


#[wasm_bindgen]
pub fn convert_sli_xsl_to_json_string() -> String {
    GLOBAL_ENTITIES.with(|cell| {
        cell.borrow()
            .as_ref()
            .map(|data| serde_json::to_string(data).unwrap())
            .unwrap_or_else(|| "[]".to_string())
    })
}

#[wasm_bindgen]
pub fn convert_data_to_js_order_byz() -> String {
    GLOBAL_ENTITIES.with(|cell| {
        cell.borrow()
            .as_ref()
            .map(|data| serde_json::to_string(data).unwrap())
            .unwrap_or_else(|| "[]".to_string())
    })
}

#[wasm_bindgen]
pub async fn create_docx(sli_data: &str, txt_data:&str,  xlsx_data: &[u8]) -> Vec<u8> {
    // Используем CPU-based генерацию документов
    crate::libs::docx_generator::create_enhanced_docx(sli_data, txt_data, xlsx_data).await
}

#[wasm_bindgen]
pub async fn create_docx_legacy(sli_data: &str, txt_data:&str,  xlsx_data: &[u8]) -> Vec<u8> {
    let entities = GLOBAL_ENTITIES.with(|cell| {
        cell.borrow()
            .as_ref()
            .cloned()
            .expect("Data not parsed! Call parse_and_store_data first!")
    });
    
    process_files(sli_data, txt_data, &xlsx_data);
    
    // Используем старый модуль для создания DOCX (с GPU)
    libs::generate_documents::docx_generator::create_docx_document(entities, "Hello, world!").await
}

pub fn create_docx_with_image(image_data: &[u8], doc: Docx) -> Result<Docx, Box<dyn std::error::Error>> {
    let mut doc = doc;
    
    // ИСПОЛЬЗУЕМ ЕДИНЫЕ КОНСТАНТЫ - ПРАВИЛЬНЫЕ ПРОПОРЦИИ A4!
    use crate::libs::drawItem::{DOCX_IMAGE_WIDTH_TWIPS, DOCX_IMAGE_HEIGHT_TWIPS};
    
	doc = doc.add_paragraph(
		Paragraph::new().add_run(
			Run::new().add_image(
			    Pic::new(image_data)
			        .size(DOCX_IMAGE_WIDTH_TWIPS, DOCX_IMAGE_HEIGHT_TWIPS)
			)
		)
	);
    Ok(doc)
}
#[derive(Serialize, Deserialize)]
struct SerializableEntity {
    vertices: Vec<Vertex>,
}
#[wasm_bindgen]
pub fn process_files(sli_data: &str,txt_data:&str,  xlsx_data: &[u8]) -> String {
    let parsed_data = convert_sli_xsl_to_json(sli_data, txt_data, xlsx_data);
    let serializable: Vec<SerializableEntity> = parsed_data
        .into_iter()
        .map(|e| SerializableEntity { vertices: e.vertices })
        .collect();
    serde_json::to_string(&serializable).unwrap()
}

pub fn new_draw_polygon(data: Vec<EntityWithXlsx>) -> Vec<u8> {
    let full_width = 680;
    let full_height = 900;
    let mut img = ImageBuffer::from_fn(full_width, full_height, |_, _| Rgb([255u8, 255u8, 255u8]));
    data.iter().for_each(|i| {
		log_data(i.vertices[0].x, i.vertices[0].y);
        if i.vertices.len() == 4 {
            // Масштабируем координаты в 10 раз
            let point_a = Point::new((i.vertices[0].x * 17.0)+150.0, (i.vertices[0].y * 17.0)+80.0);
            let point_b = Point::new((i.vertices[1].x * 17.0)+150.0, (i.vertices[1].y * 17.0)+80.0);
            let point_c = Point::new((i.vertices[2].x * 17.0)+150.0, (i.vertices[2].y * 17.0)+80.0);
            let point_d = Point::new((i.vertices[3].x * 17.0)+150.0, (i.vertices[3].y * 17.0)+80.0);

            // Рисуем линии между точками четырёхугольника
            draw_line_segment_mut(&mut img, (point_a.x as f32, point_a.y as f32), (point_b.x as f32, point_b.y as f32), Rgb([255, 0, 0]));
            draw_line_segment_mut(&mut img, (point_b.x as f32, point_b.y as f32), (point_c.x as f32, point_c.y as f32), Rgb([255, 0, 0]));
            draw_line_segment_mut(&mut img, (point_c.x as f32, point_c.y as f32), (point_d.x as f32, point_d.y as f32), Rgb([255, 0, 0]));
            draw_line_segment_mut(&mut img, (point_d.x as f32, point_d.y as f32), (point_a.x as f32, point_a.y as f32), Rgb([255, 0, 0]));
        }
    });

    let mut buffer = Vec::new();
    img.write_to(&mut Cursor::new(&mut buffer), ImageOutputFormat::Png).unwrap();
    buffer
}
// Функция sort_by_z перенесена в модуль libs::docx_generator

fn sort_by_same_z(data1: Vec<EntityWithXlsx>) -> HashMap<OrderedFloat<f32>, Vec<EntityWithXlsx>> {
    let mut map: HashMap<OrderedFloat<f32>, Vec<EntityWithXlsx>> = HashMap::new();

    for item in data1.into_iter() {
        let z0 = item.vertices[0].z;
        if item.vertices.iter().all(|v| v.z == z0) {
            let z = OrderedFloat(z0 as f32);
            map.entry(z)
                .or_insert_with(Vec::new)
                .push(item);
        }
    }
    map
}
#[wasm_bindgen]
pub fn get_changed_row_data(planes: JsValue) -> Vec<u8> {
    use serde_wasm_bindgen::from_value;
    // Десериализация JsValue в Vec<f32>
    let planes_vec: Vec<f32> = from_value(planes)
        .map_err(|e| JsValue::from_str(&format!("Ошибка десериализации: {}", e))).expect("msg");
    let data = GLOBAL_ENTITIES.with(|cell| {
        cell.borrow()
            .as_ref()
            .cloned()
            .expect("Data not parsed! Call parse_and_store_data first!")
    });
    let sorted_data = sort_by_same_z(data);
    let changed_row_data = unification_data(planes_vec, sorted_data);
    // Создаем 4 DXF файла для as1, as2, as3, as4
    let mut combined = Vec::new();
    for as_index in 0..4 {
        // Создаем DXF файл для конкретного параметра as
        let dxf_file = libs::createDxf::create_dxf_for_specific_as_manual(changed_row_data.clone(), as_index);
        // Добавляем размер файла и сам файл в общий массив
        combined.extend_from_slice(&(dxf_file.len() as u32).to_le_bytes());
        combined.extend(dxf_file);
    }
    combined
}
#[wasm_bindgen]
pub fn get_sortament_data() -> JsValue {
    let sortament_array = SORTAMENT.to_array();
    serde_wasm_bindgen::to_value(&sortament_array).unwrap_or(JsValue::NULL)
}

#[wasm_bindgen]
pub fn find_combinations_with_custom_diameters(
    target_area: f32,
    main_step: f32,
    secondary_step: f32,
    available_diameters: JsValue
) -> JsValue {
    use serde_wasm_bindgen::from_value;
    
    // Десериализация JsValue в Vec<u32>
    let diameters: Vec<u32> = match from_value(available_diameters) {
        Ok(d) => d,
        Err(e) => {
            console::log_1(&format!("Ошибка десериализации диаметров: {}", e).into());
            return JsValue::NULL;
        }
    };
    
    // Получаем комбинации с пользовательскими диаметрами
    let combinations = SORTAMENT.find_combinations_for_area_with_custom_diameters(
        target_area,
        main_step,
        secondary_step,
        &diameters
    );
    
    // Сериализуем результат обратно в JsValue
    match serde_wasm_bindgen::to_value(&combinations) {
        Ok(js_value) => js_value,
        Err(e) => {
            console::log_1(&format!("Ошибка сериализации результата: {}", e).into());
            JsValue::NULL
        }
    }
}
// В начале файла добавьте:
use std::io::Write;

use crate::libs::final_report::custom_sortament::{CustomSortament, FloorData};

// ... остальные импорты ...

#[wasm_bindgen]
pub fn create_csv_from_all_parsed_entities() -> Vec<u8> {
    let data = GLOBAL_ENTITIES.with(|cell| {
        cell.borrow()
            .as_ref()
            .cloned()
            .expect("Data not parsed! Call parse_and_store_data first!")
    });
    
    // Создаем буфер для CSV данных
    let mut csv_content = Vec::new();
    
    // Добавляем заголовок CSV
    writeln!(&mut csv_content, "Номер,Тип,X,Y,Z,Материал,Номер_материала,Тип_SG,B_или_D,H_или_D,H")
        .expect("Failed to write CSV header");
    
    // Перебираем все элементы и добавляем их в CSV
    for (i, element) in data.iter().enumerate() {
        // Получаем информацию о материале
        let material_num = element.material.as_ref().and_then(|m| m.material_num).unwrap_or(0);
        let sg_type = element.material.as_ref().and_then(|m| m.sg_type.clone()).unwrap_or_else(|| String::from("-"));
        let b_or_d = element.material.as_ref().and_then(|m| m.b_or_d).unwrap_or(0.0);
        let h_or_d = element.material.as_ref().and_then(|m| m.h_or_d).unwrap_or(0.0);
        let h = element.material.as_ref().and_then(|m| m.h).unwrap_or(0.0);
        
        if element.vertices.is_empty() {
            // Если у элемента нет координат, записываем строку с нулевыми координатами
            writeln!(
                &mut csv_content,
                "{},{},0.0,0.0,0.0,{},{},{},{},{}",
                i,
                element.entity_type,
                material_num,
                sg_type,
                b_or_d,
                h_or_d,
                h
            ).expect("Failed to write CSV row");
        } else {
            // Записываем каждую координату элемента в отдельной строке
            for vertex in &element.vertices {
                writeln!(
                    &mut csv_content,
                    "{},{},{},{},{},{},{},{},{},{}",
                    i,
                    element.entity_type,
                    vertex.x,
                    vertex.y,
                    vertex.z,
                    material_num,
                    sg_type,
                    b_or_d,
                    h_or_d,
                    h
                ).expect("Failed to write CSV row");
            }
        }
    }
    csv_content
}
#[wasm_bindgen]
pub fn get_excell_report_for_arms() -> Vec<u8> {
    use crate::libs::arm_combination::SORTAMENT;
    
    SORTAMENT.generate_excel_report_to_wasm()
        .unwrap_or_else(|_| Vec::new())
}

#[wasm_bindgen]
pub fn get_custom_sortament_report(
    available_diameters: Vec<u32>,
    floors_data_json: &str,
) -> Result<Vec<u8>, JsValue> {
    use crate::libs::final_report::custom_sortament::create_custom_sortament_report;
    
    create_custom_sortament_report(available_diameters, floors_data_json)
}

#[wasm_bindgen]
pub fn get_table_data_for_frontend(
    available_diameters: Vec<u32>,
    floors_data_json: &str,
) -> Result<String, JsValue> {
    let floors_data: Vec<FloorData> = serde_json::from_str(floors_data_json)
        .map_err(|e| JsValue::from_str(&format!("JSON parsing error: {}", e)))?;
    
    let sortament = CustomSortament::from_js_data(available_diameters);
    let excel_data = sortament.generate_excel_data_for_js(floors_data);
    
    serde_json::to_string(&excel_data)
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}

#[wasm_bindgen]
pub async fn create_docx_for_selected_combinations(selected_floors_json: &str) -> Vec<u8> {
    use serde_json;
    
    // Десериализация JSON с выбранными этажами и комбинациями
    let selected_floors: Vec<f32> = serde_json::from_str(selected_floors_json)
        .expect("Failed to parse selected floors JSON");
    
    // Получаем все сущности из глобального хранилища
    let entities = GLOBAL_ENTITIES.with(|cell| {
        cell.borrow()
            .as_ref()
            .cloned()
            .expect("Data not parsed! Call parse_and_store_data first!")
    });
    
    // Используем новый модуль для создания DOCX
    libs::generate_documents::docx_generator::create_docx_for_selected_floors(
        entities,
        selected_floors,
        "Документ с выбранными комбинациями арматуры"
    ).await
}

// ... existing code ...

use crate::libs::convas_optimization::canvas_optimization::{
    get_optimized_canvas_data, get_canvas_statistics
};

#[wasm_bindgen]
pub fn get_optimized_canvas_data_wasm(
    max_shapes_per_level: usize,
    max_total_shapes: usize,
    start_z: Option<f32>,
    end_z: Option<f32>
) -> String {
    let data = GLOBAL_ENTITIES.with(|cell| {
        cell.borrow()
            .as_ref()
            .cloned()
            .expect("Data not parsed! Call parse_and_store_data first!")
    });
    
    let canvas_data = get_optimized_canvas_data(
        data,
        max_shapes_per_level,
        max_total_shapes,
        start_z,
        end_z
    );
    
    serde_json::to_string(&canvas_data)
        .unwrap_or_else(|_| "{\"error\": \"Serialization failed\"}".to_string())
}

#[wasm_bindgen]
pub fn get_canvas_statistics_wasm() -> String {
    let data = GLOBAL_ENTITIES.with(|cell| {
        cell.borrow()
            .as_ref()
            .cloned()
            .expect("Data not parsed! Call parse_and_store_data first!")
    });
    
    let stats = get_canvas_statistics(data);
    
    serde_json::to_string(&stats)
        .unwrap_or_else(|_| "{\"error\": \"Serialization failed\"}".to_string())
}

// ... existing code ...