use std::collections::HashMap;

use crate::string_log_two_params;

use super::drawItem::DrawItemZ;
use super::parse::EntityWithXlsx;
use std::io:: Cursor;

use dxf::enums::{HorizontalTextJustification, VerticalTextJustification};
use dxf::{entities::*, Block, Color, Drawing, Vector};
use dxf::Point;
use ordered_float::OrderedFloat;
use wasm_bindgen::prelude::wasm_bindgen;



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

pub fn create_dxf_after_change(data: HashMap<OrderedFloat<f32>, Vec<EntityWithXlsx>>) -> Vec<u8> {
    let mut drawing = Drawing::new();
    
    // Создаем один общий блок для всех измененных элементов
    let block_name = "CHANGED_ELEMENTS";
    let mut changed_elements_block = Block {
        name: String::from(block_name),
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
                        
                        // Добавляем текст с значениями as1-as4 в блок
                        if let Some(row) = &entity.row {
                            let center_x = (entity.vertices[0].x + entity.vertices[2].x) / 2.0;
                            let center_y = (entity.vertices[0].y + entity.vertices[2].y) / 2.0;
                            let z = entity.vertices[0].z;
                            
                            changed_elements_block.entities.push(create_text_entity(
                                center_x, center_y, z, 0.15, 
                                format!("as1:{:.1}", row.as1[0]), 2
                            ));
                            changed_elements_block.entities.push(create_text_entity(
                                center_x, center_y, z, 0.05, 
                                format!("as2:{:.1}", row.as2[0]), 3
                            ));
                            changed_elements_block.entities.push(create_text_entity(
                                center_x, center_y, z, -0.05, 
                                format!("as3:{:.1}", row.as3[0]), 4
                            ));
                            changed_elements_block.entities.push(create_text_entity(
                                center_x, center_y, z, -0.15, 
                                format!("as4:{:.1}", row.as4[0]), 5
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
                        
                        // Добавляем текст с значениями as1-as4 в блок
                        if let Some(row) = &entity.row {
                            let center_x = (entity.vertices[0].x + entity.vertices[1].x + entity.vertices[2].x) / 3.0;
                            let center_y = (entity.vertices[0].y + entity.vertices[1].y + entity.vertices[2].y) / 3.0;
                            let z = entity.vertices[0].z;
                            
                            changed_elements_block.entities.push(create_text_entity(
                                center_x, center_y, z, 0.15, 
                                format!("as1:{:.1}", row.as1[0]), 2
                            ));
                            changed_elements_block.entities.push(create_text_entity(
                                center_x, center_y, z, 0.05, 
                                format!("as2:{:.1}", row.as2[0]), 3
                            ));
                            changed_elements_block.entities.push(create_text_entity(
                                center_x, center_y, z, -0.05, 
                                format!("as3:{:.1}", row.as3[0]), 4
                            ));
                            changed_elements_block.entities.push(create_text_entity(
                                center_x, center_y, z, -0.15, 
                                format!("as4:{:.1}", row.as4[0]), 5
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
                        
                        // Добавляем текст с значениями as1-as4
                        if let Some(row) = &entity.row {
                            let center_x = (entity.vertices[0].x + entity.vertices[2].x) / 2.0;
                            let center_y = (entity.vertices[0].y + entity.vertices[2].y) / 2.0;
                            let z = entity.vertices[0].z;
                            
                            drawing.add_entity(create_text_entity(
                                center_x, center_y, z, 0.15, 
                                format!("as1:{:.1}", row.as1[0]), 0
                            ));
                            drawing.add_entity(create_text_entity(
                                center_x, center_y, z, 0.05, 
                                format!("as2:{:.1}", row.as2[0]), 0
                            ));
                            drawing.add_entity(create_text_entity(
                                center_x, center_y, z, -0.05, 
                                format!("as3:{:.1}", row.as3[0]), 0
                            ));
                            drawing.add_entity(create_text_entity(
                                center_x, center_y, z, -0.15, 
                                format!("as4:{:.1}", row.as4[0]), 0
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
                        
                        // Добавляем текст с значениями as1-as4
                        if let Some(row) = &entity.row {
                            let center_x = (entity.vertices[0].x + entity.vertices[2].x) / 2.0;
                            let center_y = (entity.vertices[0].y + entity.vertices[2].y) / 2.0;
                            let z = entity.vertices[0].z;
                            
                            drawing.add_entity(create_text_entity(
                                center_x, center_y, z, 0.15, 
                                format!("as1:{:.1}", row.as1[0]), 0
                            ));
                            drawing.add_entity(create_text_entity(
                                center_x, center_y, z, 0.05, 
                                format!("as2:{:.1}", row.as2[0]), 0
                            ));
                            drawing.add_entity(create_text_entity(
                                center_x, center_y, z, -0.05, 
                                format!("as3:{:.1}", row.as3[0]), 0
                            ));
                            drawing.add_entity(create_text_entity(
                                center_x, center_y, z, -0.15, 
                                format!("as4:{:.1}", row.as4[0]), 0
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