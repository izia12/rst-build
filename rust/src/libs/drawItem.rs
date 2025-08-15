use std::io::Cursor;
use std::sync::{Arc, Mutex};

use image::{ImageBuffer, ImageOutputFormat, ImageFormat, Rgb, Rgba, ImageEncoder, codecs::png::{PngEncoder, CompressionType}};
use imageproc::{drawing::{draw_line_segment_mut, draw_text_mut}, point::Point};
use rusttype::{Font, Scale};
use serde::Serialize;
use web_sys::console;

use crate::string_log_two_params;
use super::parse::EntityWithXlsx;
use super::generate_documents::performance::{PerformanceConfig, PerformanceMonitor};
use super::gpu_renderer::{init_gpu_renderer, get_gpu_renderer, is_gpu_available};

pub enum AsFunctions  {
	As1,
	As2,
	As3,
	As4
}

// Структура для хранения размеров контента и границ
#[derive(Debug, Clone, Copy)]
pub struct ContentDimensions {
	pub min_x: f64,
	pub min_y: f64,
	pub max_x: f64,
	pub max_y: f64,
	pub content_width: f64,
	pub content_height: f64,
	pub img_width: u32,
	pub img_height: u32,
}

impl ContentDimensions {
	pub fn new(min_x: f64, min_y: f64, max_x: f64, max_y: f64, img_width: u32, img_height: u32) -> Self {
		Self {
			min_x,
			min_y,
			max_x,
			max_y,
			content_width: max_x - min_x,
			content_height: max_y - min_y,
			img_width,
			img_height,
		}
	}
}

// Кэшированный шрифт для повторного использования
lazy_static::lazy_static! {
    static ref CACHED_FONT: Arc<Font<'static>> = {
        let font_data = include_bytes!("../OpenSans-Regular.ttf");
        Arc::new(Font::try_from_bytes(font_data).unwrap())
    };
}

#[derive(Debug, Serialize)]
pub struct DrawItemZ{
	pub data:Vec<EntityWithXlsx>,
}
impl DrawItemZ {

	pub fn new() -> Self {
		DrawItemZ {
			data: Vec::new(),
		}
	}

	// Функция диагностики координат
	pub fn diagnose_coordinates(&self) {
		let mut negative_count = 0;
		let mut min_x = f64::INFINITY;
		let mut max_x = f64::NEG_INFINITY;
		let mut min_y = f64::INFINITY;
		let mut max_y = f64::NEG_INFINITY;
		let mut negative_coords = Vec::new();

		for (item_idx, item) in self.data.iter().enumerate() {
			for (vertex_idx, vertex) in item.vertices.iter().enumerate() {
				min_x = min_x.min(vertex.x);
				max_x = max_x.max(vertex.x);
				min_y = min_y.min(vertex.y);
				max_y = max_y.max(vertex.y);

				if vertex.x < 0.0 || vertex.y < 0.0 {
					negative_count += 1;
					negative_coords.push((item_idx, vertex_idx, vertex.x, vertex.y));
				}
			}
		}

		console::log_1(&format!(
			"=== ДИАГНОСТИКА КООРДИНАТ ===\n\
			Всего объектов: {}\n\
			Отрицательных координат: {}\n\
			Границы: X({:.2} до {:.2}), Y({:.2} до {:.2})\n\
			Размеры: {}x{}",
			self.data.len(),
			negative_count,
			min_x, max_x, min_y, max_y,
			max_x - min_x, max_y - min_y
		).into());

		if !negative_coords.is_empty() {
			console::log_1(&"=== ОТРИЦАТЕЛЬНЫЕ КООРДИНАТЫ ===".into());
			for (item_idx, vertex_idx, x, y) in negative_coords.iter().take(10) {
				console::log_1(&format!(
					"Объект {}, точка {}: ({:.2}, {:.2})",
					item_idx, vertex_idx, x, y
				).into());
			}
			if negative_coords.len() > 10 {
				console::log_1(&format!("... и еще {} координат", negative_coords.len() - 10).into());
			}
		}
	}

