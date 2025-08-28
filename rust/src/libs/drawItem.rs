use std::io::Cursor;
use std::sync::{Arc};
use image::{ImageBuffer, Rgb, Rgba, ImageEncoder, codecs::png::{PngEncoder, CompressionType}};
use imageproc::{drawing::{draw_line_segment_mut, draw_text_mut, draw_filled_rect_mut, draw_polygon_mut}, point::Point, rect::Rect};
use rusttype::{Font, Scale};
use serde::Serialize;
use web_sys::console;
use super::parse::EntityWithXlsx;
use super::generate_documents::performance::{PerformanceConfig, PerformanceMonitor};
use super::gpu_renderer::{ get_gpu_renderer,};
// Функция генерации цветовой палитры
fn generate_color_palette(scale: &str) -> Vec<Rgb<u8>> {
	// Определяем количество диапазонов в result_scale
	let ranges = parse_result_scale_ranges(scale);
	let num_colors = if ranges.is_empty() { 1 } else { ranges.len() };
	
	// Базовые цвета для интерполяции
	let base_colors = vec![
		Rgb([247, 233, 171]), // #f7e9ab - светло-желтый
		Rgb([255, 255, 0]),   // #ffff00 - желтый
		Rgb([247, 172, 52]),  // #f7ac34 - оранжево-желтый
		Rgb([232, 145, 5]),   // #e89105 - оранжевый
		Rgb([232, 96, 5]),    // #e86005 - темно-оранжевый
		Rgb([139, 0, 0]),     // #8b0000 - бордовый
	];
	
	// Если нужно меньше цветов, берем первые
	if num_colors <= base_colors.len() {
		base_colors.into_iter().take(num_colors).collect()
	} else {
		// Если нужно больше цветов, интерполируем
		let mut result = Vec::new();
		for i in 0..num_colors {
			let ratio = i as f32 / (num_colors - 1).max(1) as f32;
			let index = (ratio * (base_colors.len() - 1) as f32) as usize;
			result.push(base_colors[index.min(base_colors.len() - 1)]);
		}
		result
	}
}

// Функция получения цвета для значения на основе диапазонов result_scale
fn get_color_for_value(item: &EntityWithXlsx, field: &str, scale: Option<&str>, palette: &[Rgb<u8>]) -> Rgb<u8> {
	if let Some(values) = item.get_value(field) {
		if let Some(max_value) = values.iter().cloned().max_by(|a, b| a.partial_cmp(b).unwrap()) {
			// Если есть result_scale, используем диапазоны из него
			if let Some(scale_str) = scale {
				let ranges = parse_result_scale_ranges(scale_str);
				if !ranges.is_empty() {
					// Находим подходящий диапазон для значения
					for (i, (min_val, max_val)) in ranges.iter().enumerate() {
						if max_value >= *min_val && max_value <= *max_val {
							return palette.get(i).copied().unwrap_or(palette[0]);
						}
					}
					// Если значение больше всех диапазонов, используем последний цвет
					if max_value > ranges.last().unwrap().1 {
						return palette.get(ranges.len() - 1).copied().unwrap_or(palette[0]);
					}
					// Если значение меньше всех диапазонов, используем первый цвет
					return palette[0];
				}
			}
			// Старая логика для случаев без result_scale или пустых диапазонов
			let normalized = (max_value / 100.0).min(1.0).max(0.0);
			let index = (normalized * (palette.len() - 1) as f32) as usize;
			return palette[index.min(palette.len() - 1)];
		}
	}
	palette[0] // Дефолтный цвет
}

