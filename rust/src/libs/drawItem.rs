use std::io::Cursor;
use std::sync::{Arc};

use image::{ImageBuffer, Rgb, Rgba, ImageEncoder, codecs::png::{PngEncoder, CompressionType}};
use imageproc::{drawing::{draw_line_segment_mut, draw_text_mut, draw_filled_rect_mut, draw_polygon_mut}, point::Point, rect::Rect};
use rusttype::{Font, Scale};
use serde::Serialize;
use web_sys::console;

use crate::string_log_two_params;
use super::parse::EntityWithXlsx;
use super::generate_documents::performance::{PerformanceConfig, PerformanceMonitor};
use super::gpu_renderer::{init_gpu_renderer, get_gpu_renderer, is_gpu_available};

// ЕДИНЫЕ КОНСТАНТЫ A4 ДЛЯ ВСЕГО ПРОЕКТА - ПРАВИЛЬНЫЕ ПРОПОРЦИИ!
const A4_WIDTH_MM: f64 = 210.0;  // Ширина A4 в миллиметрах
const A4_HEIGHT_MM: f64 = 297.0; // Высота A4 в миллиметрах (БОЛЬШЕ ширины!)
const IMAGE_COVERAGE_PERCENT: f64 = 0.9; // Изображение занимает 90% страницы
const MARGIN_MM: f64 = 5.0; // Отступы 0.5 см = 5 мм
const DPI: f64 = 300.0; // Разрешение для печати
const MM_TO_PIXELS: f64 = DPI / 25.4; // Конвертация мм в пиксели (25.4 мм = 1 дюйм)

// ЕДИНЫЕ РАЗМЕРЫ DOCX - ПРАВИЛЬНЫЕ ПРОПОРЦИИ A4 (высота > ширины)!
// УМЕНЬШЕНО: Размеры изображения для лучшего размещения с отступами
pub const DOCX_IMAGE_WIDTH_TWIPS: u32 = 9500;   // Уменьшено для отступов
pub const DOCX_IMAGE_HEIGHT_TWIPS: u32 = 13430; // Уменьшено пропорционально (соотношение 0.707)
// ИСПРАВЛЕНО: Правильные размеры страницы A4 PORTRAIT (высота > ширины)!
pub const DOCX_PAGE_WIDTH_TWIPS: u32 = 11906;   // A4 portrait ширина (МЕНЬШЕ)
pub const DOCX_PAGE_HEIGHT_TWIPS: u32 = 16838;  // A4 portrait высота (БОЛЬШЕ)

// ЕДИНЫЕ РАЗМЕРЫ В EMU - ПРАВИЛЬНЫЕ ПРОПОРЦИИ A4!
// УМЕНЬШЕНО: Размеры в EMU для лучшего размещения с отступами
pub const DOCX_IMAGE_WIDTH_EMU: u32 = 6804000;   // ~18.9 см (уменьшено для отступов)
pub const DOCX_IMAGE_HEIGHT_EMU: u32 = 9627000;  // ~26.7 см (уменьшено пропорционально)

pub enum AsFunctions  {
	As1,
	As2,
	As3,
	As4
}

