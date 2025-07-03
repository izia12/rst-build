use std::collections::{HashMap, HashSet};
use super::drawItem::DrawItemZ;
use super::parse::EntityWithXlsx;
use std::io:: Cursor;

use dxf::enums::{HorizontalTextJustification, VerticalTextJustification};
use dxf::{entities::*, Block, Color, Drawing, Vector};
use dxf::Point;
use ordered_float::OrderedFloat;


pub fn create_dxf_file (data:Vec<EntityWithXlsx>)->Vec<u8>{
	let mut drawing = Drawing::new();
for item in data{
	if item.vertices.len()==4{
		let face3d = Face3D::new(
			Point::new(item.vertices[0].x, item.vertices[0].y, item.vertices[0].z),
			Point::new(item.vertices[1].x, item.vertices[1].y, item.vertices[1].z),
			Point::new(item.vertices[2].x, item.vertices[2].y, item.vertices[2].z),
			Point::new(item.vertices[3].x, item.vertices[3].y, item.vertices[3].z),
		);
		drawing.add_entity(Entity::new(EntityType::Face3D(face3d)));
	}
	if item.vertices.len()==3{
		let face3d = Face3D::new(
			Point::new(item.vertices[0].x, item.vertices[0].y, item.vertices[0].z),
			Point::new(item.vertices[1].x, item.vertices[1].y, item.vertices[1].z),
			Point::new(item.vertices[2].x, item.vertices[2].y, item.vertices[2].z),
			Point::new(item.vertices[0].x, item.vertices[0].y, item.vertices[0].z),
		);
		drawing.add_entity(Entity::new(EntityType::Face3D(face3d)));
	}
	if item.vertices.len()==2{
		let line = Line::new(
			Point::new(item.vertices[0].x, item.vertices[0].y, item.vertices[0].z),
			Point::new(item.vertices[1].x, item.vertices[1].y, item.vertices[1].z),
		);
		drawing.add_entity(Entity::new(EntityType::Line(line)));
	}
}
    // Добавляем простые линии
	// let face3d = Face3D::new(
	// 	Point::new(0.0, 0.0, 0.0), Point::new(100.0, 0.0, 0.0),
	// 	Point::new(0.0, 0.0, 0.0), Point::new(100.0, 0.0, 0.0),
	// );
    let mut buffer = Cursor::new(Vec::new());
    drawing.save(&mut buffer).expect("Ошибка записи DXF");

	buffer.into_inner()
    // Сохраняем файл

}
pub fn create_dxf_entity_xlsx(data:HashMap<OrderedFloat<f32>, DrawItemZ>)->Vec<u8>{
	let mut drawing = Drawing::new();
	for (_, item_z) in data{
		for v in item_z.data{
			if v.vertices.len()==4{
				let face3d = Face3D::new(
					Point::new(v.vertices[0].x, v.vertices[0].y, v.vertices[0].z),
					Point::new(v.vertices[1].x, v.vertices[1].y, v.vertices[1].z),
					Point::new(v.vertices[2].x, v.vertices[2].y, v.vertices[2].z),
					Point::new(v.vertices[3].x, v.vertices[3].y, v.vertices[3].z),
				);
				drawing.add_entity(Entity::new(EntityType::Face3D(face3d)));
			}
			if v.vertices.len()==3{
				let face3d = Face3D::new(
					Point::new(v.vertices[0].x, v.vertices[0].y, v.vertices[0].z),
					Point::new(v.vertices[1].x, v.vertices[1].y, v.vertices[1].z),
					Point::new(v.vertices[2].x, v.vertices[2].y, v.vertices[2].z),
					Point::new(v.vertices[0].x, v.vertices[0].y, v.vertices[0].z),
				);
				drawing.add_entity(Entity::new(EntityType::Face3D(face3d)));
			}
			if v.vertices.len()==2{
				let line = Line::new(
					Point::new(v.vertices[0].x, v.vertices[0].y, v.vertices[0].z),
					Point::new(v.vertices[1].x, v.vertices[1].y, v.vertices[1].z),
				);
				drawing.add_entity(Entity::new(EntityType::Line(line)));
			}
		}
	}
	let mut buffer = Cursor::new(Vec::new());
    drawing.save(&mut buffer).expect("Ошибка записи DXF");

	buffer.into_inner()
}
fn create_text_entity(center_x: f64, center_y: f64, z: f64, y_offset: f64, 
		value: String, color_index: u8) -> Entity {
		let text = Text {
		thickness: 0.0,
		location: Point::new(center_x, center_y + y_offset, z),
		text_height: 0.05,
		value,
		rotation: 0.0,
		relative_x_scale_factor: 1.0,
		oblique_angle: 0.0,
		text_style_name: String::from("STANDARD"),
		text_generation_flags: 0,
		horizontal_text_justification: HorizontalTextJustification::Left,
		second_alignment_point: Point::origin(),
		normal: Vector::z_axis(),
		vertical_text_justification: VerticalTextJustification::Baseline,
		};

		let mut text_entity = Entity::new(EntityType::Text(text));
		if color_index > 0 {
		text_entity.common.color = Color::from_index(color_index);
}

text_entity
}

