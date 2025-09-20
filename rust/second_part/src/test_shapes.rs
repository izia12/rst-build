use image::{ImageBuffer, Rgb, RgbImage};
use imageproc::drawing::{draw_line_segment_mut, draw_polygon_mut};
use imageproc::point::Point;
use rand::Rng;
use std::fs;
use std::path::Path;
use docx_rs::*;
use std::io::Cursor;

/// Генерирует тестовое изображение с рандомными фигурами
pub fn generate_test_image() -> Vec<u8> {
    let width = 800;
    let height = 600;
    let mut img = ImageBuffer::from_fn(width, height, |_, _| Rgb([255u8, 255u8, 255u8]));
    
    let mut rng = rand::thread_rng();
    
    // Генерируем 30 рандомных фигур
    for i in 0..30 {
        let is_triangle = i % 3 == 0; // Каждая третья фигура - треугольник
        
        if is_triangle {
            // Генерируем треугольник
            let center_x = rng.gen_range(100..width-100) as i32;
            let center_y = rng.gen_range(100..height-100) as i32;
            let size = rng.gen_range(80..150); // Увеличиваем размер треугольников
            
            let points = vec![
                Point::new(center_x, center_y - size),
                Point::new(center_x - size, center_y + size/2),
                Point::new(center_x + size, center_y + size/2),
            ];
            
            // УБИРАЕМ заливку треугольника - только контуры!
            // draw_polygon_mut(&mut img, &points, Rgb([255, 255, 255]));
            
            // Убрано: отладочный вывод координат
             
             // Возвращаем обычную заливку + контуры как в рабочем проекте
             draw_polygon_mut(&mut img, &points, Rgb([255, 255, 255]));
             
             // Рисуем черные контуры как в рабочем проекте
             for j in 0..3 {
                 let start = points[j];
                 let end = points[(j + 1) % 3];
                 draw_line_segment_mut(
                     &mut img,
                     (start.x as f32, start.y as f32),
                     (end.x as f32, end.y as f32),
                     Rgb([0, 0, 0]),
                 );
             }
        } else {
            // Генерируем четырехугольник
            let center_x = rng.gen_range(50..width-50) as i32;
            let center_y = rng.gen_range(50..height-50) as i32;
            let width_rect = rng.gen_range(30..80);
            let height_rect = rng.gen_range(20..60);
            
            let points = vec![
                Point::new(center_x - width_rect/2, center_y - height_rect/2),
                Point::new(center_x + width_rect/2, center_y - height_rect/2),
                Point::new(center_x + width_rect/2, center_y + height_rect/2),
                Point::new(center_x - width_rect/2, center_y + height_rect/2),
            ];
            
            // Белая заливка
            draw_polygon_mut(&mut img, &points, Rgb([255, 255, 255]));
            
            // Черные контуры
            for j in 0..4 {
                let start = points[j];
                let end = points[(j + 1) % 4];
                draw_line_segment_mut(
                    &mut img,
                    (start.x as f32, start.y as f32),
                    (end.x as f32, end.y as f32),
                    Rgb([0, 0, 0]),
                );
            }
        }
    }
    
    // Конвертируем в PNG байты
    let mut buffer = Vec::new();
    let cursor = Cursor::new(&mut buffer);
    let encoder = image::codecs::png::PngEncoder::new(cursor);
    img.write_with_encoder(encoder).unwrap();
    
    buffer
}

/// Создает DOCX файл с тестовым изображением
pub fn create_test_docx(image_data: &[u8]) -> Vec<u8> {
    let doc = Docx::new()
        .add_paragraph(
            Paragraph::new()
                .align(AlignmentType::Center)
                .add_run(
                    Run::new()
                        .add_text("Тест рисования фигур")
                        .size(24)
                        .bold()
                ),
        )
        .add_paragraph(Paragraph::new())
        .add_paragraph(
            Paragraph::new()
                .add_run(
                    Run::new()
                        .add_text("30 рандомных фигур с четкими контурами:")
                        .size(16)
                ),
        )
        .add_paragraph(Paragraph::new())
        .add_paragraph(
            Paragraph::new()
                .align(AlignmentType::Center)
                .add_run(
                    Run::new().add_image(
                        Pic::new(image_data)
                            .size(6000000, 4500000) // 600x450 EMU
                    )
                )
        )
        .add_paragraph(Paragraph::new())
        .add_paragraph(
            Paragraph::new()
                .add_run(
                    Run::new()
                        .add_text("Изображение содержит:")
                        .size(14)
                        .bold()
                ),
        )
        .add_paragraph(
            Paragraph::new()
                .add_run(
                    Run::new()
                        .add_text("• 20 четырехугольников")
                        .size(12)
                ),
        )
        .add_paragraph(
            Paragraph::new()
                .add_run(
                    Run::new()
                        .add_text("• 10 треугольников")
                        .size(12)
                ),
        )
        .add_paragraph(
            Paragraph::new()
                .add_run(
                    Run::new()
                        .add_text("• Белая заливка с черными контурами")
                        .size(12)
                ),
        );
    
    let mut buffer = Cursor::new(Vec::new());
    doc.build().pack(&mut buffer).unwrap();
    buffer.into_inner()
}

/// Основная функция для тестирования
pub fn run_test() -> Result<(), Box<dyn std::error::Error>> {
    // Генерируем изображение
    let image_data = generate_test_image();
    
    // Сохраняем PNG
    fs::write("test_shapes.png", &image_data)?;
    
    // Создаем DOCX
    let docx_data = create_test_docx(&image_data);
    fs::write("test_shapes.docx", &docx_data)?;
    
    Ok(())
}