// Упрощенная структура для хранения размеров контента и границ
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
	/// Простой конструктор с фиксированными размерами A4
	pub fn new_a4_simple(min_x: f64, min_y: f64, max_x: f64, max_y: f64) -> Self {
		// ПРАВИЛЬНЫЕ размеры A4 - ПОЛНЫЕ размеры без процентов!
		// Отступы 5мм будут учтены в алгоритме масштабирования
		let img_width = (A4_WIDTH_MM * MM_TO_PIXELS) as u32;  // 210мм
		let img_height = (A4_HEIGHT_MM * MM_TO_PIXELS) as u32; // 297мм
		
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
	
	/// Старый конструктор для обратной совместимости
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


	fn calculate_image_bounds_with_config(&self, _config: &PerformanceConfig) -> ContentDimensions {
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
		
		// Если контент пустой, возвращаем размеры по умолчанию
		if content_width <= 0.0 || content_height <= 0.0 {
			return ContentDimensions::new(
				min_x, min_y, max_x, max_y, 
				(A4_WIDTH_MM * IMAGE_COVERAGE_PERCENT * MM_TO_PIXELS) as u32,
				(A4_HEIGHT_MM * IMAGE_COVERAGE_PERCENT * MM_TO_PIXELS) as u32
			);
		}
		
		// Используем простой конструктор с фиксированными размерами A4
		ContentDimensions::new_a4_simple(min_x, min_y, max_x, max_y)
	}

	// CPU fallback функция
	fn render_item_cpu_fallback(&self, item: &EntityWithXlsx, img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, 
								coord_scale: f64, offset_x: f64, offset_y: f64, field: &str, 
								font_scale: Scale, text_color: Rgb<u8>, dimensions: &ContentDimensions) {
		if item.vertices.len() == 4 {
			let points: Vec<Point<f64>> = item.vertices.iter().map(|v| {
				let normalized_x = v.x - dimensions.min_x;
				let normalized_y = v.y - dimensions.min_y;
				Point::new(normalized_x * coord_scale + offset_x, normalized_y * coord_scale + offset_y)
			}).collect();
			
			// Преобразуем точки для функции заливки четырехугольника
			let quad_points: Vec<Point<i32>> = points.iter().map(|p| {
				Point::new(p.x as i32, p.y as i32)
			}).collect();
			
			// Заливаем четырехугольник темно-желтым цветом
			draw_polygon_mut(img, &quad_points, Rgb([204, 204, 0]));
		} else if item.vertices.len() == 3 {
			let points: Vec<Point<f64>> = item.vertices.iter().map(|v| {
				let normalized_x = v.x - dimensions.min_x;
				let normalized_y = v.y - dimensions.min_y;
				Point::new(normalized_x * coord_scale + offset_x, normalized_y * coord_scale + offset_y)
			}).collect();
			
			// Преобразуем точки для функции заливки треугольника
			let triangle_points: Vec<Point<i32>> = points.iter().map(|p| {
				Point::new(p.x as i32, p.y as i32)
			}).collect();
			
			// Заливаем треугольник темно-желтым цветом
			draw_polygon_mut(img, &triangle_points, Rgb([204, 204, 0]));
		}
		
		self.render_text_cpu(item, img, coord_scale, offset_x, offset_y, field, font_scale, text_color, dimensions);
	}
	
	// Функция рендеринга текста на CPU
	fn render_text_cpu(&self, item: &EntityWithXlsx, img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, 
					   coord_scale: f64, offset_x: f64, offset_y: f64, field: &str, 
					   font_scale: Scale, text_color: Rgb<u8>, dimensions: &ContentDimensions) {
		if let Some(values) = item.get_value(field) {
			if let Some(max_value) = values.iter().cloned().max_by(|a, b| a.partial_cmp(b).unwrap()) {
				let points: Vec<Point<f64>> = item.vertices.iter().map(|v| {
					let normalized_x = v.x - dimensions.min_x;
					let normalized_y = v.y - dimensions.min_y;
					Point::new(normalized_x * coord_scale + offset_x, normalized_y * coord_scale + offset_y)
				}).collect();
				
				if points.len() >= 2 {
					// Вычисляем границы фигуры
					let min_x = points.iter().map(|p| p.x).fold(f64::INFINITY, f64::min);
					let max_x = points.iter().map(|p| p.x).fold(f64::NEG_INFINITY, f64::max);
					let min_y = points.iter().map(|p| p.y).fold(f64::INFINITY, f64::min);
					let max_y = points.iter().map(|p| p.y).fold(f64::NEG_INFINITY, f64::max);
					
					// Отступы 10% от размеров фигуры
					let width = max_x - min_x;
					let height = max_y - min_y;
					let text_x = (min_x + width * 0.1) as i32;  // 10% отступ слева
					let text_y = (min_y + height * 0.1) as i32; // 10% отступ сверху
					
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
		
		// ОТКЛЮЧАЕМ GPU - используем только CPU для быстрой генерации
		let use_gpu = false;
		
		// Режим рендеринга определен
		
		let mut img = ImageBuffer::from_fn(dimensions.img_width, dimensions.img_height, |_, _| Rgb([255u8, 255u8, 255u8]));
		
		// ПРАВИЛЬНАЯ логика масштабирования с фиксированными отступами 5мм
		// Фиксированные отступы 5мм от всех краев (НЕ проценты!)
		let margin_pixels = MARGIN_MM * MM_TO_PIXELS; // 5мм в пикселях
		
		// Доступная область для рисования (вся картинка минус отступы)
		let available_width_pixels = dimensions.img_width as f64 - 2.0 * margin_pixels;
		let available_height_pixels = dimensions.img_height as f64 - 2.0 * margin_pixels;
		
		// ПРАВИЛЬНЫЙ расчет единого масштаба для X и Y
		// Вычисляем отдельные масштабы
		let scale_x = available_width_pixels / dimensions.content_width;
		let scale_y = available_height_pixels / dimensions.content_height;
		
		// Выбираем МИНИМАЛЬНЫЙ масштаб - это гарантирует:
		// 1. Одинаковый масштаб для X и Y (сохранение пропорций)
		// 2. Весь контент поместится без обрезки
		let coord_scale = scale_x.min(scale_y);
		
		// Вычисляем реальные размеры масштабированного контента
		let scaled_content_width = dimensions.content_width * coord_scale;
		let scaled_content_height = dimensions.content_height * coord_scale;
		
		// ПРАВИЛЬНОЕ центрирование масштабированного контента
		// Центрируем в доступной области (между отступами)
		let offset_x = margin_pixels + (available_width_pixels - scaled_content_width) / 2.0;
		let offset_y = margin_pixels + (available_height_pixels - scaled_content_height) / 2.0;

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
					// Правильное преобразование координат: нормализация -> масштабирование -> центрирование
					let points: Vec<Point<f64>> = item.vertices.iter().map(|v| {
						let normalized_x = v.x - dimensions.min_x; // Нормализация
						let normalized_y = v.y - dimensions.min_y;
						Point::new(
							normalized_x * coord_scale + offset_x, // Масштабирование + центрирование
							normalized_y * coord_scale + offset_y
						)
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
						let normalized_x = v.x - dimensions.min_x;
						let normalized_y = v.y - dimensions.min_y;
						Point::new(normalized_x * coord_scale + offset_x, normalized_y * coord_scale + offset_y)
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
				if let Err(_e) = gpu_renderer.render_lines_gpu(&mut rgba_img, &all_lines, [255, 0, 0, 255]).await {
					// Fallback на CPU рендеринг
					for item in &self.data {
						self.render_item_cpu_fallback(item, &mut img, coord_scale, offset_x, offset_y, field, font_scale, text_color, &dimensions);
					}
				} else {
					// Конвертируем обратно в RGB
					for (x, y, rgba_pixel) in rgba_img.enumerate_pixels() {
						img.put_pixel(x, y, image::Rgb([rgba_pixel[0], rgba_pixel[1], rgba_pixel[2]]));
					}
					
					// Рендерим текст на CPU после GPU рендеринга линий
					for item in &self.data {
						self.render_text_cpu(item, &mut img, coord_scale, offset_x, offset_y, field, font_scale, text_color, &dimensions);
					}
				}
			}
		} else {
			// Быстрый CPU рендеринг с заливкой и контурами
			for item in &self.data {
				if item.vertices.len() == 4 {
					// Преобразуем координаты
					let points: Vec<Point<f64>> = item.vertices.iter().map(|v| {
						let normalized_x = v.x - dimensions.min_x;
						let normalized_y = v.y - dimensions.min_y;
						Point::new(normalized_x * coord_scale + offset_x, normalized_y * coord_scale + offset_y)
					}).collect();
					
					// Заливка четырехугольника
					let quad_points: Vec<Point<i32>> = points.iter().map(|p| {
						Point::new(p.x as i32, p.y as i32)
					}).collect();
					draw_polygon_mut(&mut img, &quad_points, Rgb([204, 204, 0]));
					
					// Контуры поверх заливки
					for i in 0..4 {
						let next = (i + 1) % 4;
						draw_line_segment_mut(&mut img, 
							(points[i].x as f32, points[i].y as f32), 
							(points[next].x as f32, points[next].y as f32), 
							Rgb([0, 0, 0])); // Черные контуры
					}
					
					// Рендерим текст с правильными значениями из AS функций
					if let Some(values) = item.get_value(field) {
						if let Some(max_value) = values.iter().cloned().max_by(|a, b| a.partial_cmp(b).unwrap()) {
							// Вычисляем границы четырехугольника для правильного позиционирования текста
							let min_x = points.iter().map(|p| p.x).fold(f64::INFINITY, f64::min);
							let max_x = points.iter().map(|p| p.x).fold(f64::NEG_INFINITY, f64::max);
							let min_y = points.iter().map(|p| p.y).fold(f64::INFINITY, f64::min);
							let max_y = points.iter().map(|p| p.y).fold(f64::NEG_INFINITY, f64::max);
							
							// Отступы 10% от размеров фигуры
							let width = max_x - min_x;
							let height = max_y - min_y;
							let text_x = (min_x + width * 0.1) as i32;  // 10% отступ слева
							let text_y = (min_y + height * 0.1) as i32; // 10% отступ сверху
							
							draw_text_mut(
								&mut img,
								text_color,
								text_x,
								text_y,
								font_scale,
								&CACHED_FONT,
								&max_value.to_string(),
							);
						}
					}
				} else if item.vertices.len() == 3 {
					// Преобразуем координаты
					let points: Vec<Point<f64>> = item.vertices.iter().map(|v| {
						let normalized_x = v.x - dimensions.min_x;
						let normalized_y = v.y - dimensions.min_y;
						Point::new(normalized_x * coord_scale + offset_x, normalized_y * coord_scale + offset_y)
					}).collect();
					
					// Заливка треугольника
					let triangle_points: Vec<Point<i32>> = points.iter().map(|p| {
						Point::new(p.x as i32, p.y as i32)
					}).collect();
					draw_polygon_mut(&mut img, &triangle_points, Rgb([204, 204, 0]));
					
					// Контуры поверх заливки
					for i in 0..3 {
						let next = (i + 1) % 3;
						draw_line_segment_mut(&mut img,
							(points[i].x as f32, points[i].y as f32),
							(points[next].x as f32, points[next].y as f32),
							Rgb([0, 0, 0])); // Черные контуры
					}
					
					// Рендерим текст с правильными значениями из AS функций
					if let Some(values) = item.get_value(field) {
						if let Some(max_value) = values.iter().cloned().max_by(|a, b| a.partial_cmp(b).unwrap()) {
							// Вычисляем границы треугольника для правильного позиционирования текста
							let min_x = points.iter().map(|p| p.x).fold(f64::INFINITY, f64::min);
							let max_x = points.iter().map(|p| p.x).fold(f64::NEG_INFINITY, f64::max);
							let min_y = points.iter().map(|p| p.y).fold(f64::INFINITY, f64::min);
							let max_y = points.iter().map(|p| p.y).fold(f64::NEG_INFINITY, f64::max);
							
							// Отступы 10% от размеров фигуры
							let width = max_x - min_x;
							let height = max_y - min_y;
							let text_x = (min_x + width * 0.1) as i32;  // 10% отступ слева
							let text_y = (min_y + height * 0.1) as i32; // 10% отступ сверху
							
							draw_text_mut(
								&mut img,
								text_color,
								text_x,
								text_y,
								font_scale,
								&CACHED_FONT,
								&max_value.to_string(),
							);
						}
					}
				}
				// Рендерим ТОЛЬКО правильные значения из AS функций
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
			let mut results = Vec::with_capacity(4); // 4 результата для полей
			for field in fields.iter() {
				let result = self.draw_image_as1_optimized(field, config).await;
				results.push(result);
			}
			results
		}
	}

	// CPU рендеринг батча изображений
	async fn draw_all_images_cpu_batch(&self, config: &PerformanceConfig) -> Vec<Vec<u8>> {
		// CPU рендеринг батча
		let fields = ["as1", "as2", "as3", "as4"];
		
		let results = if config.enable_parallel_rendering {
			// Параллельная генерация изображений
			let futures: Vec<_> = fields
				.iter()
				.map(|field| self.draw_image_as1_optimized(field, config))
				.collect();
			let results = futures::future::join_all(futures).await;
			results
		} else {
			// Последовательная генерация
			let mut results = Vec::with_capacity(4); // 4 результата для полей
			for field in &fields {
				let result = self.draw_image_as1_optimized(field, config).await;
				results.push(result);
			}
			results
		};
		
		// CPU рендеринг завершен
		results
	}

	// GPU-оптимизированный метод для параллельного рендеринга всех изображений
	pub async fn draw_all_images_gpu_batch(&self, config: &PerformanceConfig) -> Vec<Vec<u8>> {
		if !config.enable_gpu_acceleration {
			return self.draw_all_images_optimized(config).await;
		}

		let fields = ["as1", "as2", "as3", "as4"];
		let mut images = Vec::with_capacity(4); // 4 изображения для полей
		let mut all_lines = Vec::with_capacity(100000); // Примерная оценка линий
		let mut colors = Vec::with_capacity(4); // 4 цвета для полей

		// Подготавливаем все изображения и данные для батчевого рендеринга
		for field in &fields {
			let dimensions = self.calculate_image_bounds_with_config(config);
			let mut img = ImageBuffer::new(dimensions.img_width, dimensions.img_height);
			
			// Заполняем белым фоном
			for pixel in img.pixels_mut() {
				*pixel = Rgba([255, 255, 255, 255]);
			}

			// Собираем линии для этого изображения с ПРАВИЛЬНЫМ масштабированием
			let mut lines_for_image = Vec::with_capacity(250); // Примерно 1000/4 линий на изображение
			
			// ПРАВИЛЬНЫЙ расчет масштаба с фиксированными отступами 5мм
			let margin_pixels = MARGIN_MM * MM_TO_PIXELS;
			let available_width = dimensions.img_width as f64 - 2.0 * margin_pixels;
			let available_height = dimensions.img_height as f64 - 2.0 * margin_pixels;
			let scale_x = available_width / dimensions.content_width;
			let scale_y = available_height / dimensions.content_height;
			let coord_scale = scale_x.min(scale_y); // ЕДИНЫЙ масштаб!
			
			// ПРАВИЛЬНОЕ центрирование
			let scaled_content_width = dimensions.content_width * coord_scale;
			let scaled_content_height = dimensions.content_height * coord_scale;
			let offset_x = margin_pixels + (available_width - scaled_content_width) / 2.0;
			let offset_y = margin_pixels + (available_height - scaled_content_height) / 2.0;
			
			for item in &self.data {
				if item.entity_type == *field && item.vertices.len() == 4 {
					// Добавляем линии для прямоугольника с ПРАВИЛЬНОЙ нормализацией
					let v = &item.vertices;
					
					// ПРАВИЛЬНОЕ преобразование: нормализация -> масштаб -> центр
					let x1 = ((v[0].x - dimensions.min_x) * coord_scale + offset_x) as f32;
					let y1 = ((v[0].y - dimensions.min_y) * coord_scale + offset_y) as f32;
					let x2 = ((v[1].x - dimensions.min_x) * coord_scale + offset_x) as f32;
					let y2 = ((v[1].y - dimensions.min_y) * coord_scale + offset_y) as f32;
					let x3 = ((v[2].x - dimensions.min_x) * coord_scale + offset_x) as f32;
					let y3 = ((v[2].y - dimensions.min_y) * coord_scale + offset_y) as f32;
					let x4 = ((v[3].x - dimensions.min_x) * coord_scale + offset_x) as f32;
					let y4 = ((v[3].y - dimensions.min_y) * coord_scale + offset_y) as f32;
					
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
		// ПРИНУДИТЕЛЬНО используем CPU для быстрой генерации
		return self.draw_all_images_cpu_batch(config).await;

		// PNG кодирование
		let mut results = Vec::with_capacity(4); // 4 результата для полей
		for img in images {
			let mut png_data = Vec::new();
			{
				let encoder = image::codecs::png::PngEncoder::new(&mut png_data);
				let _ = encoder.write_image(
					img.as_raw(),
					img.width(),
					img.height(),
					image::ColorType::Rgba8,
				);
			}
			results.push(png_data);
		}
		// PNG кодирование завершено

		results
	}


	
	// Новый метод для асинхронной генерации с прогрессом
	pub async fn draw_all_images_with_progress<F>(&self, mut progress_callback: F) -> Vec<Vec<u8>>
	where
		F: FnMut(usize, usize),
	{
		let fields = ["as1", "as2", "as3", "as4"];
		let total = fields.len();
		let mut results = Vec::with_capacity(4); // 4 результата для полей
		
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