pub fn create_dxf_after_change(data: HashMap<OrderedFloat<f32>, Vec<EntityWithXlsx>>) -> Vec<Vec<u8>> {
    // Создаем 4 файла для as1, as2, as3, as4
    let mut result = Vec::new();
    
    for as_index in 0..4 {
        let mut drawing = Drawing::new();
        
        // Создаем один общий блок для всех измененных элементов
        let block_name = format!("CHANGED_ELEMENTS_AS{}", as_index + 1);
        let mut changed_elements_block = Block {
            name: String::from(&block_name),
            base_point: Point::new(0.0, 0.0, 0.0),
            entities: Vec::new(),
            layer: String::from("0"),
            description: String::new(),
            xref_path_name: String::new(),
            handle: dxf::Handle(0),
            __owner_handle: dxf::Handle(0),
            flags: 0,
            is_in_paperspace: false,
            extension_data_groups: Vec::new(),
            x_data: Vec::new(),
        };
        // Сначала собираем все измененные элементы в блок
        for (_z, entities) in data.iter() {
            for entity in entities {
                if entity.changed {
                    match entity.vertices.len() {
                        4 => {
                            let face3d = Face3D::new(
                                Point::new(entity.vertices[0].x, entity.vertices[0].y, entity.vertices[0].z),
                                Point::new(entity.vertices[1].x, entity.vertices[1].y, entity.vertices[1].z),
                                Point::new(entity.vertices[2].x, entity.vertices[2].y, entity.vertices[2].z),
                                Point::new(entity.vertices[3].x, entity.vertices[3].y, entity.vertices[3].z),
                            );
                            let mut face_entity = Entity::new(EntityType::Face3D(face3d));
                            face_entity.common.color = Color::from_index(1); // 1 - красный цвет в DXF
                            changed_elements_block.entities.push(face_entity);
                            
                            // Добавляем только один текст с соответствующим значением as
                            if let Some(row) = &entity.row {
                                let center_x = (entity.vertices[0].x + entity.vertices[2].x) / 2.0;
                                let center_y = (entity.vertices[0].y + entity.vertices[2].y) / 2.0;
                                let z = entity.vertices[0].z;
                                
                                // Выбираем только один параметр в зависимости от индекса файла
                                let as_value = match as_index {
                                    0 => row.as1[0],
                                    1 => row.as2[0],
                                    2 => row.as3[0],
                                    3 => row.as4[0],
                                    _ => 0.0,
                                };
                                
                                let as_name = format!("as{}", as_index + 1);
                                changed_elements_block.entities.push(create_text_entity(
                                    center_x, center_y, z, 0.0, 
                                    format!("{}:{:.1}", as_name, as_value), 2
                                ));
                            }
                        },
                        3 => {
                            let face3d = Face3D::new(
                                Point::new(entity.vertices[0].x, entity.vertices[0].y, entity.vertices[0].z),
                                Point::new(entity.vertices[1].x, entity.vertices[1].y, entity.vertices[1].z),
                                Point::new(entity.vertices[2].x, entity.vertices[2].y, entity.vertices[2].z),
                                Point::new(entity.vertices[0].x, entity.vertices[0].y, entity.vertices[0].z),
                            );
                            
                            let mut face_entity = Entity::new(EntityType::Face3D(face3d));
                            face_entity.common.color = Color::from_index(1); // 1 - красный цвет в DXF
                            changed_elements_block.entities.push(face_entity);
                            
                            // Добавляем только один текст с соответствующим значением as
                            if let Some(row) = &entity.row {
                                let center_x = (entity.vertices[0].x + entity.vertices[1].x + entity.vertices[2].x) / 3.0;
                                let center_y = (entity.vertices[0].y + entity.vertices[1].y + entity.vertices[2].y) / 3.0;
                                let z = entity.vertices[0].z;
                                
                                // Выбираем только один параметр в зависимости от индекса файла
                                let as_value = match as_index {
                                    0 => row.as1[0],
                                    1 => row.as2[0],
                                    2 => row.as3[0],
                                    3 => row.as4[0],
                                    _ => 0.0,
                                };
                                
                                let as_name = format!("as{}", as_index + 1);
                                changed_elements_block.entities.push(create_text_entity(
                                    center_x, center_y, z, 0.0, 
                                    format!("{}:{:.1}", as_name, as_value), 2
                                ));
                            }
                        },
                        2 => {
                            let line = Line::new(
                                Point::new(entity.vertices[0].x, entity.vertices[0].y, entity.vertices[0].z),
                                Point::new(entity.vertices[1].x, entity.vertices[1].y, entity.vertices[1].z),
                            );
                            let mut line_entity = Entity::new(EntityType::Line(line));
                            line_entity.common.color = Color::from_index(1); // 1 - красный цвет в DXF
                            changed_elements_block.entities.push(line_entity);
                        },
                        _ => {}
                    }
                }
            }
        }
        
        // Добавляем блок в чертеж, если в нем есть элементы
        if !changed_elements_block.entities.is_empty() {
            drawing.add_block(changed_elements_block);
            
            // Создаем вставку блока и добавляем ее в чертеж
            let insert = Insert {
                name: String::from(&block_name),
                location: Point::new(0.0, 0.0, 0.0),
                x_scale_factor: 1.0,
                y_scale_factor: 1.0,
                z_scale_factor: 1.0,
                rotation: 0.0,
                column_count: 1,
                row_count: 1,
                column_spacing: 0.0,
                row_spacing: 0.0,
                __seqend_handle: dxf::Handle(0),
                __has_attributes: false,
                extrusion_direction: Vector::z_axis(),
                __attributes_and_handles: Vec::new(),
            };
            drawing.add_entity(Entity::new(EntityType::Insert(insert)));
        }
        
        // Теперь добавляем все неизмененные элементы напрямую в чертеж
        for (_z, entities) in data.iter() {
            for entity in entities {
                if !entity.changed {
                    match entity.vertices.len() {
                        4 => {
                            let face3d = Face3D::new(
                                Point::new(entity.vertices[0].x, entity.vertices[0].y, entity.vertices[0].z),
                                Point::new(entity.vertices[1].x, entity.vertices[1].y, entity.vertices[1].z),
                                Point::new(entity.vertices[2].x, entity.vertices[2].y, entity.vertices[2].z),
                                Point::new(entity.vertices[3].x, entity.vertices[3].y, entity.vertices[3].z),
                            );
                            drawing.add_entity(Entity::new(EntityType::Face3D(face3d)));
                            
                            // Добавляем только один текст с соответствующим значением as
                            if let Some(row) = &entity.row {
                                let center_x = (entity.vertices[0].x + entity.vertices[2].x) / 2.0;
                                let center_y = (entity.vertices[0].y + entity.vertices[2].y) / 2.0;
                                let z = entity.vertices[0].z;
                                
                                // Выбираем только один параметр в зависимости от индекса файла
                                let as_value = match as_index {
                                    0 => row.as1[0],
                                    1 => row.as2[0],
                                    2 => row.as3[0],
                                    3 => row.as4[0],
                                    _ => 0.0,
                                };
                                
                                let as_name = format!("as{}", as_index + 1);
                                drawing.add_entity(create_text_entity(
                                    center_x, center_y, z, 0.0, 
                                    format!("{}:{:.1}", as_name, as_value), 0
                                ));
                            }
                        },
                        3 => {
                            let face3d = Face3D::new(
                                Point::new(entity.vertices[0].x, entity.vertices[0].y, entity.vertices[0].z),
                                Point::new(entity.vertices[1].x, entity.vertices[1].y, entity.vertices[1].z),
                                Point::new(entity.vertices[2].x, entity.vertices[2].y, entity.vertices[2].z),
                                Point::new(entity.vertices[0].x, entity.vertices[0].y, entity.vertices[0].z),
                            );
                            drawing.add_entity(Entity::new(EntityType::Face3D(face3d)));
                            
                            // Добавляем только один текст с соответствующим значением as
                            if let Some(row) = &entity.row {
                                let center_x = (entity.vertices[0].x + entity.vertices[2].x) / 2.0;
                                let center_y = (entity.vertices[0].y + entity.vertices[2].y) / 2.0;
                                let z = entity.vertices[0].z;
                                
                                // Выбираем только один параметр в зависимости от индекса файла
                                let as_value = match as_index {
                                    0 => row.as1[0],
                                    1 => row.as2[0],
                                    2 => row.as3[0],
                                    3 => row.as4[0],
                                    _ => 0.0,
                                };
                                
                                let as_name = format!("as{}", as_index + 1);
                                drawing.add_entity(create_text_entity(
                                    center_x, center_y, z, 0.0, 
                                    format!("{}:{:.1}", as_name, as_value), 0
                                ));
                            }
                        },
                        2 => {
                            let line = Line::new(
                                Point::new(entity.vertices[0].x, entity.vertices[0].y, entity.vertices[0].z),
                                Point::new(entity.vertices[1].x, entity.vertices[1].y, entity.vertices[1].z),
                            );
                            drawing.add_entity(Entity::new(EntityType::Line(line)));
                        },
                        _ => {}
                    }
                }
            }
        }

        let mut buffer = Cursor::new(Vec::new());
        drawing.save(&mut buffer).expect("Ошибка записи DXF");
        
        result.push(buffer.into_inner());
    }
    
    result
}
// ... existing code ...

