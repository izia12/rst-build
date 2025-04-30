use std::collections::HashMap;
use std::fs::File;

use super::drawItem::Draw_Item_Z;
use super::parse::EntityWithXlsx;
use std::io::{self, Cursor, Read};

use dxf::{entities::*, Drawing};
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