
use std::{cell::RefCell, collections::HashMap, ops::Deref};
use image::{ImageBuffer, Rgb, ImageOutputFormat};
use imageproc::drawing::{draw_line_segment_mut, draw_text_mut, draw_polygon, draw_polygon_mut, Canvas};
use libs::{createDxf::{create_dxf_entity_xlsx, create_dxf_file}, parse::{convert_sli_xsl_to_json, EntityWithXlsx, Vertex}};
use rusttype::{Font, Scale};
use std::io::Cursor;
use web_sys::{console,};
use imageproc::point::Point;
use wasm_bindgen::prelude::*;
use docx_rs::{Docx, Paragraph, Pic, Run};
use serde::{Serialize, Deserialize};
use libs::drawItem::Draw_Item_Z;
use ordered_float::OrderedFloat;

pub mod libs{
	pub mod parse;
	pub mod drawItem;
	pub mod createDxf;
	pub mod getTransformedObject;
}

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
pub fn parse_data(sli_data: &str, xlsx_data: &[u8]) {
    let parsed = convert_sli_xsl_to_json(sli_data, xlsx_data);
	
    GLOBAL_ENTITIES.with(|cell| *cell.borrow_mut() = Some(parsed));
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
pub fn create_docx(sli_data: &str, xlsx_data: &[u8]) -> Vec<u8> {
    let mut doc = Docx::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Hello, world!")));
	let entities = GLOBAL_ENTITIES.with(|cell| {
        cell.borrow()
            .as_ref()
            .cloned()
            .expect("Data not parsed! Call parse_and_store_data first!")
    });
	let hash = sort_by_z(entities);
	let width_cm = 140;
    let height_cm = 105;
	for (key, item_z) in hash{
		let item = item_z;
		let imgs =  item.draw_all_images();
		let run = Run::new()
			.add_text(format!("Высота {}",&key.to_string()))
			.bold() // Жирный шрифт
			.size(22);
		doc = doc.add_paragraph(Paragraph::new().add_run(run));
		for img in imgs{
			doc=doc
				.page_size(width_cm * 290,  height_cm * 280)
				.page_orient(docx_rs::PageOrientationType::Landscape)
				.add_paragraph(
					Paragraph::new().add_run(
						Run::new().add_image(Pic::new(&img))
					)
			);
		}

	}
	let mut buffer = Cursor::new(Vec::new());
	match doc.build().pack( &mut buffer) {
		Ok(_) => (),
	    Err(e) => println!("Ошибка: {}", e),
	}
	process_files(sli_data, &xlsx_data);
	buffer.into_inner()
}

#[wasm_bindgen]
pub fn create_png_in_memory() -> Vec<u8> {
    // 1. Создаем белый холст 400x400 пикселей
	let full_width =800;
	let full_height =400;
    let mut img = ImageBuffer::from_fn(full_width, full_height, |_, _| Rgb([255u8, 255u8, 255u8]));

    // 2. Параметры сетки
    let grid_step = 100; // 10 делений (0-10) на 400px
    let grid_color = Rgb([200u8, 200u8, 200u8]);
    let text_color = Rgb([0u8, 0u8, 0u8]);
    let font_size = 15.0;
    // 3. Загружаем шрифт (файл arial.ttf должен быть в корне проекта!)
    let font_data = include_bytes!("Roboto_Condensed-Black.ttf");
    let font = Font::try_from_bytes(font_data).unwrap();
    let scale = Scale::uniform(font_size);
    // 4. Рисуем горизонтальные линии (ось Y)
    for y in (0..=full_height).step_by(grid_step) {
        draw_line_segment_mut(
            &mut img,
            (50.0, y as f32),
            (full_width as f32, y as f32),
            grid_color,
        );
    }

    // 5. Рисуем вертикальные линии (ось X)
    for x in (0..=full_width).step_by(grid_step) {
        draw_line_segment_mut(
            &mut img,
            (x as f32, 50.0),
            (x as f32, 800.0-50.0),
            grid_color,
        );
    }

    // 6. Числа по горизонтальной оси (0-10)
    for (i, x) in (0..=full_height).step_by(grid_step).enumerate() {
        let number = i.to_string();
        draw_text_mut(
            &mut img,
            text_color,
            x as i32 - 7, // Центрирование текста
            390,          // Позиция внизу
            scale,
            &font,
            &number,
        );
    }

    // 7. Числа по вертикальной оси (10-0 сверху вниз)
	
    for (i, y) in (0..=400).step_by(grid_step).enumerate() {
        let number = (10 - i).to_string();
        draw_text_mut(
            &mut img,
            text_color,
            5,             // Позиция слева
            y as i32 - 5,  // Центрирование
            scale,
            &font,
            &number,
        );
    }

    // 8. Сохраняем в буфер
    let mut buffer = Vec::new();
    img.write_to(&mut Cursor::new(&mut buffer), ImageOutputFormat::Png)
        .unwrap();
    buffer
}

pub fn create_docx_with_image(image_data: &[u8], doc: Docx) -> Result<Docx, Box<dyn std::error::Error>> {
    let mut doc = doc;
	doc = doc.add_paragraph(
		Paragraph::new().add_run(
			Run::new().add_image(Pic::new(image_data))
		)
	);
    Ok(doc)
}
#[derive(Serialize, Deserialize)]
struct SerializableEntity {
    vertices: Vec<Vertex>,
}
#[wasm_bindgen]
pub fn process_files(sli_data: &str, xlsx_data: &[u8]) -> String {
    let parsed_data = convert_sli_xsl_to_json(sli_data, xlsx_data);
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
// fn sort_by_z(data: Vec<EntityWithXlsx>) -> HashMap<OrderedFloat<f32>,  Draw_Item_Z> {
//     let mut map: HashMap<OrderedFloat<f32>, Draw_Item_Z> = HashMap::new();
//     for item in data {
//         let z = item.vertices[0].z ;
//         // Получаем или создаем вектор для текущего z и добавляем вершины
//         map.entry(OrderedFloat(z as f32)).or_insert_with(||
// 			Draw_Item_Z{
// 			data:item
// 		});
//     }
//     map
// }

fn sort_by_z(data1: Vec<EntityWithXlsx>) -> HashMap<OrderedFloat<f32>, Draw_Item_Z> {
    let mut map: HashMap<OrderedFloat<f32>, Draw_Item_Z> = HashMap::new();

    for item in data1.into_iter() {
		let z0 = item.vertices[0].z;
		if item.vertices.iter().all(|v| v.z == z0) {
			let z = OrderedFloat(z0 as f32);
			map.entry(z)
				.or_insert_with(|| Draw_Item_Z { data: Vec::new() })
				.data.push(item); // Здесь теперь item перемещается, а не заимствуется
		}
	}
    map
}