pub fn create_dxf_for_specific_as(data: HashMap<OrderedFloat<f32>, Vec<EntityWithXlsx>>, as_index: usize) -> Vec<u8> {
    let mut drawing = Drawing::new();
	let mut sum_layers:Vec<String> = Vec::new();
	// drawing.header.version = AcadVersion::R2007;
	// let layer =Layer {
	// 	name:format!("{}-{}",b_or_d_str, h_or_d_str),
	// 	color: Color::from_index(2),
	// 	..Default::default()
	// };
    // Создаем один общий блок для всех измененных элементов
    let block_name = format!("CHANGED_ELEMENTS_AS{}", as_index + 1);
    let mut changed_elements_block = Block {
        name: String::from(&block_name),
        base_point: Point::new(0.0, 0.0, 0.0),
        entities: Vec::new(),
        layer: String::from("0"),
        description: String::new(),
        xref_path_name: String::new(),
        handle: dxf::Handle(0),
        __owner_handle: dxf::Handle(0),
        flags: 0,
        is_in_paperspace: false,
        extension_data_groups: Vec::new(),
        x_data: Vec::new(),
    };
    // Сначала собираем все измененные элементы в блок
    for (_z, entities) in data.iter() {
        for entity in entities {
            if entity.changed {
                match entity.vertices.len() {
                    4 => {
                        let face3d = Face3D::new(
                            Point::new(entity.vertices[0].x, entity.vertices[0].y, entity.vertices[0].z),
                            Point::new(entity.vertices[1].x, entity.vertices[1].y, entity.vertices[1].z),
                            Point::new(entity.vertices[2].x, entity.vertices[2].y, entity.vertices[2].z),
                            Point::new(entity.vertices[3].x, entity.vertices[3].y, entity.vertices[3].z),
                        );
                        let mut face_entity = Entity::new(EntityType::Face3D(face3d));
						if entity.material.as_ref().unwrap().sg_type!=None{
                            let sg_type = entity.material.clone().unwrap().sg_type.unwrap().to_ascii_lowercase();
                            // Используем Unicode-символ вместо точки
                            let b_or_d_val = entity.material.clone().unwrap().b_or_d.unwrap_or(0.0);
                            let h_or_d_val = entity.material.clone().unwrap().h_or_d.unwrap_or(0.0);
                            
                            // Форматируем с заменой точки на Unicode-символ
                            let b_or_d_str = format!("{}", b_or_d_val).replace(".", ".");
                            let h_or_d_str = format!("{}", h_or_d_val).replace(".", ".");
                            
                            face_entity.common.layer = format!("{}-{}-{}", sg_type, b_or_d_str, h_or_d_str);
                            if !sum_layers.contains(&format!("{}-{}-{}", sg_type, b_or_d_str, h_or_d_str)){
                                sum_layers.push(format!("{}-{}-{}", sg_type, b_or_d_str, h_or_d_str));
                            }
                        } else {
                            let material_num = entity.material.clone().unwrap().material_num.unwrap_or(0);
                            // Если h уже строка, просто используем её или заменяем точки
                            let h_val = entity.material.clone().unwrap().h.unwrap_or(0.0);
                            let h_str = format!("{}", h_val).replace(".", ".");
                            
                            face_entity.common.layer = format!("{}-{}", material_num, h_str);
                            if !sum_layers.contains(&format!("{}-{}", material_num, h_str)){
                                sum_layers.push(format!("{}-{}", material_num, h_str));
                            }
                        }
                        face_entity.common.color = Color::from_index(1); // 1 - красный цвет в DXF
                        changed_elements_block.entities.push(face_entity);
                        // Добавляем только один текст с соответствующим значением as
                        if let Some(row) = &entity.row {
                            let center_x = (entity.vertices[0].x + entity.vertices[2].x) / 2.0;
                            let center_y = (entity.vertices[0].y + entity.vertices[2].y) / 2.0;
                            let z = entity.vertices[0].z;
                            // Выбираем только один параметр в зависимости от индекса файла
                            let as_value = match as_index {
                                0 => row.as1[0],
                                1 => row.as2[0],
                                2 => row.as3[0],
                                3 => row.as4[0],
                                _ => 0.0,
                            };
                            let as_name = format!("as{}", as_index + 1);
                            changed_elements_block.entities.push(create_text_entity(
                                center_x, center_y, z, 0.0, 
                                format!("{}:{:.1}", as_name, as_value), 2
                            ));
                        }
                    },
                    3 => {
                        let face3d = Face3D::new(
                            Point::new(entity.vertices[0].x, entity.vertices[0].y, entity.vertices[0].z),
                            Point::new(entity.vertices[1].x, entity.vertices[1].y, entity.vertices[1].z),
                            Point::new(entity.vertices[2].x, entity.vertices[2].y, entity.vertices[2].z),
                            Point::new(entity.vertices[0].x, entity.vertices[0].y, entity.vertices[0].z),
                        );
                        let mut face_entity = Entity::new(EntityType::Face3D(face3d));
						if entity.material.as_ref().unwrap().sg_type!=None{
                            let sg_type = entity.material.clone().unwrap().sg_type.unwrap().to_ascii_lowercase();
                            // Используем Unicode-символ вместо точки
                            let b_or_d_val = entity.material.clone().unwrap().b_or_d.unwrap_or(0.0);
                            let h_or_d_val = entity.material.clone().unwrap().h_or_d.unwrap_or(0.0);
                            
                            // Форматируем с заменой точки на Unicode-символ
                            let b_or_d_str = format!("{}", b_or_d_val).replace(".", ".");
                            let h_or_d_str = format!("{}", h_or_d_val).replace(".", ".");
                            
                            face_entity.common.layer = format!("{}-{}-{}", sg_type, b_or_d_str, h_or_d_str);
                            if !sum_layers.contains(&format!("{}-{}-{}", sg_type, b_or_d_str, h_or_d_str)){
                                sum_layers.push(format!("{}-{}-{}", sg_type, b_or_d_str, h_or_d_str));
                            }
                        } else {
                            let material_num = entity.material.clone().unwrap().material_num.unwrap_or(0);
                            // Если h уже строка, просто используем её или заменяем точки
                            let h_val = entity.material.clone().unwrap().h.unwrap_or(0.0);
                            let h_str = format!("{}", h_val).replace(".", ".");
                            
                            face_entity.common.layer = format!("{}-{}", material_num, h_str);
                            if !sum_layers.contains(&format!("{}-{}", material_num, h_str)){
                                sum_layers.push(format!("{}-{}", material_num, h_str));
                            }
                        }
                        face_entity.common.color = Color::from_index(1); // 1 - красный цвет в DXF
                        changed_elements_block.entities.push(face_entity);
                        // Добавляем только один текст с соответствующим значением as
                        if let Some(row) = &entity.row {
                            let center_x = (entity.vertices[0].x + entity.vertices[1].x + entity.vertices[2].x) / 3.0;
                            let center_y = (entity.vertices[0].y + entity.vertices[1].y + entity.vertices[2].y) / 3.0;
                            let z = entity.vertices[0].z;
                            // Выбираем только один параметр в зависимости от индекса файла
                            let as_value = match as_index {
                                0 => row.as1[0],
                                1 => row.as2[0],
                                2 => row.as3[0],
                                3 => row.as4[0],
                                _ => 0.0,
                            };
                            let as_name = format!("as{}", as_index + 1);
                            changed_elements_block.entities.push(create_text_entity(
                                center_x, center_y, z, 0.0, 
                                format!("{}:{:.1}", as_name, as_value), 2
                            ));
                        }
                    },
                    2 => {
                        let line = Line::new(
                            Point::new(entity.vertices[0].x, entity.vertices[0].y, entity.vertices[0].z),
                            Point::new(entity.vertices[1].x, entity.vertices[1].y, entity.vertices[1].z),
                        );
                        let mut face_entity = Entity::new(EntityType::Line(line));
						if entity.material.as_ref().unwrap().sg_type!=None{
                            let sg_type = entity.material.clone().unwrap().sg_type.unwrap().to_ascii_lowercase();
                            // Используем Unicode-символ вместо точки
                            let b_or_d_val = entity.material.clone().unwrap().b_or_d.unwrap_or(0.0);
                            let h_or_d_val = entity.material.clone().unwrap().h_or_d.unwrap_or(0.0);
                            
                            // Форматируем с заменой точки на Unicode-символ
                            let b_or_d_str = format!("{}", b_or_d_val).replace(".", ".");
                            let h_or_d_str = format!("{}", h_or_d_val).replace(".", ".");
                            
                            face_entity.common.layer = format!("{}-{}-{}", sg_type, b_or_d_str, h_or_d_str);
                            if !sum_layers.contains(&format!("{}-{}-{}", sg_type, b_or_d_str, h_or_d_str)){
                                sum_layers.push(format!("{}-{}-{}", sg_type, b_or_d_str, h_or_d_str));
                            }
                        } else {
                            let material_num = entity.material.clone().unwrap().material_num.unwrap_or(0);
                            // Если h уже строка, просто используем её или заменяем точки
                            let h_val = entity.material.clone().unwrap().h.unwrap_or(0.0);
                            let h_str = format!("{}", h_val).replace(".", ".");
                            
                            face_entity.common.layer = format!("{}-{}", material_num, h_str);
                            if !sum_layers.contains(&format!("{}-{}", material_num, h_str)){
                                sum_layers.push(format!("{}-{}", material_num, h_str));
                            }
                        }
                        face_entity.common.color = Color::from_index(1); // 1 - красный цвет в DXF
                        changed_elements_block.entities.push(face_entity);
                    },
                    _ => {}
                }
            }
        }
    }
    // Добавляем блок в чертеж, если в нем есть элементы
    if !changed_elements_block.entities.is_empty() {
        drawing.add_block(changed_elements_block);
        // Создаем вставку блока и добавляем ее в чертеж
        let insert = Insert {
            name: String::from(&block_name),
            location: Point::new(0.0, 0.0, 0.0),
            x_scale_factor: 1.0,
            y_scale_factor: 1.0,
            z_scale_factor: 1.0,
            rotation: 0.0,
            column_count: 1,
            row_count: 1,
            column_spacing: 0.0,
            row_spacing: 0.0,
            __seqend_handle: dxf::Handle(0),
            __has_attributes: false,
            extrusion_direction: Vector::z_axis(),
            __attributes_and_handles: Vec::new(),
        };
        drawing.add_entity(Entity::new(EntityType::Insert(insert)));
		// for layer_item in sum_layers{
		// 	let layer =Layer {
		// 			name:layer_item,
		// 			color: Color::from_index(2),
		// 			..Default::default()
		// 		};
				
		// 	drawing.add_layer(layer);
		// }
    }
    // Теперь добавляем все неизмененные элементы напрямую в чертеж
    for (_z, entities) in data.iter() {
        for entity in entities {
            if !entity.changed {
                match entity.vertices.len() {
                    4 => {
                        let face3d = Face3D::new(
                            Point::new(entity.vertices[0].x, entity.vertices[0].y, entity.vertices[0].z),
                            Point::new(entity.vertices[1].x, entity.vertices[1].y, entity.vertices[1].z),
                            Point::new(entity.vertices[2].x, entity.vertices[2].y, entity.vertices[2].z),
                            Point::new(entity.vertices[3].x, entity.vertices[3].y, entity.vertices[3].z),
                        );
                        drawing.add_entity(Entity::new(EntityType::Face3D(face3d)));
                        // Добавляем только один текст с соответствующим значением as
                        if let Some(row) = &entity.row {
                            let center_x = (entity.vertices[0].x + entity.vertices[2].x) / 2.0;
                            let center_y = (entity.vertices[0].y + entity.vertices[2].y) / 2.0;
                            let z = entity.vertices[0].z;
                            // Выбираем только один параметр в зависимости от индекса файла
                            let as_value = match as_index {
                                0 => row.as1[0],
                                1 => row.as2[0],
                                2 => row.as3[0],
                                3 => row.as4[0],
                                _ => 0.0,
                            };
                            let as_name = format!("as{}", as_index + 1);
                            drawing.add_entity(create_text_entity(
                                center_x, center_y, z, 0.0, 
                                format!("{}:{:.1}", as_name, as_value), 0
                            ));
                        }
                    },
                    3 => {
                        let face3d = Face3D::new(
                            Point::new(entity.vertices[0].x, entity.vertices[0].y, entity.vertices[0].z),
                            Point::new(entity.vertices[1].x, entity.vertices[1].y, entity.vertices[1].z),
                            Point::new(entity.vertices[2].x, entity.vertices[2].y, entity.vertices[2].z),
                            Point::new(entity.vertices[0].x, entity.vertices[0].y, entity.vertices[0].z),
                        );
                        drawing.add_entity(Entity::new(EntityType::Face3D(face3d)));
                        // Добавляем только один текст с соответствующим значением as
                        if let Some(row) = &entity.row {
                            let center_x = (entity.vertices[0].x + entity.vertices[1].x + entity.vertices[2].x) / 3.0;
                            let center_y = (entity.vertices[0].y + entity.vertices[1].y + entity.vertices[2].y) / 3.0;
                            let z = entity.vertices[0].z;
                            // Выбираем только один параметр в зависимости от индекса файла
                            let as_value = match as_index {
                                0 => row.as1[0],
                                1 => row.as2[0],
                                2 => row.as3[0],
                                3 => row.as4[0],
                                _ => 0.0,
                            };
                            
                            let as_name = format!("as{}", as_index + 1);
                            drawing.add_entity(create_text_entity(
                                center_x, center_y, z, 0.0, 
                                format!("{}:{:.1}", as_name, as_value), 0
                            ));
                        }
                    },
                    2 => {
                        let line = Line::new(
                            Point::new(entity.vertices[0].x, entity.vertices[0].y, entity.vertices[0].z),
                            Point::new(entity.vertices[1].x, entity.vertices[1].y, entity.vertices[1].z),
                        );
                        drawing.add_entity(Entity::new(EntityType::Line(line)));
                    },
                    _ => {}
                }
            }
        }
    }

    let mut buffer = Cursor::new(Vec::new());
    drawing.save(&mut buffer).expect("Ошибка записи DXF");
    buffer.into_inner()
}

