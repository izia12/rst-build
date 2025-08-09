// OPTIMIZATION: Оптимизированный compute shader для рендеринга линий
// Использует единый буфер и параллельную обработку

// Структура для метаданных изображения
struct ImageMetadata {
    width: f32,
    height: f32,
    pixel_offset: f32,
    line_offset: f32,
    line_count: f32,
    color_r: f32,
    color_g: f32,
    color_b: f32,
}

// Структура для данных линии
struct LineData {
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

// Буферы
@group(0) @binding(0) var<storage, read_write> image_buffer: array<u32>;
@group(0) @binding(1) var<storage, read> line_buffer: array<f32>;
@group(0) @binding(2) var<storage, read> metadata_buffer: array<f32>;

// OPTIMIZATION: Оптимизированная функция для рисования линии с использованием алгоритма Брезенхема
fn draw_line_optimized(x0: i32, y0: i32, x1: i32, y1: i32, color: u32, width: u32, height: u32, pixel_offset: u32) {
    var dx = abs(x1 - x0);
    var dy = abs(y1 - y0);
    var sx = select(-1, 1, x0 < x1);
    var sy = select(-1, 1, y0 < y1);
    var err = dx - dy;
    
    var x = x0;
    var y = y0;
    
    // OPTIMIZATION: Ограничиваем количество итераций для предотвращения зависания
    var max_iterations = dx + dy + 1;
    var iteration = 0;
    
    loop {
        if iteration >= max_iterations {
            break;
        }
        
        // Проверяем границы изображения
        if x >= 0 && x < i32(width) && y >= 0 && y < i32(height) {
            let pixel_index = pixel_offset + u32(y) * width + u32(x);
            if pixel_index < arrayLength(&image_buffer) {
                image_buffer[pixel_index] = color;
            }
        }
        
        if x == x1 && y == y1 {
            break;
        }
        
        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x += sx;
        }
        if e2 < dx {
            err += dx;
            y += sy;
        }
        
        iteration += 1;
    }
}

// OPTIMIZATION: Функция для упаковки цвета в u32 (ABGR формат)
fn pack_color(r: f32, g: f32, b: f32, a: f32) -> u32 {
    let r_u8 = u32(clamp(r * 255.0, 0.0, 255.0));
    let g_u8 = u32(clamp(g * 255.0, 0.0, 255.0));
    let b_u8 = u32(clamp(b * 255.0, 0.0, 255.0));
    let a_u8 = u32(clamp(a * 255.0, 0.0, 255.0));
    
    return (a_u8 << 24u) | (b_u8 << 16u) | (g_u8 << 8u) | r_u8;
}

// OPTIMIZATION: Функция для получения метаданных изображения по индексу линии
fn get_image_metadata_for_line(line_index: u32) -> ImageMetadata {
    let num_images = arrayLength(&metadata_buffer) / 8u;
    
    for (var img_idx = 0u; img_idx < num_images; img_idx += 1u) {
        let base_idx = img_idx * 8u;
        let line_offset = u32(metadata_buffer[base_idx + 3u]);
        let line_count = u32(metadata_buffer[base_idx + 4u]);
        
        if line_index >= line_offset && line_index < line_offset + line_count {
            var metadata: ImageMetadata;
            metadata.width = metadata_buffer[base_idx];
            metadata.height = metadata_buffer[base_idx + 1u];
            metadata.pixel_offset = metadata_buffer[base_idx + 2u];
            metadata.line_offset = metadata_buffer[base_idx + 3u];
            metadata.line_count = metadata_buffer[base_idx + 4u];
            metadata.color_r = metadata_buffer[base_idx + 5u];
            metadata.color_g = metadata_buffer[base_idx + 6u];
            metadata.color_b = metadata_buffer[base_idx + 7u];
            return metadata;
        }
    }
    
    // Возвращаем пустые метаданные если не найдено
    var empty_metadata: ImageMetadata;
    empty_metadata.width = 0.0;
    empty_metadata.height = 0.0;
    empty_metadata.pixel_offset = 0.0;
    empty_metadata.line_offset = 0.0;
    empty_metadata.line_count = 0.0;
    empty_metadata.color_r = 0.0;
    empty_metadata.color_g = 0.0;
    empty_metadata.color_b = 0.0;
    return empty_metadata;
}

// OPTIMIZATION: Главная функция compute shader
@compute @workgroup_size(256, 1, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let line_index = global_id.x;
    let total_lines = arrayLength(&line_buffer) / 8u;
    
    // Проверяем границы
    if line_index >= total_lines {
        return;
    }
    
    // OPTIMIZATION: Получаем метаданные изображения для этой линии
    let metadata = get_image_metadata_for_line(line_index);
    
    if metadata.width <= 0.0 || metadata.height <= 0.0 {
        return;
    }
    
    // OPTIMIZATION: Извлекаем данные линии
    let line_base = line_index * 8u;
    let x1 = metadata_buffer[line_base];
    let y1 = metadata_buffer[line_base + 1u];
    let x2 = metadata_buffer[line_base + 2u];
    let y2 = metadata_buffer[line_base + 3u];
    let line_r = metadata_buffer[line_base + 4u];
    let line_g = metadata_buffer[line_base + 5u];
    let line_b = metadata_buffer[line_base + 6u];
    let line_a = metadata_buffer[line_base + 7u];
    
    // OPTIMIZATION: Используем цвет линии, если он задан, иначе цвет изображения
    let final_r = select(metadata.color_r, line_r, line_a > 0.0);
    let final_g = select(metadata.color_g, line_g, line_a > 0.0);
    let final_b = select(metadata.color_b, line_b, line_a > 0.0);
    let final_a = select(1.0, line_a, line_a > 0.0);
    
    let color = pack_color(final_r, final_g, final_b, final_a);
    
    // OPTIMIZATION: Рисуем линию
    draw_line_optimized(
        i32(x1),
        i32(y1),
        i32(x2),
        i32(y2),
        color,
        u32(metadata.width),
        u32(metadata.height),
        u32(metadata.pixel_offset)
    );
}