// Функция парсинга диапазонов result_scale (вынесена из impl блока)
fn parse_result_scale_ranges(scale: &str) -> Vec<(f32, f32)> {
	// Парсим строку вида "[2.515см2:Ø8 мм][3.930см2:Ø8+Ø6][5.030см2:Ø8+Ø8]"
	let mut areas = Vec::new();
	let parts: Vec<&str> = scale.split("][").collect();
	
	// Извлекаем все площади
	for part in parts {
		let clean_part = part.trim_start_matches('[').trim_end_matches(']');
		if let Some(area_end) = clean_part.find("см2:") {
			if let Ok(area) = clean_part[..area_end].parse::<f32>() {
				areas.push(area);
			}
		}
	}
	
	// Сортируем площади
	areas.sort_by(|a, b| a.partial_cmp(b).unwrap());
	
	// Создаем диапазоны между соседними значениями
	let mut ranges = Vec::new();
	if !areas.is_empty() {
		// Первый диапазон: от 0 до первого значения
		ranges.push((0.0, areas[0]));
		
		// Промежуточные диапазоны: от предыдущего до текущего
		for i in 1..areas.len() {
			ranges.push((areas[i-1], areas[i]));
		}
		
		// Если есть только одно значение, добавляем еще один диапазон
		if areas.len() == 1 {
			ranges.push((areas[0], areas[0] * 2.0));
		}
	}
	
	ranges
}



// ЕДИНЫЕ КОНСТАНТЫ A4 ДЛЯ ВСЕГО ПРОЕКТА - ПРАВИЛЬНЫЕ ПРОПОРЦИИ!
const A4_WIDTH_MM: f64 = 210.0;  // Ширина A4 в миллиметрах
const A4_HEIGHT_MM: f64 = 297.0; // Высота A4 в миллиметрах (БОЛЬШЕ ширины!)
const IMAGE_COVERAGE_PERCENT: f64 = 0.9; // Изображение занимает 90% страницы
const MARGIN_MM: f64 = 2.0; // Константные отступы 2 мм для правильного позиционирования
const DPI: f64 = 300.0; // Разрешение для печати
const MM_TO_PIXELS: f64 = DPI / 25.4; // Конвертация мм в пиксели (25.4 мм = 1 дюйм)

// ЕДИНЫЕ РАЗМЕРЫ DOCX - ПРАВИЛЬНЫЕ ПРОПОРЦИИ A4 (высота > ширины)!
// УВЕЛИЧЕНО: Размеры изображения для максимального использования пространства с минимальными отступами
pub const DOCX_IMAGE_WIDTH_TWIPS: u32 = 11300;  // Увеличено благодаря минимальным отступам
pub const DOCX_IMAGE_HEIGHT_TWIPS: u32 = 16000;  // Увеличено пропорционально
// ИСПРАВЛЕНО: Правильные размеры страницы A4 PORTRAIT (высота > ширины)!
pub const DOCX_PAGE_WIDTH_TWIPS: u32 = 11906;   // A4 portrait ширина (МЕНЬШЕ)
pub const DOCX_PAGE_HEIGHT_TWIPS: u32 = 16838;  // A4 portrait высота (БОЛЬШЕ)