pub fn create_simple_dxf_with_block() -> Vec<u8> {
    let mut drawing = Drawing::new();
    // Создаем блок
    let block_name = "SIMPLE_BLOCK";
    let mut simple_block = Block {
        name: String::from(block_name),
        base_point: Point::new(0.0, 0.0, 0.0),
        entities: Vec::new(),
        layer: String::from("0"),
        description:  String::new(),
        xref_path_name: String::new(),
        handle: dxf::Handle(0),
        __owner_handle: dxf::Handle(0),
        flags: 0,
        is_in_paperspace: false,
        extension_data_groups: Vec::new(),
        x_data: Vec::new(),
    };
    // Добавляем линии в блок
    let line1 = Line::new(
        Point::new(0.0, 0.0, 0.0),
        Point::new(1.0, 1.0, 0.0),
    );
    let mut line1_entity = Entity::new(EntityType::Line(line1));
    line1_entity.common.color = Color::from_index(1); // Красный
    simple_block.entities.push(line1_entity);
    let line2 = Line::new(
        Point::new(0.0, 1.0, 0.0),
        Point::new(1.0, 0.0, 0.0),
    );
    let mut line2_entity = Entity::new(EntityType::Line(line2));
    line2_entity.common.color = Color::from_index(2); // Желтый
    simple_block.entities.push(line2_entity);
    // Добавляем 3D грани в блок
    let face1 = Face3D::new(
        Point::new(0.0, 0.0, 0.0),
        Point::new(1.0, 0.0, 0.0),
        Point::new(1.0, 1.0, 0.0),
        Point::new(0.0, 1.0, 0.0),
    );
    let mut face1_entity = Entity::new(EntityType::Face3D(face1));
    face1_entity.common.color = Color::from_index(3); // Зеленый
    simple_block.entities.push(face1_entity);
    
    let face2 = Face3D::new(
        Point::new(0.0, 0.0, 0.0),
        Point::new(1.0, 0.0, 0.0),
        Point::new(1.0, 0.0, 1.0),
        Point::new(0.0, 0.0, 1.0),
    );
    let mut face2_entity = Entity::new(EntityType::Face3D(face2));
    face2_entity.common.color = Color::from_index(4); // Голубой
    simple_block.entities.push(face2_entity);
    
    // Добавляем блок в чертеж
    drawing.add_block(simple_block);
    
    // Создаем вставку блока
    let insert = Insert {
        name: String::from(block_name),
        location: Point::new(0.0, 0.0, 0.0),
        x_scale_factor: 1.0,
        y_scale_factor: 1.0,
        z_scale_factor: 1.0,
        rotation: 0.0,
        column_count: 1,
        row_count: 1,
        column_spacing: 0.0,
        row_spacing: 0.0,
        __seqend_handle: dxf::Handle(0),
        __has_attributes: false,
        extrusion_direction: Vector::z_axis(),
        __attributes_and_handles: Vec::new(),
    };
    drawing.add_entity(Entity::new(EntityType::Insert(insert)));
    
    // Добавляем еще одну вставку блока с другими параметрами
    let insert2 = Insert {
        name: String::from(block_name),
        location: Point::new(3.0, 3.0, 0.0),
        x_scale_factor: 2.0,
        y_scale_factor: 2.0,
        z_scale_factor: 2.0,
        rotation: 45.0,
        column_count: 1,
        row_count: 1,
        column_spacing: 0.0,
        row_spacing: 0.0,
        __seqend_handle: dxf::Handle(0),
        __has_attributes: false,
        extrusion_direction: Vector::z_axis(),
        __attributes_and_handles: Vec::new(),
    };
    drawing.add_entity(Entity::new(EntityType::Insert(insert2)));
    
    // Сохраняем в буфер
    let mut buffer = Cursor::new(Vec::new());
    drawing.save(&mut buffer).expect("Ошибка записи DXF");
    
    buffer.into_inner()
}


pub fn create_dxf_for_specific_as_manual1(data: HashMap<OrderedFloat<f32>, Vec<EntityWithXlsx>>, as_index: usize) -> Vec<u8> {
    // Начинаем создавать DXF файл как текст
    let mut dxf_content = String::new();
    
    // Заголовок DXF файла
    dxf_content.push_str("0\nSECTION\n2\nHEADER\n");
    dxf_content.push_str("9\n$ACADVER\n1\nAC1009\n"); // Версия AutoCAD 2007 (R14)
    dxf_content.push_str("9\n$INSBASE\n10\n0.0\n20\n0.0\n30\n0.0\n");
    // Другие параметры заголовка
    dxf_content.push_str("0\nENDSEC\n");
    
    // Секция таблиц
    dxf_content.push_str("0\nSECTION\n2\nTABLES\n");
    
    // Таблица слоев
    dxf_content.push_str("0\nTABLE\n2\nLAYER\n70\n0\n");
    
    // Слой 0 (обязательный)
    dxf_content.push_str("0\nLAYER\n2\n0\n70\n0\n62\n7\n6\nCONTINUOUS\n");
    
    // Собираем все уникальные слои
    let mut layers = HashSet::new();
    
    for (_z, entities) in data.iter() {
        for entity in entities {
            if entity.changed {
                if entity.material.as_ref().unwrap().sg_type != None {
                    let sg_type = entity.material.clone().unwrap().sg_type.unwrap().to_ascii_lowercase();
                    let b_or_d_val = entity.material.clone().unwrap().b_or_d.unwrap_or(0.0);
                    let h_or_d_val = entity.material.clone().unwrap().h_or_d.unwrap_or(0.0);
                    
                    // Используем формат с заменой точки на дефис
                    let b_or_d_str = format!("{}", b_or_d_val).replace(".", "-");
                    let h_or_d_str = format!("{}", h_or_d_val).replace(".", "-");
                    
                    let layer_name = format!("{}-{}-{}", sg_type, b_or_d_str, h_or_d_str);
                    layers.insert(layer_name);
                } else {
                    let material_num = entity.material.clone().unwrap().material_num.unwrap_or(0);
                    let h_val = entity.material.clone().unwrap().h.unwrap_or(0.0);
                    
                    let h_str = format!("{}", h_val).replace(".", "-");
                    let layer_name = format!("{}-{}", material_num, h_str);
                    layers.insert(layer_name);
                }
            }
        }
    }
    
    // Добавляем все слои в таблицу
    for (i, layer_name) in layers.iter().enumerate() {
        dxf_content.push_str(&format!("0\nLAYER\n5\n{}\n100\nAcDbSymbolTableRecord\n100\nAcDbLayerTableRecord\n2\n{}\n70\n0\n62\n7\n6\nCONTINUOUS\n", 
            i + 10, layer_name));
    }
    
    dxf_content.push_str("0\nENDTAB\n");
    
    // Другие таблицы (стили текста и т.д.)
    dxf_content.push_str("0\nTABLE\n2\nSTYLE\n70\n0\n");
    dxf_content.push_str("0\nSTYLE\n2\nSTANDARD\n70\n0\n40\n0.0\n41\n1.0\n50\n0.0\n71\n0\n42\n0.2\n3\ntxt\n4\n\n0\nENDTAB\n");
    
    // Завершаем секцию таблиц
    dxf_content.push_str("0\nENDSEC\n");
    
    // Начинаем секцию блоков
    dxf_content.push_str("0\nSECTION\n2\nBLOCKS\n");
    
    // Создаем блок для измененных элементов
    let block_name = format!("CHANGED_ELEMENTS_AS{}", as_index + 1);
    dxf_content.push_str(&format!("0\nBLOCK\n8\n0\n2\n{}\n70\n0\n10\n0.0\n20\n0.0\n30\n0.0\n3\n{}\n1\n\n", 
        block_name, block_name));
    
    // Добавляем все измененные элементы
    let mut entity_counter = 0;
    for (_z, entities) in data.iter() {
        for entity in entities {
            if entity.changed {
                match entity.vertices.len() {
                    4 => {
                        // Создаем 3DFACE
                        entity_counter += 1;
                        
                        // Определяем слой
                        let layer_name = if entity.material.as_ref().unwrap().sg_type != None {
                            let sg_type = entity.material.clone().unwrap().sg_type.unwrap().to_ascii_lowercase();
                            let b_or_d_val = entity.material.clone().unwrap().b_or_d.unwrap_or(0.0);
                            let h_or_d_val = entity.material.clone().unwrap().h_or_d.unwrap_or(0.0);
                            
                            let b_or_d_str = format!("{}", b_or_d_val).replace(".", "-");
                            let h_or_d_str = format!("{}", h_or_d_val).replace(".", "-");
                            
                            format!("{}-{}-{}", sg_type, b_or_d_str, h_or_d_str)
                        } else {
                            let material_num = entity.material.clone().unwrap().material_num.unwrap_or(0);
                            let h_val = entity.material.clone().unwrap().h.unwrap_or(0.0);
                            
                            let h_str = format!("{}", h_val).replace(".", "-");
                            format!("{}-{}", material_num, h_str)
                        };
                        
                        // Добавляем 3DFACE
                        dxf_content.push_str(&format!("0\n3DFACE\n5\n{:X}\n8\n{}\n62\n1\n10\n{}\n20\n{}\n30\n{}\n11\n{}\n21\n{}\n31\n{}\n12\n{}\n22\n{}\n32\n{}\n13\n{}\n23\n{}\n33\n{}\n",
                            entity_counter,
                            layer_name,
                            entity.vertices[0].x, entity.vertices[0].y, entity.vertices[0].z,
                            entity.vertices[1].x, entity.vertices[1].y, entity.vertices[1].z,
                            entity.vertices[2].x, entity.vertices[2].y, entity.vertices[2].z,
                            entity.vertices[3].x, entity.vertices[3].y, entity.vertices[3].z
                        ));
                        
                        // Добавляем текст с as значением
                        if let Some(row) = &entity.row {
                            entity_counter += 1;
                            let center_x = (entity.vertices[0].x + entity.vertices[2].x) / 2.0;
                            let center_y = (entity.vertices[0].y + entity.vertices[2].y) / 2.0;
                            let z = entity.vertices[0].z;
                            
                            let as_value = match as_index {
                                0 => row.as1[0],
                                1 => row.as2[0],
                                2 => row.as3[0],
                                3 => row.as4[0],
                                _ => 0.0,
                            };
                            
                            let as_name = format!("as{}", as_index + 1);
                            dxf_content.push_str(&format!("0\nTEXT\n5\n{:X}\n8\n0\n62\n2\n10\n{}\n20\n{}\n30\n{}\n40\n0.05\n1\n{}:{:.1}\n",
                                entity_counter,
                                center_x, center_y, z,
                                as_name, as_value
                            ));
                        }
                    },
                    3 => {
                        // Создаем 3DFACE для треугольника
                        entity_counter += 1;
                        
                        // Определяем слой
                        let layer_name = if entity.material.as_ref().unwrap().sg_type != None {
                            let sg_type = entity.material.clone().unwrap().sg_type.unwrap().to_ascii_lowercase();
                            let b_or_d_val = entity.material.clone().unwrap().b_or_d.unwrap_or(0.0);
                            let h_or_d_val = entity.material.clone().unwrap().h_or_d.unwrap_or(0.0);
                            
                            let b_or_d_str = format!("{}", b_or_d_val).replace(".", "-");
                            let h_or_d_str = format!("{}", h_or_d_val).replace(".", "-");
                            
                            format!("{}-{}-{}", sg_type, b_or_d_str, h_or_d_str)
                        } else {
                            let material_num = entity.material.clone().unwrap().material_num.unwrap_or(0);
                            let h_val = entity.material.clone().unwrap().h.unwrap_or(0.0);
                            
                            let h_str = format!("{}", h_val).replace(".", "-");
                            format!("{}-{}", material_num, h_str)
                        };
                        
                        // Добавляем 3DFACE (повторяем первую вершину в качестве четвертой)
                        dxf_content.push_str(&format!("0\n3DFACE\n5\n{:X}\n8\n{}\n62\n1\n10\n{}\n20\n{}\n30\n{}\n11\n{}\n21\n{}\n31\n{}\n12\n{}\n22\n{}\n32\n{}\n13\n{}\n23\n{}\n33\n{}\n",
                            entity_counter,
                            layer_name,
                            entity.vertices[0].x, entity.vertices[0].y, entity.vertices[0].z,
                            entity.vertices[1].x, entity.vertices[1].y, entity.vertices[1].z,
                            entity.vertices[2].x, entity.vertices[2].y, entity.vertices[2].z,
                            entity.vertices[0].x, entity.vertices[0].y, entity.vertices[0].z
                        ));
                        
                        // Добавляем текст с as значением
                        if let Some(row) = &entity.row {
                            entity_counter += 1;
                            let center_x = (entity.vertices[0].x + entity.vertices[1].x + entity.vertices[2].x) / 3.0;
                            let center_y = (entity.vertices[0].y + entity.vertices[1].y + entity.vertices[2].y) / 3.0;
                            let z = entity.vertices[0].z;
                            
                            let as_value = match as_index {
                                0 => row.as1[0],
                                1 => row.as2[0],
                                2 => row.as3[0],
                                3 => row.as4[0],
                                _ => 0.0,
                            };
                            
                            let as_name = format!("as{}", as_index + 1);
                            dxf_content.push_str(&format!("0\nTEXT\n5\n{:X}\n8\n0\n62\n2\n10\n{}\n20\n{}\n30\n{}\n40\n0.05\n1\n{}:{:.1}\n",
                                entity_counter,
                                center_x, center_y, z,
                                as_name, as_value
                            ));
                        }
                    },
                    2 => {
                        // Создаем LINE
                        entity_counter += 1;
                        
                        // Определяем слой
                        let layer_name = if entity.material.as_ref().unwrap().sg_type != None {
                            let sg_type = entity.material.clone().unwrap().sg_type.unwrap().to_ascii_lowercase();
                            let b_or_d_val = entity.material.clone().unwrap().b_or_d.unwrap_or(0.0);
                            let h_or_d_val = entity.material.clone().unwrap().h_or_d.unwrap_or(0.0);
                            
                            let b_or_d_str = format!("{}", b_or_d_val).replace(".", "-");
                            let h_or_d_str = format!("{}", h_or_d_val).replace(".", "-");
                            
                            format!("{}-{}-{}", sg_type, b_or_d_str, h_or_d_str)
                        } else {
                            let material_num = entity.material.clone().unwrap().material_num.unwrap_or(0);
                            let h_val = entity.material.clone().unwrap().h.unwrap_or(0.0);
                            
                            let h_str = format!("{}", h_val).replace(".", "-");
                            format!("{}-{}", material_num, h_str)
                        };
                        
                        // Добавляем LINE
                        dxf_content.push_str(&format!("0\nLINE\n5\n{:X}\n8\n{}\n62\n1\n10\n{}\n20\n{}\n30\n{}\n11\n{}\n21\n{}\n31\n{}\n",
                            entity_counter,
                            layer_name,
                            entity.vertices[0].x, entity.vertices[0].y, entity.vertices[0].z,
                            entity.vertices[1].x, entity.vertices[1].y, entity.vertices[1].z
                        ));
                    },
                    _ => {}
                }
            }
        }
    }
    
    // Завершаем блок
    dxf_content.push_str("0\nENDBLK\n");
    dxf_content.push_str("0\nENDSEC\n");
    
    // Секция сущностей
    dxf_content.push_str("0\nSECTION\n2\nENTITIES\n");
    
    // Вставляем ссылку на блок
    dxf_content.push_str(&format!("0\nINSERT\n8\n0\n2\n{}\n10\n0.0\n20\n0.0\n30\n0.0\n41\n1.0\n42\n1.0\n43\n1.0\n50\n0.0\n",
        block_name
    ));
    
    // Завершаем секцию сущностей и файл
    dxf_content.push_str("0\nENDSEC\n0\nEOF\n");
    
    // Возвращаем содержимое как массив байтов
    dxf_content.into_bytes()
}


