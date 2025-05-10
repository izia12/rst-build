use std::collections::HashMap;
use std::fs::File;

use crate::string_log_two_params;

use super::drawItem::Draw_Item_Z;
use super::parse::EntityWithXlsx;
use std::io::{self, Cursor, Read};

use dxf::enums::{HorizontalTextJustification, VerticalTextJustification};
use dxf::{entities::*, Color, Drawing, Vector};
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
pub fn create_dxf_entity_xlsx(data:HashMap<OrderedFloat<f32>, Draw_Item_Z>)->Vec<u8>{
	let mut drawing = Drawing::new();
	for (key, item_z) in data{
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
// ... existing code ...

pub fn create_dxf_after_change(data: HashMap<OrderedFloat<f32>, Vec<EntityWithXlsx>>) -> Vec<u8> {
    let mut drawing = Drawing::new();
    
    for (_z, entities) in data.iter() {
        for entity in entities {
            // Создаем основную геометрию
            match entity.vertices.len() {
                4 => {
                    let face3d = Face3D::new(
                        Point::new(entity.vertices[0].x, entity.vertices[0].y, entity.vertices[0].z),
                        Point::new(entity.vertices[1].x, entity.vertices[1].y, entity.vertices[1].z),
                        Point::new(entity.vertices[2].x, entity.vertices[2].y, entity.vertices[2].z),
                        Point::new(entity.vertices[3].x, entity.vertices[3].y, entity.vertices[3].z),
                    );
					if entity.changed==true {
						string_log_two_params(&format!("{}", serde_json::json!(entity)), &String::from("This changed"));
						let mut face_entity = Entity::new(EntityType::Face3D(face3d.clone()));
						face_entity.common.color = Color::from_index(1); // 1 - красный цвет в DXF
						drawing.add_entity(face_entity);
					}else {
						drawing.add_entity(Entity::new(EntityType::Face3D(face3d)));
					}
					// 1 - красный цвет в DXF
                    // Добавляем текст с значениями as1-as4
                    if let Some(row) = &entity.row {
                        // Вычисляем центр фигуры для размещения текста
                        let center_x = (entity.vertices[0].x + entity.vertices[2].x) / 2.0;
                        let center_y = (entity.vertices[0].y + entity.vertices[2].y) / 2.0;
                        let z = entity.vertices[0].z;

                        // Создаем отдельные текстовые надписи для каждого значения
                        let as1_text = Text{
                            thickness: 0.0,
                            location: Point::new(center_x, center_y + 0.15, z),
                            text_height: 0.05,
                            value: format!("as1:{:.1}", row.as1[0]),
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
                        drawing.add_entity(Entity::new(EntityType::Text(as1_text)));
                        
                        let as2_text = Text{
                            thickness: 0.0,
                            location: Point::new(center_x, center_y + 0.05, z),
                            text_height: 0.05,
                            value: format!("as2:{:.1}", row.as2[0]),
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
                        drawing.add_entity(Entity::new(EntityType::Text(as2_text)));
                        
                        let as3_text = Text{
                            thickness: 0.0,
                            location: Point::new(center_x, center_y - 0.05, z),
                            text_height: 0.05,
                            value: format!("as3:{:.1}", row.as3[0]),
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
                        drawing.add_entity(Entity::new(EntityType::Text(as3_text)));
                        
                        let as4_text = Text{
                            thickness: 0.0,
                            location: Point::new(center_x, center_y - 0.15, z),
                            text_height: 0.05,
                            value: format!("as4:{:.1}", row.as4[0]),
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
                        drawing.add_entity(Entity::new(EntityType::Text(as4_text)));
                    }
                },
                3 => {
                    let face3d = Face3D::new(
                        Point::new(entity.vertices[0].x, entity.vertices[0].y, entity.vertices[0].z),
                        Point::new(entity.vertices[1].x, entity.vertices[1].y, entity.vertices[1].z),
                        Point::new(entity.vertices[2].x, entity.vertices[2].y, entity.vertices[2].z),
                        Point::new(entity.vertices[0].x, entity.vertices[0].y, entity.vertices[0].z),
                    );
					if entity.changed==true {
						string_log_two_params(&format!("{}", serde_json::json!(entity)), &String::from("This changed"));
						let mut face_entity = Entity::new(EntityType::Face3D(face3d.clone()));
						face_entity.common.color = Color::from_index(1); // 1 - красный цвет в DXF
						drawing.add_entity(face_entity);
					}
					else {
						drawing.add_entity(Entity::new(EntityType::Face3D(face3d)));
					}
					// 1 - красный цвет в DXF

                    // Добавляем текст с значениями as1-as4 для треугольника
                    if let Some(row) = &entity.row {
                        let center_x = (entity.vertices[0].x + entity.vertices[1].x + entity.vertices[2].x) / 3.0;
                        let center_y = (entity.vertices[0].y + entity.vertices[1].y + entity.vertices[2].y) / 3.0;
                        let z = entity.vertices[0].z;

                        // Создаем отдельные текстовые надписи для каждого значения
                        let as1_text = Text{
                            thickness: 0.0,
                            location: Point::new(center_x, center_y + 0.15, z),
                            text_height: 0.05,
                            value: format!("as1:{:.1}", row.as1[0]),
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
                        drawing.add_entity(Entity::new(EntityType::Text(as1_text)));
                        
                        let as2_text = Text{
                            thickness: 0.0,
                            location: Point::new(center_x, center_y + 0.05, z),
                            text_height: 0.05,
                            value: format!("as2:{:.1}", row.as2[0]),
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
                        drawing.add_entity(Entity::new(EntityType::Text(as2_text)));
                        
                        let as3_text = Text{
							thickness: 0.0,
                            location: Point::new(center_x, center_y - 0.05, z),
                            text_height: 0.05,
                            value: format!("as3:{:.1}", row.as3[0]),
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
                        drawing.add_entity(Entity::new(EntityType::Text(as3_text)));
                        
                        let as4_text = Text{
                            thickness: 0.0,
                            location: Point::new(center_x, center_y - 0.15, z),
                            text_height: 0.05,
                            value: format!("as4:{:.1}", row.as4[0]),
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
                        drawing.add_entity(Entity::new(EntityType::Text(as4_text)));
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

    let mut buffer = Cursor::new(Vec::new());
    drawing.save(&mut buffer).expect("Ошибка записи DXF");
    
    buffer.into_inner()
}