	pub fn add_entity(&mut self, entity: EntityWithXlsx) {
		self.data.push(entity);
	}

	// Функция для автоматического расчета границ изображения
	fn calculate_image_bounds(&self) -> ContentDimensions {
		let mut min_x = f64::INFINITY;
		let mut max_x = f64::NEG_INFINITY;
		let mut min_y = f64::INFINITY;
		let mut max_y = f64::NEG_INFINITY;

		// Находим границы всех объектов
		for item in &self.data {
			for vertex in &item.vertices {
				min_x = min_x.min(vertex.x);
				max_x = max_x.max(vertex.x);
				min_y = min_y.min(vertex.y);
				max_y = max_y.max(vertex.y);
			}
		}

		// Добавляем отступы
		let padding = 10.0;
		
		// Вычисляем реальные размеры контента (без масштабирования)
		let content_width = max_x - min_x;
		let content_height = max_y - min_y;
		
		// Определяем желаемый размер изображения (разумные значения)
		let target_width = 1200.0;
		let target_height = 900.0;
		
		// Вычисляем масштаб на основе реальных размеров контента
		let scale_x = if content_width > 0.0 { (target_width - padding * 2.0) / content_width } else { 1.0 };
		let scale_y = if content_height > 0.0 { (target_height - padding * 2.0) / content_height } else { 1.0 };
		
		// Используем меньший масштаб, чтобы все поместилось
		let scale = scale_x.max(scale_y)*1.5;
		
		// Вычисляем финальные размеры изображения
		let width = (content_width * scale + padding * 2.0) as u32;
		let height = (content_height * scale + padding * 2.0) as u32;
		
		// Применяем минимальные ограничения
		let width = width.max(400);
		let height = height.max(300);
		
		ContentDimensions::new(min_x, min_y, max_x, max_y, width, height)
	}

	fn calculate_image_bounds_with_config(&self, config: &PerformanceConfig) -> ContentDimensions {
		let mut min_x = f64::INFINITY;
		let mut max_x = f64::NEG_INFINITY;
		let mut min_y = f64::INFINITY;
		let mut max_y = f64::NEG_INFINITY;

		// Находим границы всех объектов
		for item in &self.data {
			for vertex in &item.vertices {
				min_x = min_x.min(vertex.x);
				max_x = max_x.max(vertex.x);
				min_y = min_y.min(vertex.y);
				max_y = max_y.max(vertex.y);
			}
		}

		// Вычисляем реальные размеры контента
		let content_width = max_x - min_x;
		let content_height = max_y - min_y;
		
		// Если контент пустой, возвращаем размеры из конфигурации
		if content_width <= 0.0 || content_height <= 0.0 {
			return ContentDimensions::new(min_x, min_y, max_x, max_y, (config.max_image_size.0 as f64 * 3.0) as u32, (config.max_image_size.1 as f64 * 3.0) as u32);
		}
		
		// Добавляем отступы вокруг контента
		let padding_x = content_width * 0.01;
		let padding_y = content_height * 0.01;
		
		// Расширяем границы с учетом отступов
		let padded_min_x = min_x - padding_x;
		let padded_max_x = max_x + padding_x;
		let padded_min_y = min_y - padding_y;
		let padded_max_y = max_y + padding_y;
		
		// Фиксированные размеры изображения для качества
		let img_width = (config.max_image_size.0 as f64 * 2.5) as u32;
		let img_height = (config.max_image_size.1 as f64 * 2.5) as u32;
		
		ContentDimensions::new(padded_min_x, padded_min_y, padded_max_x, padded_max_y, img_width, img_height)
	}


	
	// CPU fallback функция
	fn render_item_cpu_fallback(&self, item: &EntityWithXlsx, img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, 
								coord_scale: f64, offset_x: f64, offset_y: f64, field: &str, 
								font_scale: Scale, text_color: Rgb<u8>) {
		if item.vertices.len() == 4 {
			let points: Vec<Point<f64>> = item.vertices.iter().map(|v| {
				Point::new((v.x * coord_scale) + offset_x, (v.y * coord_scale) + offset_y)
			}).collect();
			
			for i in 0..4 {
				let next = (i + 1) % 4;
				draw_line_segment_mut(img, 
					(points[i].x as f32, points[i].y as f32), 
					(points[next].x as f32, points[next].y as f32), 
					Rgb([255, 0, 0]));
			}
		} else if item.vertices.len() == 3 {
			let points: Vec<Point<f64>> = item.vertices.iter().map(|v| {
				Point::new((v.x * coord_scale) + offset_x, (v.y * coord_scale) + offset_y)
			}).collect();
			
			for i in 0..3 {
				let next = (i + 1) % 3;
				draw_line_segment_mut(img,
					(points[i].x as f32, points[i].y as f32),
					(points[next].x as f32, points[next].y as f32),
					Rgb([255, 0, 0]));
			}
		}
		
		self.render_text_cpu(item, img, coord_scale, offset_x, offset_y, field, font_scale, text_color);
	}
	