// pub fn create_dxf_for_specific_as_manual(data: HashMap<OrderedFloat<f32>, Vec<EntityWithXlsx>>, as_index: usize) -> Vec<u8> {
//     // Создаем новый DXF файл вручную
//     let mut dxf_content = String::new();
    
//     // Заголовок DXF файла
//     dxf_content.push_str("0\nSECTION\n2\nHEADER\n");
//     dxf_content.push_str("9\n$ACADVER\n1\nAC1009\n");
//     dxf_content.push_str("0\nENDSEC\n");
    
//     // Секция таблиц
//     dxf_content.push_str("0\nSECTION\n2\nTABLES\n");
    
//     // Таблица слоев
//     dxf_content.push_str("0\nTABLE\n2\nLAYER\n");
//     dxf_content.push_str("0\nLAYER\n2\n0\n70\n0\n62\n7\n6\nCONTINUOUS\n");
//     dxf_content.push_str("0\nENDTAB\n");
    
//     // Конец секции таблиц
//     dxf_content.push_str("0\nENDSEC\n");
    
//     // Секция блоков
//     dxf_content.push_str("0\nSECTION\n2\nBLOCKS\n");
    
//     // Группируем измененные элементы по координатам X,Y
//     let mut xy_groups: HashMap<String, Vec<&EntityWithXlsx>> = HashMap::new();
    
//     for (_z, entities) in data.iter() {
//         for entity in entities {
//             if entity.changed && !entity.vertices.is_empty() {
//                 // Вычисляем центр элемента по X,Y
//                 let mut center_x = 0.0;
//                 let mut center_y = 0.0;
                
//                 for vertex in &entity.vertices {
//                     center_x += vertex.x;
//                     center_y += vertex.y;
//                 }
                
//                 center_x /= entity.vertices.len() as f64;
//                 center_y /= entity.vertices.len() as f64;
                
//                 // Форматируем координаты для имени блока
//                 // Заменяем точку на подчеркивание для имени блока
//                 let x_str = format!("{:.1}", center_x).replace(".", "_");
//                 let y_str = format!("{:.1}", center_y).replace(".", "_");
//                 let key = format!("x{}_y{}", x_str, y_str);
                
//                 // Добавляем элемент в соответствующую группу
//                 xy_groups.entry(key).or_insert_with(Vec::new).push(entity);
//             }
//         }
//     }
    
//     // Создаем блок для каждой группы элементов с одинаковыми X,Y
//     for (key, entities) in &xy_groups {
//         // Начало определения блока
//         dxf_content.push_str(&format!("0\nBLOCK\n2\n{}\n", key));
//         dxf_content.push_str("70\n0\n10\n0.0\n20\n0.0\n30\n0.0\n");
        
//         // Добавляем элементы в блок
//         for entity in entities {
//             match entity.vertices.len() {
//                 4 => {
//                     // 3DFACE для четырехугольника
//                     dxf_content.push_str("0\n3DFACE\n");
//                     dxf_content.push_str("8\n0\n"); // Слой
//                     dxf_content.push_str("62\n1\n"); // Красный цвет
                    
//                     // Координаты вершин
//                     dxf_content.push_str(&format!("10\n{}\n20\n{}\n30\n{}\n", 
//                         entity.vertices[0].x, entity.vertices[0].y, entity.vertices[0].z));
//                     dxf_content.push_str(&format!("11\n{}\n21\n{}\n31\n{}\n", 
//                         entity.vertices[1].x, entity.vertices[1].y, entity.vertices[1].z));
//                     dxf_content.push_str(&format!("12\n{}\n22\n{}\n32\n{}\n", 
//                         entity.vertices[2].x, entity.vertices[2].y, entity.vertices[2].z));
//                     dxf_content.push_str(&format!("13\n{}\n23\n{}\n33\n{}\n", 
//                         entity.vertices[3].x, entity.vertices[3].y, entity.vertices[3].z));
                    
