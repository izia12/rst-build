use std::{char::MAX, io::Cursor};

use image::{ImageBuffer, ImageOutputFormat, Rgb};
use imageproc::{drawing::{draw_line_segment_mut, draw_text_mut}, point::Point};
use rusttype::{Font, Scale};
use serde::{Deserialize, Serialize};

use crate::string_log_two_params;

use super::parse::EntityWithXlsx;

pub enum AsFunctions  {
	As1,
	As2,
	As3,
	As4
} 
#[derive(Debug, Serialize)]
pub struct Draw_Item_Z{
	pub data:Vec<EntityWithXlsx>,

}
impl Draw_Item_Z {

	pub fn draw_image_AS1( &self, field:&str)->Vec<u8> {
		let full_width = 2500;
    	let full_height = 2000;
		let mut img = ImageBuffer::from_fn(full_width, full_height, |_, _| Rgb([255u8, 255u8, 255u8]));
		let font_size = 12.0;
		let text_color = Rgb([0u8, 0u8, 0u8]);

		let font_data = include_bytes!("../OpenSans-Regular.ttf");
	    let font = Font::try_from_bytes(font_data).unwrap();
	    let scale = Scale::uniform(font_size);
		// self
		for item in &self.data{
			// if item.vertices.len()<200{
			// 	string_log_two_params("Это те самые малеькие", &serde_json::to_string_pretty(&item).unwrap());
			// }
			if item.vertices.len()==4{
				
				let point_a = Point::new((item.vertices[0].x * 40.0)+120.0, (item.vertices[0].y * 40.0)+80.0);
				let point_b = Point::new((item.vertices[1].x * 40.0)+120.0, (item.vertices[1].y * 40.0)+80.0);
				let point_c = Point::new((item.vertices[2].x * 40.0)+120.0, (item.vertices[2].y * 40.0)+80.0);
				let point_d = Point::new((item.vertices[3].x * 40.0)+120.0, (item.vertices[3].y * 40.0)+80.0);

				draw_line_segment_mut(&mut img, (point_a.x as f32, point_a.y as f32), (point_b.x as f32, point_b.y as f32), Rgb([255, 0, 0]));
				draw_line_segment_mut(&mut img, (point_b.x as f32, point_b.y as f32), (point_c.x as f32, point_c.y as f32), Rgb([255, 0, 0]));
				draw_line_segment_mut(&mut img, (point_c.x as f32, point_c.y as f32), (point_d.x as f32, point_d.y as f32), Rgb([255, 0, 0]));
				draw_line_segment_mut(&mut img, (point_d.x as f32, point_d.y as f32), (point_a.x as f32, point_a.y as f32), Rgb([255, 0, 0]));
				draw_text_mut(
					&mut img,
					text_color,
					((point_b.x+point_a.x)/2.0) as i32, // Центрирование текста
					((point_b.y+point_a.y)/2.0) as i32,          // Позиция внизу
					scale,
					&font,
					&item.get_value(field).unwrap().iter().cloned().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap().to_string(),
				);
			}
			else if item.vertices.len()==3 {
				let point_a = Point::new((item.vertices[0].x * 40.0)+120.0, (item.vertices[0].y * 40.0)+80.0);
				let point_b = Point::new((item.vertices[1].x * 40.0)+120.0, (item.vertices[1].y * 40.0)+80.0);
				let point_c = Point::new((item.vertices[2].x * 40.0)+120.0, (item.vertices[2].y * 40.0)+80.0);
				// let point_d = Point::new((item.vertices[3].x * 17.0)+150.0, (item.vertices[3].y * 17.0)+80.0);
				draw_line_segment_mut(&mut img, (point_a.x as f32, point_a.y as f32), (point_b.x as f32, point_b.y as f32), Rgb([255, 0, 0]));
				draw_line_segment_mut(&mut img, (point_b.x as f32, point_b.y as f32), (point_c.x as f32, point_c.y as f32), Rgb([255, 0, 0]));
				draw_line_segment_mut(&mut img, (point_c.x as f32, point_c.y as f32), (point_a.x as f32, point_a.y as f32), Rgb([255, 0, 0]));
				// draw_line_segment_mut(&mut img, (point_d.x as f32, point_d.y as f32), (point_a.x as f32, point_a.y as f32), Rgb([255, 0, 0]));
				draw_text_mut(
					&mut img,
					text_color,
					((point_b.x+point_a.x)/2.0) as i32, // Центрирование текста
					(((point_b.y+point_a.y)/2.0)-5.0) as i32,          // Позиция внизу
					scale,
					&font,
					// &item.row.as1.iter().cloned().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap().to_string(),
					&item.get_value(field).unwrap().iter().cloned().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap().to_string(),
				);
			}
			// else if item.vertices.len()==2 {
			// 	let point_a = Point::new((item.vertices[0].x * 17.0)+150.0, (item.vertices[0].y * 17.0)+80.0);
			// 	let point_b = Point::new((item.vertices[1].x * 17.0)+150.0, (item.vertices[1].y * 17.0)+80.0);
			// 	draw_line_segment_mut(&mut img, (point_a.x as f32, point_a.y as f32), (point_b.x as f32, point_b.y as f32), Rgb([255, 0, 0]));

			// }
		}
		let mut buffer = Vec::new();
		img.write_to(&mut Cursor::new(&mut buffer), ImageOutputFormat::Png).unwrap();
		buffer
	}
	pub fn draw_all_images( &self)->Vec<Vec<u8>> {
		let img1 = self.draw_image_AS1("as1");
		let img2 = self.draw_image_AS1("as2");
		let img3 = self.draw_image_AS1("as3");
		let img4 = self.draw_image_AS1("as4");
		let mut all_images = vec![];
		all_images.push(img1);
		all_images.push(img2);
		all_images.push(img3);
		all_images.push(img4);
		all_images
	}
	// pub fn get_image_item(id:usize)->Vec<u8>{

	// }
	pub fn log_to_data(&self){
		string_log_two_params("Это после сортировки по z",&serde_json::to_string_pretty(&self.data).unwrap());
	}
}