	// Функция рендеринга текста на CPU
	fn render_text_cpu(&self, item: &EntityWithXlsx, img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, 
					   coord_scale: f64, offset_x: f64, offset_y: f64, field: &str, 
					   font_scale: Scale, text_color: Rgb<u8>) {
		if let Some(values) = item.get_value(field) {
			if let Some(max_value) = values.iter().cloned().max_by(|a, b| a.partial_cmp(b).unwrap()) {
				let points: Vec<Point<f64>> = item.vertices.iter().map(|v| {
					Point::new((v.x * coord_scale) + offset_x, (v.y * coord_scale) + offset_y)
				}).collect();
				
				if points.len() >= 2 {
					let text_x = ((points[1].x + points[0].x) / 2.0) as i32;
					let text_y = if item.vertices.len() == 3 {
						(((points[1].y + points[0].y) / 2.0) - 2.0) as i32
					} else {
						((points[1].y + points[0].y) / 2.0) as i32
					};
					
					draw_text_mut(img, text_color, text_x, text_y, font_scale, &CACHED_FONT, &max_value.to_string());
				}
			}
		}
	}

	pub async fn draw_image_as1(&self, field: &str) -> Vec<u8> {
		self.draw_image_as1_optimized(field, &PerformanceConfig::default()).await
	}

