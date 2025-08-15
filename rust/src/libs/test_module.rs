use crate::libs::drawItem::DrawItemZ;
use crate::libs::parse::EntityWithXlsx;
use crate::libs::generate_documents::PerformanceConfig;
use image::{ImageBuffer, Rgb};
use crate::Vertex;
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone)]
pub struct TestVertex {
    pub x: f64,
    pub y: f64,
}

#[wasm_bindgen]
pub struct TestModule {
    draw_item: DrawItemZ,
}

#[wasm_bindgen]
impl TestModule {
    #[wasm_bindgen(constructor)]
    pub fn new() -> TestModule {
        TestModule {
            draw_item: DrawItemZ::new(),
        }
    }

    /// Создает тестовые данные - массив четырехугольников
    pub fn create_test_data(&mut self) {
        // Очищаем существующие данные
        self.draw_item = DrawItemZ::new();
        
        // Тестовые четырехугольники с разными размерами и позициями
        let test_rectangles = vec![
            // Прямоугольник 1 (левый верхний)
            vec![
                TestVertex { x: 2.0, y: 2.0 },
                TestVertex { x: 6.0, y: 2.0 },
                TestVertex { x: 6.0, y: 4.0 },
                TestVertex { x: 2.0, y: 4.0 },
            ],
            // Прямоугольник 2 (правый верхний)
            vec![
                TestVertex { x: 8.0, y: 2.0 },
                TestVertex { x: 12.0, y: 2.0 },
                TestVertex { x: 12.0, y: 5.0 },
                TestVertex { x: 8.0, y: 5.0 },
            ],
            // Прямоугольник 3 (левый нижний)
            vec![
                TestVertex { x: 1.0, y: 7.0 },
                TestVertex { x: 5.0, y: 7.0 },
                TestVertex { x: 5.0, y: 10.0 },
                TestVertex { x: 1.0, y: 10.0 },
            ],
            // Прямоугольник 4 (правый нижний)
            vec![
                TestVertex { x: 9.0, y: 8.0 },
                TestVertex { x: 13.0, y: 8.0 },
                TestVertex { x: 13.0, y: 11.0 },
                TestVertex { x: 9.0, y: 11.0 },
            ],
            // Прямоугольник 5 (центральный)
            vec![
                TestVertex { x: 6.0, y: 6.0 },
                TestVertex { x: 8.0, y: 6.0 },
                TestVertex { x: 8.0, y: 8.0 },
                TestVertex { x: 6.0, y: 8.0 },
            ],
        ];

        // Преобразуем тестовые данные в формат EntityWithXlsx
        for (i, rectangle) in test_rectangles.iter().enumerate() {
            let vertices: Vec<Vertex> = rectangle
                 .iter()
                 .map(|v| Vertex { x: v.x, y: v.y, z: 0.0 })
                 .collect();

            let entity = EntityWithXlsx {
                 entity_type: "rectangle".to_string(),
                 vertices,
                 row: None,
                 changed: false,
                 material: None,
             };

            self.draw_item.add_entity(entity);
        }
    }

    /// Генерирует изображение с тестовыми данными
    pub fn generate_test_image(&self) -> Vec<u8> {
        let config = PerformanceConfig {
            enable_parallel_rendering: false,
            enable_gpu_acceleration: false,
            max_image_size: (800, 600),
            compression_quality: 80,
            enable_caching: false,
        };

        // Используем синхронную функцию для генерации изображения
        self.draw_simple_test_image()
    }

    /// Получает информацию о границах тестовых данных
    pub fn get_test_bounds_info(&self) -> String {
        let config = PerformanceConfig {
            enable_parallel_rendering: false,
            enable_gpu_acceleration: false,
            max_image_size: (800, 600),
            compression_quality: 80,
            enable_caching: false,
        };

        // Используем внутренний метод для получения границ
        let (min_x, min_y, max_x, max_y, width, height) = 
            self.draw_item.calculate_image_bounds_with_config(&config);

        format!(
            "Границы тестовых данных:\n\
            X: от {:.2} до {:.2} (ширина: {:.2})\n\
            Y: от {:.2} до {:.2} (высота: {:.2})\n\
            Размер изображения: {}x{}",
            min_x, max_x, max_x - min_x,
            min_y, max_y, max_y - min_y,
            width, height
        )
    }

    /// Получает количество тестовых объектов
    pub fn get_test_objects_count(&self) -> usize {
        self.draw_item.data.len()
    }

    /// Генерирует указанное количество тестовых сущностей
    pub fn generate_test_entities(&mut self, count: usize) {
        for i in 0..count {
            let rectangle_x = (i as f64 * 2.0) % 10.0;
            let rectangle_y = (i as f64 * 1.5) % 8.0;
            self.add_custom_rectangle(rectangle_x, rectangle_y, 1.5, 1.0);
        }
    }

    /// Добавляет прямоугольник с заданными координатами и размерами
    pub fn add_custom_rectangle(&mut self, start_x: f64, start_y: f64, width: f64, height: f64) {
        let vertices = vec![
            Vertex { x: start_x, y: start_y, z: 0.0 },
            Vertex { x: start_x + width, y: start_y, z: 0.0 },
            Vertex { x: start_x + width, y: start_y + height, z: 0.0 },
            Vertex { x: start_x, y: start_y + height, z: 0.0 },
        ];

        let entity = EntityWithXlsx {
            entity_type: "rectangle".to_string(),
            vertices,
            row: None,
            changed: false,
            material: None,
        };

        self.draw_item.data.push(entity);
    }
}