//                     // Добавляем текст с значением as
//                     if let Some(row) = &entity.row {
//                         let center_x = (entity.vertices[0].x + entity.vertices[2].x) / 2.0;
//                         let center_y = (entity.vertices[0].y + entity.vertices[2].y) / 2.0;
//                         let z = entity.vertices[0].z;
                        
//                         // Выбираем параметр в зависимости от индекса
//                         let as_value = match as_index {
//                             0 => row.as1[0],
//                             1 => row.as2[0],
//                             2 => row.as3[0],
//                             3 => row.as4[0],
//                             _ => 0.0,
//                         };
                        
//                         let as_name = format!("as{}", as_index + 1);
//                         let text = format!("{}:{:.1}", as_name, as_value);
                        
//                         // Добавляем текст
//                         dxf_content.push_str("0\nTEXT\n");
//                         dxf_content.push_str("8\n0\n"); // Слой
//                         dxf_content.push_str("62\n2\n"); // Зеленый цвет для текста
//                         dxf_content.push_str(&format!("10\n{}\n20\n{}\n30\n{}\n", center_x, center_y, z));
//                         dxf_content.push_str("40\n0.05\n"); // Высота текста
//                         dxf_content.push_str(&format!("1\n{}\n", text));
//                         dxf_content.push_str("50\n0.0\n"); // Угол поворота
//                     }
//                 },
//                 3 => {
//                     // 3DFACE для треугольника
//                     dxf_content.push_str("0\n3DFACE\n");
//                     dxf_content.push_str("8\n0\n"); // Слой
//                     dxf_content.push_str("62\n1\n"); // Красный цвет
                    
//                     // Координаты вершин
//                     dxf_content.push_str(&format!("10\n{}\n20\n{}\n30\n{}\n", 
//                         entity.vertices[0].x, entity.vertices[0].y, entity.vertices[0].z));
//                     dxf_content.push_str(&format!("11\n{}\n21\n{}\n31\n{}\n", 
//                         entity.vertices[1].x, entity.vertices[1].y, entity.vertices[1].z));
//                     dxf_content.push_str(&format!("12\n{}\n22\n{}\n32\n{}\n", 
//                         entity.vertices[2].x, entity.vertices[2].y, entity.vertices[2].z));
//                     dxf_content.push_str(&format!("13\n{}\n23\n{}\n33\n{}\n", 
//                         entity.vertices[0].x, entity.vertices[0].y, entity.vertices[0].z));
                    
//                     // Добавляем текст с значением as
//                     if let Some(row) = &entity.row {
//                         let center_x = (entity.vertices[0].x + entity.vertices[1].x + entity.vertices[2].x) / 3.0;
//                         let center_y = (entity.vertices[0].y + entity.vertices[1].y + entity.vertices[2].y) / 3.0;
//                         let z = entity.vertices[0].z;
                        
//                         // Выбираем параметр в зависимости от индекса
//                         let as_value = match as_index {
//                             0 => row.as1[0],
//                             1 => row.as2[0],
//                             2 => row.as3[0],
//                             3 => row.as4[0],
//                             _ => 0.0,
//                         };
                        
//                         let as_name = format!("as{}", as_index + 1);
//                         let text = format!("{}:{:.1}", as_name, as_value);
                        
//                         // Добавляем текст
//                         dxf_content.push_str("0\nTEXT\n");
//                         dxf_content.push_str("8\n0\n"); // Слой
//                         dxf_content.push_str("62\n2\n"); // Зеленый цвет для текста
//                         dxf_content.push_str(&format!("10\n{}\n20\n{}\n30\n{}\n", center_x, center_y, z));
//                         dxf_content.push_str("40\n0.05\n"); // Высота текста
//                         dxf_content.push_str(&format!("1\n{}\n", text));
//                         dxf_content.push_str("50\n0.0\n"); // Угол поворота
//                     }
//                 },
//                 2 => {
//                     // LINE для линии
//                     dxf_content.push_str("0\nLINE\n");
//                     dxf_content.push_str("8\n0\n"); // Слой
//                     dxf_content.push_str("62\n1\n"); // Красный цвет
                    
//                     // Координаты начала и конца
//                     dxf_content.push_str(&format!("10\n{}\n20\n{}\n30\n{}\n", 
//                         entity.vertices[0].x, entity.vertices[0].y, entity.vertices[0].z));
//                     dxf_content.push_str(&format!("11\n{}\n21\n{}\n31\n{}\n", 
//                         entity.vertices[1].x, entity.vertices[1].y, entity.vertices[1].z));
//                 },
//                 _ => {}
//             }
//         }
        
//         // Конец определения блока
//         dxf_content.push_str("0\nENDBLK\n");
//     }
    
//     // Конец секции блоков
//     dxf_content.push_str("0\nENDSEC\n");
    
//     // Секция сущностей
//     dxf_content.push_str("0\nSECTION\n2\nENTITIES\n");
    
//     // Вставляем блоки
//     for key in xy_groups.keys() {
//         dxf_content.push_str("0\nINSERT\n");
//         dxf_content.push_str(&format!("2\n{}\n", key));
//         dxf_content.push_str("8\n0\n"); // Слой
//         dxf_content.push_str("10\n0.0\n20\n0.0\n30\n0.0\n"); // Точка вставки
//         dxf_content.push_str("41\n1.0\n42\n1.0\n43\n1.0\n"); // Масштаб
//         dxf_content.push_str("50\n0.0\n"); // Угол поворота
//     }
    
//     // Добавляем неизмененные элементы напрямую
//     for (_z, entities) in data.iter() {
//         for entity in entities {
//             if !entity.changed {
//                 match entity.vertices.len() {
//                     4 => {
//                         // 3DFACE для четырехугольника
//                         dxf_content.push_str("0\n3DFACE\n");
//                         dxf_content.push_str("8\n0\n"); // Слой
                        
//                         // Координаты вершин
//                         dxf_content.push_str(&format!("10\n{}\n20\n{}\n30\n{}\n", 
//                             entity.vertices[0].x, entity.vertices[0].y, entity.vertices[0].z));
//                         dxf_content.push_str(&format!("11\n{}\n21\n{}\n31\n{}\n", 
//                             entity.vertices[1].x, entity.vertices[1].y, entity.vertices[1].z));
//                         dxf_content.push_str(&format!("12\n{}\n22\n{}\n32\n{}\n", 
//                             entity.vertices[2].x, entity.vertices[2].y, entity.vertices[2].z));
//                         dxf_content.push_str(&format!("13\n{}\n23\n{}\n33\n{}\n", 
//                             entity.vertices[3].x, entity.vertices[3].y, entity.vertices[3].z));
                        
//                         // Добавляем текст с значением as
//                         if let Some(row) = &entity.row {
//                             let center_x = (entity.vertices[0].x + entity.vertices[2].x) / 2.0;
//                             let center_y = (entity.vertices[0].y + entity.vertices[2].y) / 2.0;
//                             let z = entity.vertices[0].z;
                            
//                             // Выбираем параметр в зависимости от индекса
//                             let as_value = match as_index {
//                                 0 => row.as1[0],
//                                 1 => row.as2[0],
//                                 2 => row.as3[0],
//                                 3 => row.as4[0],
//                                 _ => 0.0,
//                             };
                            
//                             let as_name = format!("as{}", as_index + 1);
//                             let text = format!("{}:{:.1}", as_name, as_value);
                            
//                             // Добавляем текст
//                             dxf_content.push_str("0\nTEXT\n");
//                             dxf_content.push_str("8\n0\n"); // Слой
//                             dxf_content.push_str(&format!("10\n{}\n20\n{}\n30\n{}\n", center_x, center_y, z));
//                             dxf_content.push_str("40\n0.05\n"); // Высота текста
//                             dxf_content.push_str(&format!("1\n{}\n", text));
//                             dxf_content.push_str("50\n0.0\n"); // Угол поворота
//                         }
//                     },
//                     3 => {
//                         // 3DFACE для треугольника
//                         dxf_content.push_str("0\n3DFACE\n");
//                         dxf_content.push_str("8\n0\n"); // Слой
                        
//                         // Координаты вершин
//                         dxf_content.push_str(&format!("10\n{}\n20\n{}\n30\n{}\n", 
//                             entity.vertices[0].x, entity.vertices[0].y, entity.vertices[0].z));
//                         dxf_content.push_str(&format!("11\n{}\n21\n{}\n31\n{}\n", 
//                             entity.vertices[1].x, entity.vertices[1].y, entity.vertices[1].z));
//                         dxf_content.push_str(&format!("12\n{}\n22\n{}\n32\n{}\n", 
//                             entity.vertices[2].x, entity.vertices[2].y, entity.vertices[2].z));
//                         dxf_content.push_str(&format!("13\n{}\n23\n{}\n33\n{}\n", 
//                             entity.vertices[0].x, entity.vertices[0].y, entity.vertices[0].z));
                        
//                         // Добавляем текст с значением as
//                         if let Some(row) = &entity.row {
//                             let center_x = (entity.vertices[0].x + entity.vertices[1].x + entity.vertices[2].x) / 3.0;
//                             let center_y = (entity.vertices[0].y + entity.vertices[1].y + entity.vertices[2].y) / 3.0;
//                             let z = entity.vertices[0].z;
                            
//                             // Выбираем параметр в зависимости от индекса
//                             let as_value = match as_index {
//                                 0 => row.as1[0],
//                                 1 => row.as2[0],
//                                 2 => row.as3[0],
//                                 3 => row.as4[0],
//                                 _ => 0.0,
//                             };
                            