	pub async fn draw_image_as1_optimized(&self, field: &str, config: &PerformanceConfig) -> Vec<u8> {
		// Диагностика координат
		self.diagnose_coordinates();
		
		let dimensions = self.calculate_image_bounds_with_config(config);
		
		// Проверяем использование GPU ускорения
		let monitor = PerformanceMonitor::new(config.clone());
		
		// Проверяем доступность GPU и инициализируем если нужно
		let use_gpu = if !is_gpu_available() {
			match init_gpu_renderer().await {
				Ok(()) => {
					web_sys::console::log_1(&"GPU renderer initialized successfully".into());
					monitor.should_use_gpu_acceleration(self.data.len()) && get_gpu_renderer().is_some()
				},
				Err(e) => {
					web_sys::console::log_1(&format!("Failed to initialize GPU renderer: {}", e).into());
					false
				}
			}
		} else {
			monitor.should_use_gpu_acceleration(self.data.len()) && get_gpu_renderer().is_some()
		};
		
		if use_gpu {
			web_sys::console::log_1(&"🚀 GPU acceleration enabled for image rendering".into());
		} else {
			web_sys::console::log_1(&"💻 CPU rendering mode".into());
		}
		
		let mut img = ImageBuffer::from_fn(dimensions.img_width, dimensions.img_height, |_, _| Rgb([255u8, 255u8, 255u8]));
		
		// Оптимальное масштабирование для максимального заполнения изображения
		// Оставляем небольшие отступы (5% от размера изображения) для качества отображения
		let margin_x = dimensions.img_width as f64 * 0.05;
		let margin_y = dimensions.img_height as f64 * 0.05;
		let available_width = dimensions.img_width as f64 - 2.0 * margin_x;
		let available_height = dimensions.img_height as f64 - 2.0 * margin_y;
		
		let coord_scale = (available_width / dimensions.content_width).min(available_height / dimensions.content_height)*1.5;
		
		// Центрируем контент в доступной области с учетом отступов
		let scaled_content_width = dimensions.content_width * coord_scale;
		let scaled_content_height = dimensions.content_height * coord_scale;
		
		let offset_x = margin_x + (available_width - scaled_content_width) / 2.0 - dimensions.min_x * coord_scale;
		let offset_y = margin_y + (available_height - scaled_content_height) / 2.0 - dimensions.min_y * coord_scale;
		let font_size = 25.0; 
		let text_color = Rgb([0u8, 0u8, 0u8]);
		// Используем кэшированный шрифт
	    let font_scale = Scale::uniform(font_size);
		// Рисуем все объекты с автоматическим позиционированием
		if use_gpu && self.data.len() > 100 {
			// GPU-ускоренный рендеринг - собираем все линии сразу
			let mut all_lines = Vec::new();
			
			for item in &self.data {
				if item.vertices.len() == 4 {
					let points: Vec<Point<f64>> = item.vertices.iter().map(|v| {
						Point::new((v.x * coord_scale) + offset_x, (v.y * coord_scale) + offset_y)
					}).collect();
					
					// Добавляем линии четырехугольника
					for i in 0..4 {
						let next = (i + 1) % 4;
						all_lines.push((
							points[i].x as f32, 
							points[i].y as f32, 
							points[next].x as f32, 
							points[next].y as f32
						));
					}
				} else if item.vertices.len() == 3 {
					let points: Vec<Point<f64>> = item.vertices.iter().map(|v| {
						Point::new((v.x * coord_scale) + offset_x, (v.y * coord_scale) + offset_y)
					}).collect();
					
					// Добавляем линии треугольника
					for i in 0..3 {
						let next = (i + 1) % 3;
						all_lines.push((
							points[i].x as f32,
							points[i].y as f32,
							points[next].x as f32,
							points[next].y as f32
						));
					}
				}
			}
			
			// Конвертируем RGB в RGBA для GPU
			let mut rgba_img = ImageBuffer::new(img.width(), img.height());
			for (x, y, rgb_pixel) in img.enumerate_pixels() {
				rgba_img.put_pixel(x, y, image::Rgba([rgb_pixel[0], rgb_pixel[1], rgb_pixel[2], 255]));
			}
			
			// Рендерим все линии одним вызовом GPU
			if let Some(gpu_renderer) = get_gpu_renderer() {
				if let Err(e) = gpu_renderer.render_lines_gpu(&mut rgba_img, &all_lines, [255, 0, 0, 255]).await {
					web_sys::console::log_1(&format!("GPU rendering error: {}, falling back to CPU", e).into());
					// Fallback на CPU рендеринг
					for item in &self.data {
						self.render_item_cpu_fallback(item, &mut img, coord_scale, offset_x, offset_y, field, font_scale, text_color);
					}
				} else {
					// Конвертируем обратно в RGB
					for (x, y, rgba_pixel) in rgba_img.enumerate_pixels() {
						img.put_pixel(x, y, image::Rgb([rgba_pixel[0], rgba_pixel[1], rgba_pixel[2]]));
					}
					
					// Рендерим текст на CPU после GPU рендеринга линий
					for item in &self.data {
						self.render_text_cpu(item, &mut img, coord_scale, offset_x, offset_y, field, font_scale, text_color);
					}
				}
			}
		} else {
			// Обычный CPU рендеринг
			for item in &self.data{
			if item.vertices.len()==4{
				let point_a = Point::new((item.vertices[0].x * coord_scale) + offset_x, (item.vertices[0].y * coord_scale) + offset_y);
				let point_b = Point::new((item.vertices[1].x * coord_scale) + offset_x, (item.vertices[1].y * coord_scale) + offset_y);
				let point_c = Point::new((item.vertices[2].x * coord_scale) + offset_x, (item.vertices[2].y * coord_scale) + offset_y);
				let point_d = Point::new((item.vertices[3].x * coord_scale) + offset_x, (item.vertices[3].y * coord_scale) + offset_y);

				draw_line_segment_mut(&mut img, (point_a.x as f32, point_a.y as f32), (point_b.x as f32, point_b.y as f32), Rgb([255, 0, 0]));
				draw_line_segment_mut(&mut img, (point_b.x as f32, point_b.y as f32), (point_c.x as f32, point_c.y as f32), Rgb([255, 0, 0]));
				draw_line_segment_mut(&mut img, (point_c.x as f32, point_c.y as f32), (point_d.x as f32, point_d.y as f32), Rgb([255, 0, 0]));
				draw_line_segment_mut(&mut img, (point_d.x as f32, point_d.y as f32), (point_a.x as f32, point_a.y as f32), Rgb([255, 0, 0]));
				draw_text_mut(
					&mut img,
					text_color,
					((point_b.x+point_a.x)/2.0) as i32, // Центрирование текста
					((point_b.y+point_a.y)/2.0) as i32,          // Позиция внизу
					font_scale,
					&CACHED_FONT,
					&item.get_value(field).unwrap().iter().cloned().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap().to_string(),
				);
			}
			else if item.vertices.len()==3 {
				let point_a = Point::new((item.vertices[0].x * coord_scale) + offset_x, (item.vertices[0].y * coord_scale) + offset_y);
				let point_b = Point::new((item.vertices[1].x * coord_scale) + offset_x, (item.vertices[1].y * coord_scale) + offset_y);
				let point_c = Point::new((item.vertices[2].x * coord_scale) + offset_x, (item.vertices[2].y * coord_scale) + offset_y);
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
					font_scale,
					&CACHED_FONT,
					// &item.row.as1.iter().cloned().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap().to_string(),
					&item.get_value(field).unwrap().iter().cloned().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap().to_string(),
				);
			}
			}
		}
		// Оптимизированное PNG кодирование
		let mut buffer = Vec::new();
		let cursor = Cursor::new(&mut buffer);
		