// Вспомогательные функции для работы с тестовыми данными
impl TestModule {
    /// Добавляет дополнительный прямоугольник к тестовым данным
    pub fn add_rectangle(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64, x4: f64, y4: f64) {
        let vertices = vec![
            Vertex { x: x1, y: y1, z: 0.0 },
            Vertex { x: x2, y: y2, z: 0.0 },
            Vertex { x: x3, y: y3, z: 0.0 },
            Vertex { x: x4, y: y4, z: 0.0 },
        ];

        let entity = EntityWithXlsx {
             entity_type: "rectangle".to_string(),
             vertices,
             row: None,
             changed: false,
             material: None,
         };

        self.draw_item.add_entity(entity);
    }

    /// Очищает все тестовые данные
    pub fn clear_data(&mut self) {
        self.draw_item = DrawItemZ::new();
    }

    // Простая синхронная функция для рисования тестового изображения и создания DOCX
    pub fn draw_simple_test_image(&self) -> Vec<u8> {
        use docx_rs::*;
        let config = PerformanceConfig {
            enable_parallel_rendering: false,
            enable_gpu_acceleration: false,
            max_image_size: (1600, 1200), // Увеличиваем размер изображения
            compression_quality: 80,
            enable_caching: false,
        };
        use image::{ImageBuffer, Rgba, ImageEncoder};
        
        // Создаем большое изображение с фиксированным размером
        let width = 1200u32;
        let height = 1400u32;
        let mut img = ImageBuffer::new(width, height);
        
        // Заполняем белым фоном
        for pixel in img.pixels_mut() {
            *pixel = Rgba([255, 255, 255, 255]);
        }
        
        // Рисуем большие тестовые фигуры по всему изображению
        let colors = [
            Rgba([255, 0, 0, 255]),   // Красный
            Rgba([0, 255, 0, 255]),   // Зеленый
            Rgba([0, 0, 255, 255]),   // Синий
            Rgba([255, 255, 0, 255]), // Желтый
            Rgba([255, 0, 255, 255]), // Пурпурный
            Rgba([0, 255, 255, 255]), // Голубой
        ];

        // Создаем много больших прямоугольников
        for i in 0..12 {
            let row = i / 4;
            let col = i % 4;
            
            let rect_width = 300.0;
            let rect_height = 200.0;
            let margin = 50.0;
            
            let start_figure_x = margin + col as f32 * (rect_width + margin) +200.0;
            let start_figure_y = margin + row as f32 * (rect_height + margin)+200.0;
            
            let color = colors[i % colors.len()];
            
            // Рисуем толстые линии прямоугольника
            for thickness in 0..5 {
                let offset = thickness as f32;
                imageproc::drawing::draw_line_segment_mut(&mut img, 
                    (start_figure_x + offset, start_figure_y + offset), 
                    (start_figure_x + rect_width - offset, start_figure_y + offset), color);
                imageproc::drawing::draw_line_segment_mut(&mut img, 
                    (start_figure_x + rect_width - offset, start_figure_y + offset), 
                    (start_figure_x + rect_width - offset, start_figure_y + rect_height - offset), color);
                imageproc::drawing::draw_line_segment_mut(&mut img, 
                    (start_figure_x + rect_width - offset, start_figure_y + rect_height - offset), 
                    (start_figure_x + offset, start_figure_y + rect_height - offset), color);
                imageproc::drawing::draw_line_segment_mut(&mut img, 
                    (start_figure_x + offset, start_figure_y + rect_height - offset), 
                    (start_figure_x + offset, start_figure_y + offset), color);
            }
            
            // Добавляем заливку
            for fill_y in (start_figure_y as u32 + 10)..(start_figure_y as u32 + rect_height as u32 - 10) {
                for fill_x in (start_figure_x as u32 + 10)..(start_figure_x as u32 + rect_width as u32 - 10) {
                    if fill_x < width && fill_y < height {
                        let mut fill_color = color;
                        fill_color.0[3] = 100; // Полупрозрачная заливка
                        img.put_pixel(fill_x, fill_y, fill_color);
                    }
                }
            }
        }
        
        // Конвертируем в PNG
        let mut png_data = Vec::new();
        {
            let encoder = image::codecs::png::PngEncoder::new(&mut png_data);
            if let Err(_) = encoder.write_image(
                img.as_raw(),
                width,
                height,
                image::ColorType::Rgba8,
            ) {
                return Vec::new();
            }
        }
        
        // Создаем DOCX документ с изображением
        // Рассчитываем размеры для DOCX в твипах (1 твип = 1/20 точки)
        // Для изображения 1200x1400 пикселей при 96 DPI:
        // 1200 пикселей = 1200/96*72*20 = 18000 твипов
        // 1400 пикселей = 1400/96*72*20 = 21000 твипов
        let docx_width_twips = (width as f64 * 72.0 / 96.0 * 20.0) as u32;  // ~18000 твипов
        let docx_height_twips = (height as f64 * 72.0 / 96.0 * 20.0) as u32; // ~21000 твипов
        
        let doc = Docx::new()
            .add_paragraph(
                Paragraph::new()
                    .add_run(Run::new().add_text("Тестовое изображение с геометрическими фигурами"))
            )
            .add_paragraph(
                Paragraph::new()
                    .add_run(
                        Run::new().add_image(
                            Pic::new(&png_data)
                                .size(docx_width_twips, docx_height_twips)
                        )
                    )
            );
        
        // Конвертируем DOCX в байты
        let mut buffer = Vec::new();
        let mut cursor = std::io::Cursor::new(&mut buffer);
        match doc.build().pack(&mut cursor) {
            Ok(_) => buffer,
            Err(_) => Vec::new(),
        }
    }
}