use std::collections::HashMap;
use std::fs::File;

use super::drawItem::Draw_Item_Z;
use super::parse::EntityWithXlsx;
use std::io::{self, Cursor, Read};

use dxf::enums::{HorizontalTextJustification, VerticalTextJustification};
use dxf::{entities::*, Drawing, Vector};
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
                    drawing.add_entity(Entity::new(EntityType::Face3D(face3d)));

                    // Добавляем текст с значениями as1-as4
                    if let Some(row) = &entity.row {
                        // Вычисляем центр фигуры для размещения текста
                        let center_x = (entity.vertices[0].x + entity.vertices[2].x) / 2.0;
                        let center_y = (entity.vertices[0].y + entity.vertices[2].y) / 2.0;
                        let z = entity.vertices[0].z;

                        // Создаем текстовые надписи
                        let text_content = format!(
                            "as1:{:.1}\nas2:{:.1}\nas3:{:.1}\nas4:{:.1}",
                            row.as1[0], row.as2[0], row.as3[0], row.as4[0]
                        );
                        // let text = Text::new(
                        //     Point::new(center_x, center_y, z),
                        //     1.0, // высота текста
                        //     text_content
                        // );
						// let text = Text::default(point:Point::new(center_x, center_y, z));
						let normal = Vector::new(
							(entity.vertices[1].y - entity.vertices[0].y) * (entity.vertices[2].z - entity.vertices[0].z) - 
							(entity.vertices[1].z - entity.vertices[0].z) * (entity.vertices[2].y - entity.vertices[0].y),
							(entity.vertices[1].z - entity.vertices[0].z) * (entity.vertices[2].x - entity.vertices[0].x) - 
							(entity.vertices[1].x - entity.vertices[0].x) * (entity.vertices[2].z - entity.vertices[0].z),
							(entity.vertices[1].x - entity.vertices[0].x) * (entity.vertices[2].y - entity.vertices[0].y) - 
							(entity.vertices[1].y - entity.vertices[0].y) * (entity.vertices[2].x - entity.vertices[0].x)
						);
						let length = (normal.x.powi(2) + normal.y.powi(2) + normal.z.powi(2)).sqrt();
							let normal = if length > 0.0 {
							    Vector::new(normal.x / length, normal.y / length, normal.z / length)
							} else {
							    Vector::z_axis()
							};
						let text = Text{
							thickness:0.5,
							location: Point::new(center_x, center_y , z), // ,
							text_height:0.05,
							value: String::from(text_content),
							rotation: 0.0,
							relative_x_scale_factor: 1.0,
							oblique_angle: 0.0,
							text_style_name: String::from("STANDARD"),
							text_generation_flags: 0,
							horizontal_text_justification: HorizontalTextJustification::Center,
							second_alignment_point: Point::origin(),
							normal,
							vertical_text_justification: VerticalTextJustification::Middle,
						};
                        drawing.add_entity(Entity::new(EntityType::Text(text)));
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

                    // Добавляем текст с значениями as1-as4 для треугольника
                    if let Some(row) = &entity.row {
                        let center_x = (entity.vertices[0].x + entity.vertices[1].x + entity.vertices[2].x) / 3.0;
                        let center_y = (entity.vertices[0].y + entity.vertices[1].y + entity.vertices[2].y) / 3.0;
                        let z = entity.vertices[0].z;

                        let text_content = format!(
                            "as1:{:.1}\nas2:{:.1}\nas3:{:.1}\nas4:{:.1}",
                            row.as1[0], row.as2[0], row.as3[0], row.as4[0]
                        );
						let normal = Vector::new(
							(entity.vertices[1].y - entity.vertices[0].y) * (entity.vertices[2].z - entity.vertices[0].z) - 
							(entity.vertices[1].z - entity.vertices[0].z) * (entity.vertices[2].y - entity.vertices[0].y),
							(entity.vertices[1].z - entity.vertices[0].z) * (entity.vertices[2].x - entity.vertices[0].x) - 
							(entity.vertices[1].x - entity.vertices[0].x) * (entity.vertices[2].z - entity.vertices[0].z),
							(entity.vertices[1].x - entity.vertices[0].x) * (entity.vertices[2].y - entity.vertices[0].y) - 
							(entity.vertices[1].y - entity.vertices[0].y) * (entity.vertices[2].x - entity.vertices[0].x)
						);
						let length = (normal.x.powi(2) + normal.y.powi(2) + normal.z.powi(2)).sqrt();
							let normal = if length > 0.0 {
							    Vector::new(normal.x / length, normal.y / length, normal.z / length)
							} else {
							    Vector::z_axis()
							};
						let text = Text{
							thickness:0.5,
							location: Point::new(center_x , center_y , z), // ,
							text_height:0.05,
							value: String::from(text_content),
							rotation: 0.0,
							relative_x_scale_factor: 1.0,
							oblique_angle: 0.0,
							text_style_name: String::from("STANDARD"),
							text_generation_flags: 0,
							horizontal_text_justification: HorizontalTextJustification::Left,
							second_alignment_point: Point::origin(),
							normal,
							vertical_text_justification: VerticalTextJustification::Baseline,
						};
                        drawing.add_entity(Entity::new(EntityType::Text(text)));
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