		if config.compression_quality > 80 {
			// Высокое качество - медленное сжатие
			let encoder = PngEncoder::new_with_quality(cursor, CompressionType::Best, image::codecs::png::FilterType::Adaptive);
			img.write_with_encoder(encoder).unwrap();
		} else if config.compression_quality > 50 {
			// Среднее качество - быстрое сжатие
			let encoder = PngEncoder::new_with_quality(cursor, CompressionType::Default, image::codecs::png::FilterType::Sub);
			img.write_with_encoder(encoder).unwrap();
		} else {
			// Низкое качество - максимальная скорость
			let encoder = PngEncoder::new_with_quality(cursor, CompressionType::Fast, image::codecs::png::FilterType::Sub);
			img.write_with_encoder(encoder).unwrap();
		}
		
		buffer
	}
	pub async fn draw_all_images(&self) -> Vec<Vec<u8>> {
		self.draw_all_images_optimized(&PerformanceConfig::default()).await
	}

	pub async fn draw_all_images_optimized(&self, config: &PerformanceConfig) -> Vec<Vec<u8>> {
		let fields = ["as1", "as2", "as3", "as4"];
		
		if config.enable_parallel_rendering {
			let futures: Vec<_> = fields
				.iter()
				.map(|field| self.draw_image_as1_optimized(field, config))
				.collect();
			futures::future::join_all(futures).await
		} else {
			let mut results = Vec::new();
			for field in fields.iter() {
				let result = self.draw_image_as1_optimized(field, config).await;
				results.push(result);
			}
			results
		}
	}



	// CPU рендеринг батча изображений
	async fn draw_all_images_cpu_batch(&self, config: &PerformanceConfig) -> Vec<Vec<u8>> {
		// OPTIMIZATION: Замеряем время CPU рендеринга
		web_sys::console::time_with_label("CPU Batch Total");
		let fields = ["as1", "as2", "as3", "as4"];
		
		let results = if config.enable_parallel_rendering {
			// OPTIMIZATION: Параллельная генерация изображений
			web_sys::console::time_with_label("CPU Parallel Processing");
			let futures: Vec<_> = fields
				.iter()
				.map(|field| self.draw_image_as1_optimized(field, config))
				.collect();
			let results = futures::future::join_all(futures).await;
			web_sys::console::time_end_with_label("CPU Parallel Processing");
			results
		} else {
			// OPTIMIZATION: Последовательная генерация для отладки
			web_sys::console::time_with_label("CPU Sequential Processing");
			let mut results = Vec::new();
			for field in &fields {
				let result = self.draw_image_as1_optimized(field, config).await;
				results.push(result);
			}
			web_sys::console::time_end_with_label("CPU Sequential Processing");
			results
		};
		
		web_sys::console::time_end_with_label("CPU Batch Total");
		results
	}

	fn draw_all_images_cpu_batch_sync(&self, config: &PerformanceConfig) -> Vec<Vec<u8>> {
		// OPTIMIZATION: Замеряем время синхронного CPU рендеринга
		web_sys::console::time_with_label("CPU Sync Batch Total");
		let fields = ["as1", "as2", "as3", "as4"];
		let mut results = Vec::new();

		for field in &fields {
			// OPTIMIZATION: Замеряем время отдельного изображения
			web_sys::console::time_with_label(&format!("CPU Sync Image: {}", field));
			
			let dimensions = self.calculate_image_bounds_with_config(config);
			let mut img = ImageBuffer::new(dimensions.img_width, dimensions.img_height);
			
			// OPTIMIZATION: Замеряем время инициализации фона
			web_sys::console::time_with_label("CPU Sync Background Fill");
			// Заполняем белым фоном
			for pixel in img.pixels_mut() {
				*pixel = Rgba([255, 255, 255, 255]);
			}
			web_sys::console::time_end_with_label("CPU Sync Background Fill");

			// OPTIMIZATION: Замеряем время рисования линий
			web_sys::console::time_with_label("CPU Sync Line Drawing");
			// Рендерим линии для этого поля
			let scale_x = dimensions.img_width as f64 / dimensions.content_width;
			let scale_y = dimensions.img_height as f64 / dimensions.content_height;
			
			for item in &self.data {
				if item.entity_type == *field {
					if item.vertices.len() == 4 {
						let v = &item.vertices;
						let x1 = ((v[0].x - dimensions.min_x) * scale_x) as f32;
						let y1 = ((v[0].y - dimensions.min_y) * scale_y) as f32;
						let x2 = ((v[1].x - dimensions.min_x) * scale_x) as f32;
						let y2 = ((v[1].y - dimensions.min_y) * scale_y) as f32;
						let x3 = ((v[2].x - dimensions.min_x) * scale_x) as f32;
						let y3 = ((v[2].y - dimensions.min_y) * scale_y) as f32;
						let x4 = ((v[3].x - dimensions.min_x) * scale_x) as f32;
						let y4 = ((v[3].y - dimensions.min_y) * scale_y) as f32;
						
						// Рисуем линии прямоугольника
						imageproc::drawing::draw_line_segment_mut(&mut img, (x1, y1), (x2, y2), Rgba([255, 0, 0, 255]));
						imageproc::drawing::draw_line_segment_mut(&mut img, (x2, y2), (x3, y3), Rgba([255, 0, 0, 255]));
						imageproc::drawing::draw_line_segment_mut(&mut img, (x3, y3), (x4, y4), Rgba([255, 0, 0, 255]));
						imageproc::drawing::draw_line_segment_mut(&mut img, (x4, y4), (x1, y1), Rgba([255, 0, 0, 255]));
					} else if item.vertices.len() == 3 {
						let v = &item.vertices;
						let x1 = ((v[0].x - dimensions.min_x) * scale_x) as f32;
						let y1 = ((v[0].y - dimensions.min_y) * scale_y) as f32;
						let x2 = ((v[1].x - dimensions.min_x) * scale_x) as f32;
						let y2 = ((v[1].y - dimensions.min_y) * scale_y) as f32;
						let x3 = ((v[2].x - dimensions.min_x) * scale_x) as f32;
						let y3 = ((v[2].y - dimensions.min_y) * scale_y) as f32;
						
						// Рисуем линии треугольника
						imageproc::drawing::draw_line_segment_mut(&mut img, (x1, y1), (x2, y2), Rgba([255, 0, 0, 255]));
						imageproc::drawing::draw_line_segment_mut(&mut img, (x2, y2), (x3, y3), Rgba([255, 0, 0, 255]));
						imageproc::drawing::draw_line_segment_mut(&mut img, (x3, y3), (x1, y1), Rgba([255, 0, 0, 255]));
					}
				}
			}
			web_sys::console::time_end_with_label("CPU Sync Line Drawing");

			// OPTIMIZATION: Замеряем время PNG кодирования
			web_sys::console::time_with_label("CPU Sync PNG Encoding");
			// Конвертируем в PNG
			let mut png_data = Vec::new();
			{
				let encoder = image::codecs::png::PngEncoder::new(&mut png_data);
				if let Err(e) = encoder.write_image(
					img.as_raw(),
					img.width(),
					img.height(),
					image::ColorType::Rgba8,
				) {
					web_sys::console::log_1(&format!("PNG encoding error: {}", e).into());
				}
			}
			web_sys::console::time_end_with_label("CPU Sync PNG Encoding");
			web_sys::console::time_end_with_label(&format!("CPU Sync Image: {}", field));
			results.push(png_data);
		}

		web_sys::console::time_end_with_label("CPU Sync Batch Total");
		results
	}

	// GPU-оптимизированный метод для параллельного рендеринга всех изображений
	pub async fn draw_all_images_gpu_batch(&self, config: &PerformanceConfig) -> Vec<Vec<u8>> {
		if !config.enable_gpu_acceleration {
			return self.draw_all_images_optimized(config).await;
		}

		let fields = ["as1", "as2", "as3", "as4"];
		let mut images = Vec::new();
		let mut all_lines = Vec::new();
		let mut colors = Vec::new();

		// Подготавливаем все изображения и данные для батчевого рендеринга
		for field in &fields {
			let dimensions = self.calculate_image_bounds_with_config(config);
			let mut img = ImageBuffer::new(dimensions.img_width, dimensions.img_height);
			
			// Заполняем белым фоном
			for pixel in img.pixels_mut() {
				*pixel = Rgba([255, 255, 255, 255]);
			}

			// Собираем линии для этого изображения
			let mut lines_for_image = Vec::new();
			let coord_scale = (dimensions.img_width as f64 / dimensions.content_width).min(dimensions.img_height as f64 / dimensions.content_height) * 1.15;
			let offset_x = (dimensions.img_width as f64 - dimensions.content_width * coord_scale) / 2.0 - dimensions.min_x * coord_scale;
			let offset_y = (dimensions.img_height as f64 - dimensions.content_height * coord_scale) / 2.0 - dimensions.min_y * coord_scale;
			
			for item in &self.data {
				if item.entity_type == *field && item.vertices.len() == 4 {
					// Добавляем линии для прямоугольника
					let v = &item.vertices;
					
					let x1 = (v[0].x * coord_scale + offset_x) as f32;
					let y1 = (v[0].y * coord_scale + offset_y) as f32;
					let x2 = (v[1].x * coord_scale + offset_x) as f32;
					let y2 = (v[1].y * coord_scale + offset_y) as f32;
					let x3 = (v[2].x * coord_scale + offset_x) as f32;
					let y3 = (v[2].y * coord_scale + offset_y) as f32;
					let x4 = (v[3].x * coord_scale + offset_x) as f32;
					let y4 = (v[3].y * coord_scale + offset_y) as f32;
					
					lines_for_image.push((x1, y1, x2, y2));
					lines_for_image.push((x2, y2, x3, y3));
					lines_for_image.push((x3, y3, x4, y4));
					lines_for_image.push((x4, y4, x1, y1));
				}
			}

			images.push(img);
			all_lines.push(lines_for_image);
			colors.push([0, 0, 0, 255]); // Черный цвет
		}

		
		// Выполняем рендеринг с автоматическим выбором метода
		if let Some(gpu_renderer) = crate::libs::gpu_renderer::get_gpu_renderer() {
			let batch_data_for_analysis: Vec<_> = images.iter()
				.zip(all_lines.iter())
				.zip(colors.iter())
				.map(|((img, lines), color)| (img.clone(), lines.clone(), *color))
				.collect();

			// Определяем оптимальный метод рендеринга
			let render_method = gpu_renderer.determine_render_method(&batch_data_for_analysis);
			web_sys::console::time_end_with_label("Render Method Analysis");
			
			match render_method {
				crate::libs::gpu_renderer::RenderMethod::Gpu => {
					// OPTIMIZATION: Замеряем время подготовки данных для GPU
					web_sys::console::time_with_label("GPU Data Preparation");
					let mut batch_data: Vec<_> = images.iter_mut()
						.zip(all_lines.iter())
						.zip(colors.iter())
						.map(|((img, lines), color)| (img, lines.as_slice(), *color))
						.collect();
					web_sys::console::time_end_with_label("GPU Data Preparation");

					// OPTIMIZATION: Замеряем время GPU рендеринга
					web_sys::console::time_with_label("GPU Batch Rendering");
					if let Err(e) = gpu_renderer.render_lines_gpu_batch(&mut batch_data).await {
						web_sys::console::time_end_with_label("GPU Batch Rendering");
						console::log_1(&format!("GPU batch rendering failed: {}, falling back to CPU batch", e).into());
						return self.draw_all_images_cpu_batch(config).await;
					}
					web_sys::console::time_end_with_label("GPU Batch Rendering");
				},
				crate::libs::gpu_renderer::RenderMethod::Cpu => {
					console::log_1(&"Используем CPU батчевый рендеринг по рекомендации анализатора".into());
					return self.draw_all_images_cpu_batch(config).await;
				}
			}
		} else {
			web_sys::console::time_end_with_label("Render Method Analysis");
			web_sys::console::log_1(&"GPU renderer not available, falling back to CPU batch".into());
			return self.draw_all_images_cpu_batch(config).await;
		}

		// OPTIMIZATION: Замеряем время PNG кодирования
		web_sys::console::time_with_label("PNG Encoding");
		let mut results = Vec::new();
		for img in images {
			let mut png_data = Vec::new();
			{
				let encoder = image::codecs::png::PngEncoder::new(&mut png_data);
				if let Err(e) = encoder.write_image(
					img.as_raw(),
					img.width(),
					img.height(),
					image::ColorType::Rgba8,
				) {
					web_sys::console::log_1(&format!("PNG encoding error: {}", e).into());
				}
			}
			results.push(png_data);
		}
		web_sys::console::time_end_with_label("PNG Encoding");
		web_sys::console::time_end_with_label("Total GPU Rendering");

		results
	}


	
	// Новый метод для асинхронной генерации с прогрессом
	pub async fn draw_all_images_with_progress<F>(&self, mut progress_callback: F) -> Vec<Vec<u8>>
	where
		F: FnMut(usize, usize),
	{
		let fields = ["as1", "as2", "as3", "as4"];
		let total = fields.len();
		let mut results = Vec::new();
		
		for (index, field) in fields.iter().enumerate() {
			let result = self.draw_image_as1(field).await;
			progress_callback(index + 1, total);
			results.push(result);
		}
		
		results
	}
	pub fn log_to_data(&self){
		string_log_two_params("Это после сортировки по z",&serde_json::to_string_pretty(&self.data).unwrap());
	}
}