//                             let as_name = format!("as{}", as_index + 1);
//                             let text = format!("{}:{:.1}", as_name, as_value);
                            
//                             // Добавляем текст
//                             dxf_content.push_str("0\nTEXT\n");
//                             dxf_content.push_str("8\n0\n"); // Слой
//                             dxf_content.push_str(&format!("10\n{}\n20\n{}\n30\n{}\n", center_x, center_y, z));
//                             dxf_content.push_str("40\n0.05\n"); // Высота текста
//                             dxf_content.push_str(&format!("1\n{}\n", text));
//                             dxf_content.push_str("50\n0.0\n"); // Угол поворота
//                         }
//                     },
//                     2 => {
//                         // LINE для линии
//                         dxf_content.push_str("0\nLINE\n");
//                         dxf_content.push_str("8\n0\n"); // Слой
                        
//                         // Координаты начала и конца
//                         dxf_content.push_str(&format!("10\n{}\n20\n{}\n30\n{}\n", 
//                             entity.vertices[0].x, entity.vertices[0].y, entity.vertices[0].z));
//                         dxf_content.push_str(&format!("11\n{}\n21\n{}\n31\n{}\n", 
//                             entity.vertices[1].x, entity.vertices[1].y, entity.vertices[1].z));
//                     },
//                     _ => {}
//                 }
//             }
//         }
//     }
    
//     // Конец секции сущностей
//     dxf_content.push_str("0\nENDSEC\n");
    
//     // Конец файла
//     dxf_content.push_str("0\nEOF\n");
    