// ЕДИНЫЕ РАЗМЕРЫ В EMU - ПРАВИЛЬНЫЕ ПРОПОРЦИИ A4!
// УВЕЛИЧЕНО: Размеры в EMU для максимального использования пространства
pub const DOCX_IMAGE_WIDTH_EMU: u32 = 8100000;   // ~22.5 см (увеличено благодаря минимальным отступам)
pub const DOCX_IMAGE_HEIGHT_EMU: u32 = 11430000;  // ~31.7 см (увеличено пропорционально)

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
		self.draw_image(field, None).await
	}

	pub async fn draw_image_as1_optimized(&self, field: &str, config: &PerformanceConfig) -> Vec<u8> {
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
		
		// Доступная область для рисования с асимметричными отступами
		// Слева и сверху константные отступы 2мм, справа и снизу оптимизированные
		let right_margin_pixels = margin_pixels * 1.5; // Небольшой отступ справа (3мм)
		let bottom_margin_pixels = margin_pixels * 1.5; // Небольшой отступ снизу (3мм)
		let available_width_pixels = dimensions.img_width as f64 - margin_pixels - right_margin_pixels;
		let available_height_pixels = dimensions.img_height as f64 - margin_pixels - bottom_margin_pixels;
		
		// ТОЧНЫЙ расчет масштаба для предотвращения выхода за границы
		// Вычисляем отдельные масштабы
		let scale_x = available_width_pixels / dimensions.content_width;
		let scale_y = available_height_pixels / dimensions.content_height;
		
		// Используем точный масштаб 96.8% для предотвращения обрезки
		// Это обеспечивает идеальное размещение без выхода за границы
		let safety_margin = 0.968; // Точно 96.8% как требуется
		let coord_scale = scale_x.min(scale_y) * safety_margin;
		
		// Вычисляем реальные размеры масштабированного контента
		let scaled_content_width = dimensions.content_width * coord_scale;
		let scaled_content_height = dimensions.content_height * coord_scale;
		
		// ЭКСПЕРИМЕНТАЛЬНОЕ позиционирование: нулевой отступ слева для максимального использования
		// Убираем горизонтальное центрирование, начинаем рисовать от левого края
		let offset_x = margin_pixels; // Нулевой отступ слева (только базовый margin)
		let offset_y = margin_pixels + (available_height_pixels - scaled_content_height) / 2.0; // Вертикальное центрирование сохраняем

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
					
					// Простая заливка без автоматических цветов
					let fill_color = Rgb([204, 204, 0]); // Дефолтный темно-желтый
					
					draw_polygon_mut(&mut img, &quad_points, fill_color);
					
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
		
		// Простое PNG кодирование без сложностей
        let encoder = PngEncoder::new(cursor);
        img.write_with_encoder(encoder).unwrap();
		
		buffer
	}
	/// ЕДИНСТВЕННАЯ ФУНКЦИЯ ДЛЯ ВСЕХ ИЗОБРАЖЕНИЙ - ПРОСТАЯ И БЫСТРАЯ
	/// Автоматически применяет цвета если есть result_scales
	pub async fn draw_all_images(&self) -> Vec<Vec<u8>> {
		self.draw_all_images_with_colors(None).await
	}
	
	/// Функция для всех изображений с поддержкой цветов
	pub async fn draw_all_images_with_colors(&self, result_scales: Option<&[Option<&str>]>) -> Vec<Vec<u8>> {
        self.draw_all_images_with_colors_and_floor(result_scales, 0.0).await
    }
    
    /// Функция для всех изображений с поддержкой цветов и floor_level
    pub async fn draw_all_images_with_colors_and_floor(&self, result_scales: Option<&[Option<&str>]>, floor_level: f32) -> Vec<Vec<u8>> {
        // 🔍 PERFORMANCE: Засекаем общее время генерации всех изображений
        let total_start = web_sys::window().unwrap().performance().unwrap().now();
        
        web_sys::console::log_1(&format!(
            "🚀 [DOCX-GEN] Starting generation of 4 images for {} entities", 
            self.data.len()
        ).into());
        
        let fields = ["as1", "as2", "as3", "as4"];
        let mut results = Vec::with_capacity(4);
        
        for (i, field) in fields.iter().enumerate() {
            let result_scale = result_scales.and_then(|scales| scales.get(i)).and_then(|s| *s);
            
            let result = self.draw_image_with_floor(field, result_scale, floor_level).await;
            results.push(result);
        }
        
        // 🔍 PERFORMANCE: Финальные метрики по всем изображениям
        let total_time = web_sys::window().unwrap().performance().unwrap().now() - total_start;
        let avg_per_image = total_time / 4.0;
        let total_size: usize = results.iter().map(|r| r.len()).sum();
        
        web_sys::console::log_1(&format!(
            "✅ [DOCX-GEN] All 4 images completed: Total={:.1}ms, Avg={:.1}ms/image, Size={:.1}MB", 
            total_time, avg_per_image, total_size as f64 / 1024.0 / 1024.0
        ).into());
        
        results
	}

	/// Основная функция рендеринга с поддержкой цветов
	pub async fn draw_image(&self, field: &str, result_scale: Option<&str>) -> Vec<u8> {
        self.draw_image_with_floor(field, result_scale, 0.0).await
    }
    
    pub async fn draw_image_with_floor(&self, field: &str, result_scale: Option<&str>, floor_level: f32) -> Vec<u8> {
        // 🔍 PERFORMANCE: Начало генерации изображения
        let start_time = web_sys::window().unwrap().performance().unwrap().now();
        
        // Используем дефолтные настройки для простоты
        let dimensions = self.calculate_image_bounds_with_config(&PerformanceConfig::default());
        
        // Генерируем цветовую палитру если есть result_scale
        let color_palette = if let Some(scale) = result_scale {
            generate_color_palette(scale)
        } else {
            vec![Rgb([204, 204, 0])] // Дефолтный темно-желтый
        };
		
		// ТОЛЬКО CPU - никакого GPU, никаких сложностей
		let use_gpu = false;
		
		let mut img = ImageBuffer::from_fn(dimensions.img_width, dimensions.img_height, |_, _| Rgb([255u8, 255u8, 255u8]));
		
		// Логика масштабирования и рендеринга аналогична draw_image_as1_optimized
		let margin_pixels = MARGIN_MM * MM_TO_PIXELS;
		let right_margin_pixels = margin_pixels * 1.5;
		let bottom_margin_pixels = margin_pixels * 1.5;
		let available_width_pixels = dimensions.img_width as f64 - margin_pixels - right_margin_pixels;
		let available_height_pixels = dimensions.img_height as f64 - margin_pixels - bottom_margin_pixels;
		
		let scale_x = available_width_pixels / dimensions.content_width;
		let scale_y = available_height_pixels / dimensions.content_height;
		let safety_margin = 0.968;
		let coord_scale = scale_x.min(scale_y) * safety_margin;
		
		let scaled_content_width = dimensions.content_width * coord_scale;
		let scaled_content_height = dimensions.content_height * coord_scale;
		
		let offset_x = margin_pixels;
		let offset_y = margin_pixels + (available_height_pixels - scaled_content_height) / 2.0;
		
		let font_size = 25.0;
		let text_color = Rgb([0u8, 0u8, 0u8]);
		let font_scale = Scale::uniform(font_size);
		
		// === STEP 10: RENDERING FIGURES ===
		let mut rendered_count = 0;
		
		// 🔍 PERFORMANCE: Начало рендеринга полигонов
		let polygon_start = web_sys::window().unwrap().performance().unwrap().now();
		let mut rendered_count = 0;
		
		// Рендерим все объекты с цветовой палитрой
		for (item_idx, item) in self.data.iter().enumerate() {
			if item.vertices.len() == 4 {
				let points: Vec<Point<f64>> = item.vertices.iter().map(|v| {
					let normalized_x = v.x - dimensions.min_x;
					let normalized_y = v.y - dimensions.min_y;
					Point::new(normalized_x * coord_scale + offset_x, normalized_y * coord_scale + offset_y)
				}).collect();
				
				let quad_points: Vec<Point<i32>> = points.iter().map(|p| {
					Point::new(p.x as i32, p.y as i32)
				}).collect();
				
				// Выбираем цвет из палитры или дефолтный
				let fill_color = if color_palette.len() > 1 {
					get_color_for_value(item, field, result_scale, &color_palette)
				} else {
					color_palette[0]
				};
				
				draw_polygon_mut(&mut img, &quad_points, fill_color);
				rendered_count += 1;
				
				// Контуры
				for i in 0..4 {
					let next = (i + 1) % 4;
					draw_line_segment_mut(&mut img,
						(points[i].x as f32, points[i].y as f32),
						(points[next].x as f32, points[next].y as f32),
						Rgb([0, 0, 0]));
				}
				
				// 🔍 PERFORMANCE: Текст рендеринг (потенциально медленная операция)
				if let Some(values) = item.get_value(field) {
					if let Some(max_value) = values.iter().cloned().max_by(|a, b| a.partial_cmp(b).unwrap()) {
						let min_x = points.iter().map(|p| p.x).fold(f64::INFINITY, f64::min);
						let max_x = points.iter().map(|p| p.x).fold(f64::NEG_INFINITY, f64::max);
						let min_y = points.iter().map(|p| p.y).fold(f64::INFINITY, f64::min);
						let max_y = points.iter().map(|p| p.y).fold(f64::NEG_INFINITY, f64::max);
						
						let width = max_x - min_x;
						let height = max_y - min_y;
						let text_x = (min_x + width * 0.1) as i32;
						let text_y = (min_y + height * 0.1) as i32;
						
						draw_text_mut(&mut img, text_color, text_x, text_y, font_scale, &CACHED_FONT, &max_value.to_string());
					}
				}
			}
		}
		
		// === STEP 11: СОЗДАНИЕ ЛЕГЕНДЫ ===
         let legend_width = dimensions.img_width;
         let legend_height = 200; // Уменьшена высота для меньших шрифтов
        
        let legend_img = Self::create_legend_image(
             floor_level,
             field,
             result_scale,
             &color_palette,
             legend_width,
             legend_height
         );
        
        // === STEP 12: КОМБИНИРОВАНИЕ ИЗОБРАЖЕНИЙ ===
        let total_height = legend_height + dimensions.img_height;
        let mut combined_img = ImageBuffer::from_fn(dimensions.img_width, total_height, |_, _| Rgb([255u8, 255u8, 255u8]));
        
        // Копируем легенду сверху
        for y in 0..legend_height {
            for x in 0..dimensions.img_width {
                if x < legend_width && y < legend_height {
                    let pixel = legend_img.get_pixel(x, y);
                    combined_img.put_pixel(x, y, *pixel);
                }
            }
        }
        
        // Копируем основное изображение снизу
        for y in 0..dimensions.img_height {
            for x in 0..dimensions.img_width {
                let pixel = img.get_pixel(x, y);
                combined_img.put_pixel(x, y + legend_height, *pixel);
            }
        }
        
        // 🔍 PERFORMANCE: Измеряем время полигонов
        let polygon_time = web_sys::window().unwrap().performance().unwrap().now() - polygon_start;
        
        // 🔍 PERFORMANCE: Начало PNG кодирования
        let png_start = web_sys::window().unwrap().performance().unwrap().now();
        
        // PNG кодирование
        let mut buffer = Vec::new();
        let cursor = Cursor::new(&mut buffer);
        let encoder = PngEncoder::new(cursor);
        combined_img.write_with_encoder(encoder).unwrap();
        
        // 🔍 PERFORMANCE: Финальные метрики
        let png_time = web_sys::window().unwrap().performance().unwrap().now() - png_start;
        let total_time = web_sys::window().unwrap().performance().unwrap().now() - start_time;
        
        web_sys::console::log_1(&format!(
            "📊 PERFORMANCE [{}]: Total={:.1}ms, Polygons={:.1}ms ({} objects), PNG={:.1}ms ({:.1}MB)", 
            field, total_time, polygon_time, rendered_count, png_time, buffer.len() as f64 / 1024.0 / 1024.0
        ).into());
        
        buffer
    }

    /// Создает легенду с заголовком, цветовой шкалой и метаданными
    fn create_legend_image(
        floor_level: f32,
        function_name: &str,
        result_scale: Option<&str>,
        color_palette: &[Rgb<u8>],
        legend_width: u32,
        legend_height: u32
    ) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let mut legend_img = ImageBuffer::from_fn(legend_width, legend_height, |_, _| Rgb([255u8, 255u8, 255u8]));
        
        let mut current_y = 10;
        
        // === РАЗДЕЛ 1: TITLE ===
         let title_text = format!("Этаж {} - Функция {}", floor_level, function_name.to_uppercase());
         let title_font_size = 40.0; // Уменьшено в 2 раза (80 / 2)
         let title_scale = Scale::uniform(title_font_size);
         let title_color = Rgb([0u8, 0u8, 0u8]);
         
         // Центрируем заголовок
          let title_x = (legend_width as i32 - (title_text.len() as i32 * 24)) / 2; // Пропорционально уменьшено
          draw_text_mut(&mut legend_img, title_color, title_x, current_y, title_scale, &CACHED_FONT, &title_text);
          current_y += 50; // Уменьшено пропорционально
        
        // === РАЗДЕЛ 2: ЦВЕТОВАЯ ШКАЛА ===
        if let Some(scale) = result_scale {
            let scale_ranges = Self::parse_result_scale_ranges(scale);
            let rect_count = scale_ranges.len();
            
            if rect_count > 0 {
                let total_scale_width = legend_width - 40; // Отступы по 20px с каждой стороны
                let rect_width = total_scale_width / rect_count as u32;
                let rect_height = 20;
                
                // Рисуем прямоугольники с цветами
                for (i, _range) in scale_ranges.iter().enumerate() {
                    let x = 20 + (i as u32 * rect_width);
                    let y = current_y as u32;
                    
                    let color = color_palette.get(i).copied().unwrap_or(color_palette[i]);
                    let rect = Rect::at(x as i32, y as i32).of_size(rect_width - 2, rect_height); // -2 для отступа
                    draw_filled_rect_mut(&mut legend_img, rect, color);
                }
                
                current_y += rect_height as i32 + 10; // Уменьшен отступ
                
                // Подписи диапазонов
                 let range_font_size = 30.0; // Уменьшено в 4 раза (120 / 4)
                 let range_scale = Scale::uniform(range_font_size);
                
                for (i, range) in scale_ranges.iter().enumerate() {
                    let x = 20 + (i as u32 * rect_width);
                    let range_text = if i == 0 {
                        format!("0 - {:.3}", range.1)
                    } else {
                        format!("{:.3} - {:.3}", scale_ranges[i-1].1, range.1)
                    };
                    
                    draw_text_mut(&mut legend_img, title_color, x as i32, current_y, range_scale, &CACHED_FONT, &range_text);
                }
                
                current_y += 40; // Уменьшен отступ после диапазонов
            }
        }
        
        // === РАЗДЕЛ 3: МЕТАДАННЫЕ ===
         let metadata_font_size = 25.0; // Уменьшено в 4 раза (100 / 4)
         let metadata_scale = Scale::uniform(metadata_font_size);
        
        let calculation_method = Self::get_calculation_method();
        draw_text_mut(&mut legend_img, title_color, 20, current_y, metadata_scale, &CACHED_FONT, &calculation_method);
          current_y += 30; // Уменьшено в 4 раза
          
          let units_text = "Единицы измерения см2";
          draw_text_mut(&mut legend_img, title_color, 20, current_y, metadata_scale, &CACHED_FONT, units_text);
          current_y += 30; // Уменьшено в 4 раза
          
          let diameter_text = "Шаг диаметр - мм";
          draw_text_mut(&mut legend_img, title_color, 20, current_y, metadata_scale, &CACHED_FONT, diameter_text);
        
        legend_img
    }
    
    /// Парсит result_scale и возвращает диапазоны (min, max)
     fn parse_result_scale_ranges(scale: &str) -> Vec<(f32, f32)> {
         parse_result_scale_ranges(scale)
     }
    
    /// Возвращает метод расчета (выносим в отдельную функцию для будущих изменений)
    fn get_calculation_method() -> String {
        "Расчет по усилиям СНиП 2.03.01-84".to_string()
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
		
		// Простая последовательная генерация с новой функцией
		let mut results = Vec::with_capacity(4);
		for field in fields.iter() {
			let result = self.draw_image(field, None).await;
			results.push(result);
		}
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
}