//     // Возвращаем байты
//     dxf_content.into_bytes()
// }
pub fn create_dxf_for_specific_as_manual(data: HashMap<OrderedFloat<f32>, Vec<EntityWithXlsx>>, as_index: usize) -> Vec<u8> {
    // Создаем новый DXF файл вручную
    let mut dxf_content = String::new();
    
    // Заголовок DXF файла
    dxf_content.push_str("0\nSECTION\n2\nHEADER\n");
    dxf_content.push_str("9\n$ACADVER\n1\nAC1009\n");
    dxf_content.push_str("0\nENDSEC\n");
    
    // Собираем все уникальные слои
    let mut unique_layers = HashSet::new();
    unique_layers.insert("0".to_string()); // Добавляем слой по умолчанию
    
    // Проходим по всем элементам и собираем уникальные слои
    for (_z, entities) in data.iter() {
        for entity in entities {
            if let Some(material) = &entity.material {
                if let Some(sg_type) = &material.sg_type {
                    let sg_type = sg_type.to_ascii_lowercase();
                    let b_or_d_val = material.b_or_d.unwrap_or(0.0);
                    let h_or_d_val = material.h_or_d.unwrap_or(0.0);
                    
                    let b_or_d_str = format!("{}", b_or_d_val).replace(".", "_");
                    let h_or_d_str = format!("{}", h_or_d_val).replace(".", "_");
                    
                    let layer_name = format!("{}-{}-{}", sg_type, b_or_d_str, h_or_d_str);
                    unique_layers.insert(layer_name);
                } else if let Some(material_num) = material.material_num {
                    let layer_name = format!("material-{}", material_num);
                    unique_layers.insert(layer_name);
                }
            }
        }
    }
    
    // Секция таблиц
    dxf_content.push_str("0\nSECTION\n2\nTABLES\n");
    
    // Таблица слоев
    dxf_content.push_str("0\nTABLE\n2\nLAYER\n");
    
    // Добавляем все уникальные слои
    for layer_name in &unique_layers {
        dxf_content.push_str("0\nLAYER\n");
        dxf_content.push_str(&format!("2\n{}\n", layer_name));
        dxf_content.push_str("70\n0\n"); // Флаги слоя
        
        // Разные цвета для разных слоев (можно настроить по своему усмотрению)
        let color_index = if layer_name == "0" { 7 } else { 
            // Простой хеш для получения разных цветов
            ((layer_name.chars().fold(0, |acc, c| acc + c as u32)) % 7 + 1) as u8
        };
        
        dxf_content.push_str(&format!("62\n{}\n", color_index));
        dxf_content.push_str("6\nCONTINUOUS\n"); // Тип линии
    }
    
    dxf_content.push_str("0\nENDTAB\n");
    dxf_content.push_str("0\nENDSEC\n");
    
    // Секция блоков
    dxf_content.push_str("0\nSECTION\n2\nBLOCKS\n");
    
    // Группируем измененные элементы по координатам X,Y
    let mut xy_groups: HashMap<String, Vec<&EntityWithXlsx>> = HashMap::new();
    
    for (_z, entities) in data.iter() {
        for entity in entities {
            if entity.changed && !entity.vertices.is_empty() {
                // Вычисляем центр элемента по X,Y
                let mut center_x = 0.0;
                let mut center_y = 0.0;
                
                for vertex in &entity.vertices {
                    center_x += vertex.x;
                    center_y += vertex.y;
                }
                
                center_x /= entity.vertices.len() as f64;
                center_y /= entity.vertices.len() as f64;
                
                // Форматируем координаты для имени блока
                // Заменяем точку на подчеркивание для имени блока
                let x_str = format!("{:.1}", center_x).replace(".", "_");
                let y_str = format!("{:.1}", center_y).replace(".", "_");
                let key = format!("x{}_y{}", x_str, y_str);
                
                // Добавляем элемент в соответствующую группу
                xy_groups.entry(key).or_insert_with(Vec::new).push(entity);
            }
        }
    }
    
    // Создаем блок для каждой группы элементов с одинаковыми X,Y
    for (key, entities) in &xy_groups {
        // Начало определения блока
        dxf_content.push_str(&format!("0\nBLOCK\n2\n{}\n", key));
        dxf_content.push_str("70\n0\n10\n0.0\n20\n0.0\n30\n0.0\n");
        
        // Добавляем элементы в блок
        for entity in entities {
            // Определяем слой для элемента
            let layer_name = if let Some(material) = &entity.material {
                if let Some(sg_type) = &material.sg_type {
                    let sg_type = sg_type.to_ascii_lowercase();
                    let b_or_d_val = material.b_or_d.unwrap_or(0.0);
                    let h_or_d_val = material.h_or_d.unwrap_or(0.0);
                    
                    let b_or_d_str = format!("{}", b_or_d_val).replace(".", "_");
                    let h_or_d_str = format!("{}", h_or_d_val).replace(".", "_");
                    
                    format!("{}-{}-{}", sg_type, b_or_d_str, h_or_d_str)
                } else if let Some(material_num) = material.material_num {
                    format!("material-{}", material_num)
                } else {
                    "0".to_string()
                }
            } else {
                "0".to_string()
            };
            
            match entity.vertices.len() {
                4 => {
                    // 3DFACE для четырехугольника
                    dxf_content.push_str("0\n3DFACE\n");
                    dxf_content.push_str(&format!("8\n{}\n", layer_name)); // Слой
                    dxf_content.push_str("62\n1\n"); // Красный цвет
                    
                    // Координаты вершин
                    dxf_content.push_str(&format!("10\n{}\n20\n{}\n30\n{}\n", 
                        entity.vertices[0].x, entity.vertices[0].y, entity.vertices[0].z));
                    dxf_content.push_str(&format!("11\n{}\n21\n{}\n31\n{}\n", 
                        entity.vertices[1].x, entity.vertices[1].y, entity.vertices[1].z));
                    dxf_content.push_str(&format!("12\n{}\n22\n{}\n32\n{}\n", 
                        entity.vertices[2].x, entity.vertices[2].y, entity.vertices[2].z));
                    dxf_content.push_str(&format!("13\n{}\n23\n{}\n33\n{}\n", 
                        entity.vertices[3].x, entity.vertices[3].y, entity.vertices[3].z));
                    
                    // Добавляем текст с значением as
                    if let Some(row) = &entity.row {
                        let center_x = (entity.vertices[0].x + entity.vertices[2].x) / 2.0;
                        let center_y = (entity.vertices[0].y + entity.vertices[2].y) / 2.0;
                        let z = entity.vertices[0].z;
                        
                        // Выбираем параметр в зависимости от индекса
                        let as_value = match as_index {
                            0 => row.as1[0],
                            1 => row.as2[0],
                            2 => row.as3[0],
                            3 => row.as4[0],
                            _ => 0.0,
                        };
                        
                        let as_name = format!("as{}", as_index + 1);
                        let text = format!("{}:{:.1}", as_name, as_value);
                        
                        // Добавляем текст
                        dxf_content.push_str("0\nTEXT\n");
                        dxf_content.push_str(&format!("8\n{}\n", layer_name)); // Тот же слой
                        dxf_content.push_str("62\n2\n"); // Зеленый цвет для текста
                        dxf_content.push_str(&format!("10\n{}\n20\n{}\n30\n{}\n", center_x, center_y, z));
                        dxf_content.push_str("40\n0.05\n"); // Высота текста
                        dxf_content.push_str(&format!("1\n{}\n", text));
                        dxf_content.push_str("50\n0.0\n"); // Угол поворота
                    }
                },
                3 => {
                    // 3DFACE для треугольника
                    dxf_content.push_str("0\n3DFACE\n");
                    dxf_content.push_str(&format!("8\n{}\n", layer_name)); // Слой
                    dxf_content.push_str("62\n1\n"); // Красный цвет
                    
                    // Координаты вершин
                    dxf_content.push_str(&format!("10\n{}\n20\n{}\n30\n{}\n", 
                        entity.vertices[0].x, entity.vertices[0].y, entity.vertices[0].z));
                    dxf_content.push_str(&format!("11\n{}\n21\n{}\n31\n{}\n", 
                        entity.vertices[1].x, entity.vertices[1].y, entity.vertices[1].z));
                    dxf_content.push_str(&format!("12\n{}\n22\n{}\n32\n{}\n", 
                        entity.vertices[2].x, entity.vertices[2].y, entity.vertices[2].z));
                    dxf_content.push_str(&format!("13\n{}\n23\n{}\n33\n{}\n", 
                        entity.vertices[0].x, entity.vertices[0].y, entity.vertices[0].z));
                    
                    // Добавляем текст с значением as
                    if let Some(row) = &entity.row {
                        let center_x = (entity.vertices[0].x + entity.vertices[1].x + entity.vertices[2].x) / 3.0;
                        let center_y = (entity.vertices[0].y + entity.vertices[1].y + entity.vertices[2].y) / 3.0;
                        let z = entity.vertices[0].z;
                        
                        // Выбираем параметр в зависимости от индекса
                        let as_value = match as_index {
                            0 => row.as1[0],
                            1 => row.as2[0],
                            2 => row.as3[0],
                            3 => row.as4[0],
                            _ => 0.0,
                        };
                        
                        let as_name = format!("as{}", as_index + 1);
                        let text = format!("{}:{:.1}", as_name, as_value);
                        
                        // Добавляем текст
                        dxf_content.push_str("0\nTEXT\n");
                        dxf_content.push_str(&format!("8\n{}\n", layer_name)); // Тот же слой
                        dxf_content.push_str("62\n2\n"); // Зеленый цвет для текста
                        dxf_content.push_str(&format!("10\n{}\n20\n{}\n30\n{}\n", center_x, center_y, z));
                        dxf_content.push_str("40\n0.05\n"); // Высота текста
                        dxf_content.push_str(&format!("1\n{}\n", text));
                        dxf_content.push_str("50\n0.0\n"); // Угол поворота
                    }
                },
                2 => {
                    // LINE для линии
                    dxf_content.push_str("0\nLINE\n");
                    dxf_content.push_str(&format!("8\n{}\n", layer_name)); // Слой
                    dxf_content.push_str("62\n1\n"); // Красный цвет
                    
                    // Координаты начала и конца
                    dxf_content.push_str(&format!("10\n{}\n20\n{}\n30\n{}\n", 
                        entity.vertices[0].x, entity.vertices[0].y, entity.vertices[0].z));
                    dxf_content.push_str(&format!("11\n{}\n21\n{}\n31\n{}\n", 
                        entity.vertices[1].x, entity.vertices[1].y, entity.vertices[1].z));
                },
                _ => {}
            }
        }
        
        // Конец определения блока
        dxf_content.push_str("0\nENDBLK\n");
    }
    
    // Конец секции блоков
    dxf_content.push_str("0\nENDSEC\n");
    
    // Секция сущностей
    dxf_content.push_str("0\nSECTION\n2\nENTITIES\n");
    
    // Вставляем блоки
    for key in xy_groups.keys() {
        dxf_content.push_str("0\nINSERT\n");
        dxf_content.push_str(&format!("2\n{}\n", key));
        dxf_content.push_str("8\n0\n"); // Слой для вставки блока
        dxf_content.push_str("10\n0.0\n20\n0.0\n30\n0.0\n"); // Точка вставки
        dxf_content.push_str("41\n1.0\n42\n1.0\n43\n1.0\n"); // Масштаб
        dxf_content.push_str("50\n0.0\n"); // Угол поворота
    }
    
    // Добавляем неизмененные элементы напрямую
    for (_z, entities) in data.iter() {
        for entity in entities {
            if !entity.changed {
                // Определяем слой для элемента
                let layer_name = if let Some(material) = &entity.material {
                    if let Some(sg_type) = &material.sg_type {
                        let sg_type = sg_type.to_ascii_lowercase();
                        let b_or_d_val = material.b_or_d.unwrap_or(0.0);
                        let h_or_d_val = material.h_or_d.unwrap_or(0.0);
                        let b_or_d_str = format!("{}", b_or_d_val).replace(".", "_");
                        let h_or_d_str = format!("{}", h_or_d_val).replace(".", "_");
                        format!("{}-{}-{}", sg_type, b_or_d_str, h_or_d_str)
                    } else if let Some(material_num) = material.material_num {
                        format!("material-{}", material_num)
                    } else {
                        "0".to_string()
                    }
                } else {
                    "0".to_string()
                };
                
                match entity.vertices.len() {
                    4 => {
                        // 3DFACE для четырехугольника
                        dxf_content.push_str("0\n3DFACE\n");
                        dxf_content.push_str(&format!("8\n{}\n", layer_name)); // Слой
                        
                        // Координаты вершин
                        dxf_content.push_str(&format!("10\n{}\n20\n{}\n30\n{}\n", 
                            entity.vertices[0].x, entity.vertices[0].y, entity.vertices[0].z));
                        dxf_content.push_str(&format!("11\n{}\n21\n{}\n31\n{}\n", 
                            entity.vertices[1].x, entity.vertices[1].y, entity.vertices[1].z));
                        dxf_content.push_str(&format!("12\n{}\n22\n{}\n32\n{}\n", 
                            entity.vertices[2].x, entity.vertices[2].y, entity.vertices[2].z));
                        dxf_content.push_str(&format!("13\n{}\n23\n{}\n33\n{}\n", 
                            entity.vertices[3].x, entity.vertices[3].y, entity.vertices[3].z));
                        
                        // Добавляем текст с значением as
                        if let Some(row) = &entity.row {
                            let center_x = (entity.vertices[0].x + entity.vertices[2].x) / 2.0;
                            let center_y = (entity.vertices[0].y + entity.vertices[2].y) / 2.0;
                            let z = entity.vertices[0].z;
                            
                            // Выбираем параметр в зависимости от индекса
                            let as_value = match as_index {
                                0 => row.as1[0],
                                1 => row.as2[0],
                                2 => row.as3[0],
                                3 => row.as4[0],
                                _ => 0.0,
                            };
                            
                            let as_name = format!("as{}", as_index + 1);
                            let text = format!("{}:{:.1}", as_name, as_value);
                            
                            // Добавляем текст
                            dxf_content.push_str("0\nTEXT\n");
                            dxf_content.push_str(&format!("8\n{}\n", layer_name)); // Тот же слой
                            dxf_content.push_str(&format!("10\n{}\n20\n{}\n30\n{}\n", center_x, center_y, z));
                            dxf_content.push_str("40\n0.05\n"); // Высота текста
                            dxf_content.push_str(&format!("1\n{}\n", text));
                            dxf_content.push_str("50\n0.0\n"); // Угол поворота
                        }
                    },
                    3 => {
                        // 3DFACE для треугольника
                        dxf_content.push_str("0\n3DFACE\n");
                        dxf_content.push_str(&format!("8\n{}\n", layer_name)); // Слой
                        
                        // Координаты вершин
                        dxf_content.push_str(&format!("10\n{}\n20\n{}\n30\n{}\n", 
                            entity.vertices[0].x, entity.vertices[0].y, entity.vertices[0].z));
                        dxf_content.push_str(&format!("11\n{}\n21\n{}\n31\n{}\n", 
                            entity.vertices[1].x, entity.vertices[1].y, entity.vertices[1].z));
                        dxf_content.push_str(&format!("12\n{}\n22\n{}\n32\n{}\n", 
                            entity.vertices[2].x, entity.vertices[2].y, entity.vertices[2].z));
                        dxf_content.push_str(&format!("13\n{}\n23\n{}\n33\n{}\n", 
                            entity.vertices[0].x, entity.vertices[0].y, entity.vertices[0].z));
                        
                        // Добавляем текст с значением as
                        if let Some(row) = &entity.row {
                            let center_x = (entity.vertices[0].x + entity.vertices[1].x + entity.vertices[2].x) / 3.0;
                            let center_y = (entity.vertices[0].y + entity.vertices[1].y + entity.vertices[2].y) / 3.0;
                            let z = entity.vertices[0].z;
                            
                            // Выбираем параметр в зависимости от индекса
                            let as_value = match as_index {
                                0 => row.as1[0],
                                1 => row.as2[0],
                                2 => row.as3[0],
                                3 => row.as4[0],
                                _ => 0.0,
                            };
                            
                            let as_name = format!("as{}", as_index + 1);
                            let text = format!("{}:{:.1}", as_name, as_value);
                            
                            // Добавляем текст
                            dxf_content.push_str("0\nTEXT\n");
                            dxf_content.push_str(&format!("8\n{}\n", layer_name)); // Тот же слой
                            dxf_content.push_str(&format!("10\n{}\n20\n{}\n30\n{}\n", center_x, center_y, z));
                            dxf_content.push_str("40\n0.05\n"); // Высота текста
                            dxf_content.push_str(&format!("1\n{}\n", text));
                            dxf_content.push_str("50\n0.0\n"); // Угол поворота
                        }
                    },
                    2 => {
                        // LINE для линии
                        dxf_content.push_str("0\nLINE\n");
                        dxf_content.push_str(&format!("8\n{}\n", layer_name)); // Слой
                        
                        // Координаты начала и конца
                        dxf_content.push_str(&format!("10\n{}\n20\n{}\n30\n{}\n", 
                            entity.vertices[0].x, entity.vertices[0].y, entity.vertices[0].z));
                        dxf_content.push_str(&format!("11\n{}\n21\n{}\n31\n{}\n", 
                            entity.vertices[1].x, entity.vertices[1].y, entity.vertices[1].z));
                    },
                    _ => {}
                }
            }
        }
    }
    
    // Конец секции сущностей
    dxf_content.push_str("0\nENDSEC\n");
    
    // Конец файла
    dxf_content.push_str("0\nEOF\n");
    
    // Возвращаем байты
    dxf_content.into_bytes()
}