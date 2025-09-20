use crate::types::*;
use anyhow::{Result, Context};
use glam::Vec3;
use image::{ImageBuffer, Rgb, RgbImage};
use imageproc::drawing::{draw_line_segment_mut, draw_polygon_mut, draw_filled_rect_mut, draw_text_mut};
use imageproc::point::Point;
use log::{debug, info};
use palette::{Hsv, IntoColor, Srgb};
use std::collections::HashMap;

/// Генератор изображений для визуализации элементов
pub struct Visualizer {
    /// Настройки рендеринга
    settings: RenderSettings,
    /// Кэш цветов для групп
    group_colors: HashMap<usize, Rgb<u8>>,
}

/// Настройки рендеринга
#[derive(Debug, Clone)]
pub struct RenderSettings {
    /// Размер изображения
    pub image_width: u32,
    pub image_height: u32,
    /// Цвет фона
    pub background_color: Rgb<u8>,
    /// Толщина линий для пластинчатых элементов
    pub shell_line_width: f32,
    /// Толщина линий для стержневых элементов
    pub beam_line_width: f32,
    /// Цвета по умолчанию
    pub default_shell_color: Rgb<u8>,
    pub default_beam_color: Rgb<u8>,
    pub default_column_color: Rgb<u8>,
    /// Отступы
    pub margin: f32,
    /// Масштабирование
    pub auto_scale: bool,
}

impl Default for RenderSettings {
    fn default() -> Self {
        Self {
            image_width: 2400,  // Увеличиваем в 2 раза для лучшего качества
            image_height: 1800, // Увеличиваем в 2 раза для лучшего качества
            background_color: Rgb([255, 255, 255]), // Белый фон
            shell_line_width: 1.0,
            beam_line_width: 3.0,
            default_shell_color: Rgb([128, 128, 128]), // Серый
            default_beam_color: Rgb([255, 0, 0]),      // Красный
            default_column_color: Rgb([0, 0, 255]),    // Синий
            margin: 50.0,
            auto_scale: true,
        }
    }
}

/// Результат проекции 3D координат в 2D
#[derive(Debug, Clone)]
struct Projection {
    /// 2D координаты
    points_2d: Vec<Point<i32>>,
    /// Границы области
    min_x: f64,
    max_x: f64,
    min_y: f64,
    max_y: f64,
    /// Параметры масштабирования как в рабочем проекте
    scale: f64,
    offset_x: f64,
    offset_y: f64,
}

impl Visualizer {
    /// Создает новый визуализатор
    pub fn new() -> Self {
        Self {
            settings: RenderSettings::default(),
            group_colors: HashMap::new(),
        }
    }

    /// Создает визуализатор с настройками
    pub fn with_settings(settings: RenderSettings) -> Self {
        Self {
            settings,
            group_colors: HashMap::new(),
        }
    }

    /// Создает изображения по этажам на основе Z-координат
    pub fn create_floor_images(
        &self,
        elements: &[LiraElement],
        output_dir: &std::path::Path,
    ) -> Result<Vec<std::path::PathBuf>> {
        // Группируем элементы по Z-координатам (этажам)
        let floors = self.group_elements_by_floor(elements);
        
        let mut image_paths = Vec::new();
        
        for (floor_index, (z_level, floor_elements)) in floors.iter().enumerate() {
            let filename = format!("floor_{}_z{:.1}.png", floor_index + 1, z_level);
            let path = output_dir.join(&filename);
            
            let image_data = self.render_floor_image(floor_elements, *z_level, floor_index + 1)?;
            std::fs::write(&path, image_data)?;
            image_paths.push(path.clone());
            
            // Убрано: лог сохранения
        }
        
        // Генерация изображений завершена
        Ok(image_paths)
    }

    /// Группирует элементы по Z-координатам (этажам) как в рабочем проекте
    fn group_elements_by_floor<'a>(&self, elements: &'a [LiraElement]) -> Vec<(f32, Vec<&'a LiraElement>)> {
        // Сначала собираем все уникальные Z-координаты пластинчатых элементов
        let mut unique_z_coords = std::collections::HashSet::new();
        
        for element in elements {
            if !element.coordinates.is_empty() && matches!(element.element_type, ElementType::Shell) {
                let first_z = element.coordinates[0].z;
                let is_plate = element.coordinates.iter().all(|c| (c.z - first_z).abs() < 0.001);
                
                if is_plate {
                    // Округляем до 3 знаков после запятой для группировки близких значений
                    let rounded_z = (first_z * 1000.0).round() / 1000.0;
                    unique_z_coords.insert((rounded_z * 1000.0) as i32);
                }
            }
        }
        
        // Сортируем Z-координаты и группируем близкие (в пределах 3.3м)
        let mut sorted_z: Vec<f32> = unique_z_coords.iter()
            .map(|&z| z as f32 / 1000.0)
            .collect();
        sorted_z.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        // Группируем близкие Z-координаты в этажи (разница ~3.3м между этажами)
        let mut floor_groups = Vec::new();
        let floor_height = 3.3; // Примерная высота этажа
        
        for &z in &sorted_z {
            let mut found_group = false;
            for group in &mut floor_groups {
                let group_z: f32 = *group;
                if (z - group_z).abs() < floor_height {
                    found_group = true;
                    break;
                }
            }
            if !found_group {
                floor_groups.push(z);
            }
        }
        
        // Создаем карту этажей
        let mut floor_map: std::collections::HashMap<usize, Vec<&LiraElement>> = std::collections::HashMap::new();
        
        for element in elements {
            if !element.coordinates.is_empty() {
                let first_z = element.coordinates[0].z;
                let is_plate = element.coordinates.iter().all(|c| (c.z - first_z).abs() < 0.001);
                
                if is_plate && matches!(element.element_type, ElementType::Shell) {
                    // Находим ближайший этаж для пластинчатого элемента
                    let mut closest_floor = 0;
                    let mut min_distance = f32::INFINITY;
                    
                    for (i, &floor_z) in floor_groups.iter().enumerate() {
                        let distance = (first_z - floor_z).abs();
                        if distance < min_distance {
                            min_distance = distance;
                            closest_floor = i;
                        }
                    }
                    
                    if min_distance < floor_height {
                        floor_map.entry(closest_floor).or_insert_with(Vec::new).push(element);
                    }
                } else {
                    // Стержневые элементы добавляем ко всем пересекаемым этажам
                    let min_z = element.coordinates.iter().map(|c| c.z).fold(f32::INFINITY, f32::min);
                    let max_z = element.coordinates.iter().map(|c| c.z).fold(f32::NEG_INFINITY, f32::max);
                    
                    for (i, &floor_z) in floor_groups.iter().enumerate() {
                        if floor_z >= min_z - 1.0 && floor_z <= max_z + 1.0 {
                            floor_map.entry(i).or_insert_with(Vec::new).push(element);
                        }
                    }
                }
            }
        }
        
        // Создаем результат с реальными Z-координатами этажей
        let mut floors: Vec<_> = floor_map.into_iter()
            .filter(|(_, elements)| !elements.is_empty())
            .map(|(floor_idx, elements)| (floor_groups[floor_idx], elements))
            .collect();
        floors.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        
        floors
    }

    /// Рендерит изображение этажа
    fn render_floor_image(
        &self,
        elements: &[&LiraElement],
        z_level: f32,
        floor_number: usize,
    ) -> Result<Vec<u8>> {
        // Отладочный вывод для третьего этажа
        if (z_level - 6.6).abs() < 0.1 {
            let debug_info = format!("\n🏢 ОТЛАДКА ЭТАЖ 3 (z={:.1}): Начинаем рендеринг {} элементов\n", z_level, elements.len());
            std::fs::write("output/floor3_debug.txt", debug_info).ok();
            
            let shell_count = elements.iter().filter(|e| matches!(e.element_type, ElementType::Shell)).count();
            let beam_count = elements.iter().filter(|e| matches!(e.element_type, ElementType::Beam | ElementType::Column)).count();
            
            let mut debug_content = format!("   Пластинчатых элементов: {}\n", shell_count);
            debug_content.push_str(&format!("   Стержневых элементов: {}\n", beam_count));
            
            // Выводим ID всех элементов
            for element in elements {
                debug_content.push_str(&format!("   Элемент {}: тип {:?}, узлы {:?}\n", 
                         element.id, element.element_type, element.nodes));
            }
            
            std::fs::write("output/floor3_debug.txt", debug_content).ok();
        }
        
        let mut image = ImageBuffer::new(self.settings.image_width, self.settings.image_height);
        
        // Заливаем фон
        for pixel in image.pixels_mut() {
            *pixel = self.settings.background_color;
        }
        
        if elements.is_empty() {
            return self.image_to_png_bytes(image);
        }
        
        // Вычисляем проекцию только для элементов этажа
        let floor_elements_vec: Vec<_> = elements.iter().cloned().cloned().collect();
        let projection = self.calculate_projection(&floor_elements_vec)?;
        
        // Рендерим элементы с новой цветовой схемой
         for element in elements {
             self.render_element_with_adjacency(&mut image, element, &projection, elements)?;
         }
        
        // Добавляем заголовок с отладочной информацией для третьего этажа
        let title = if (z_level - 6.6).abs() < 0.1 {
            let shell_count = elements.iter().filter(|e| matches!(e.element_type, ElementType::Shell)).count();
            let beam_count = elements.iter().filter(|e| matches!(e.element_type, ElementType::Beam | ElementType::Column)).count();
            let adjacent_count = elements.iter()
                .filter(|e| matches!(e.element_type, ElementType::Shell))
                .filter(|e| self.is_adjacent_to_beam_by_nodes(e, elements))
                .count();
            format!("Этаж {} (Z = {:.1}м) - Пластин: {}, Стержней: {}, Прилегающих: {}", 
                    floor_number, z_level, shell_count, beam_count, adjacent_count)
        } else {
            format!("Этаж {} (Z = {:.1}м, {} элементов)", floor_number, z_level, elements.len())
        };
        self.add_title(&mut image, &title)?;
        
        self.image_to_png_bytes(image)
    }

    /// Генерирует изображения для всех групп
    pub fn generate_images(
        &mut self,
        elements: &[LiraElement],
        groups: &[ConnectedGroup],
    ) -> Result<Vec<Vec<u8>>> {
        // Генерация изображений
        
        let mut images = Vec::new();
        
        // Генерируем цвета для групп
        self.generate_group_colors(groups);
        
        // 1. Общее изображение со всеми элементами
        let overview_image = self.render_overview(elements, groups)
            .context("Ошибка генерации общего изображения")?;
        images.push(overview_image);
        
        // 2. Изображения для каждой группы отдельно
        for (i, group) in groups.iter().enumerate() {
            let group_elements: Vec<_> = elements
                .iter()
                .filter(|e| group.elements.contains(&e.id))
                .collect();
            
            let group_image = self.render_group(&group_elements, group, i)
                .with_context(|| format!("Ошибка генерации изображения группы {}", i))?;
            images.push(group_image);
        }
        
        // Генерация изображений завершена
        Ok(images)
    }

    /// Рендерит общий вид всех элементов
    fn render_overview(
        &self,
        elements: &[LiraElement],
        groups: &[ConnectedGroup],
    ) -> Result<Vec<u8>> {
        // Рендеринг общего вида
        
        let mut image = ImageBuffer::new(self.settings.image_width, self.settings.image_height);
        
        // Заливаем фон
        for pixel in image.pixels_mut() {
            *pixel = self.settings.background_color;
        }
        
        // Вычисляем проекцию
        let projection = self.calculate_projection(elements)?;
        
        // Рендерим элементы по группам с учетом прилегания к стержням
        for group in groups {
            let group_color = self.group_colors.get(&group.id)
                .copied()
                .unwrap_or(self.settings.default_shell_color);
            
            for &element_id in &group.elements {
                if let Some(element) = elements.iter().find(|e| e.id == element_id) {
                    // Для пластинчатых элементов используем раскраску по прилеганию к стержням
                    let color = if element.element_type == ElementType::Shell {
                        self.get_element_color_by_adjacency(element, &elements.iter().collect::<Vec<_>>())
                    } else {
                        // Для стержневых элементов используем цвет группы
                        group_color
                    };
                    
                    self.render_element(&mut image, element, &projection, Some(color))?;
                }
            }
        }
        
        // Добавляем заголовок
        self.add_title(&mut image, "Общий вид конструкции")?;
        
        // Добавляем легенду
        self.add_legend(&mut image, groups)?;
        
        self.image_to_png_bytes(image)
    }

    /// Рендерит отдельную группу
    fn render_group(
        &self,
        elements: &[&LiraElement],
        group: &ConnectedGroup,
        group_index: usize,
    ) -> Result<Vec<u8>> {
        // Убрано избыточное логирование для ускорения работы
        
        let mut image = ImageBuffer::new(self.settings.image_width, self.settings.image_height);
        
        // Заливаем фон
        for pixel in image.pixels_mut() {
            *pixel = self.settings.background_color;
        }
        
        // Вычисляем проекцию только для элементов группы
        let group_elements_vec: Vec<_> = elements.iter().cloned().cloned().collect();
        let projection = self.calculate_projection(&group_elements_vec)?;
        
        let group_color = self.group_colors.get(&group.id)
            .copied()
            .unwrap_or(self.settings.default_shell_color);
        
        // Рендерим элементы с учетом прилегания к стержням
        for element in elements {
            // Для пластинчатых элементов используем раскраску по прилеганию к стержням
            let color = if element.element_type == ElementType::Shell {
                self.get_element_color_by_adjacency(element, elements)
            } else {
                // Для стержневых элементов используем цвет группы
                group_color
            };
            
            self.render_element(&mut image, element, &projection, Some(color))?;
        }
        
        // Добавляем заголовок
        let title = format!("Группа {} ({} элементов)", group_index + 1, elements.len());
        self.add_title(&mut image, &title)?;
        
        // Добавляем статистику
        self.add_group_statistics(&mut image, group)?;
        
        self.image_to_png_bytes(image)
    }

    /// Рендерит отдельный элемент
    fn render_element(
        &self,
        image: &mut RgbImage,
        element: &LiraElement,
        projection: &Projection,
        color_override: Option<Rgb<u8>>,
    ) -> Result<()> {
        if element.coordinates.len() < 2 {
            return Ok(()); // Недостаточно точек для рендеринга
        }
        
        // Проецируем координаты элемента
        let element_points_2d: Vec<Point<i32>> = element.coordinates
            .iter()
            .map(|&coord| self.project_point(coord, projection))
            .collect();
        
        // Выбираем цвет и толщину линии
        let (color, line_width) = if let Some(color) = color_override {
            (color, if element.is_beam() { self.settings.beam_line_width } else { self.settings.shell_line_width })
        } else {
            match element.element_type {
                ElementType::Shell => (self.settings.default_shell_color, self.settings.shell_line_width),
                ElementType::Beam => (self.settings.default_beam_color, self.settings.beam_line_width),
                ElementType::Column => (self.settings.default_column_color, self.settings.beam_line_width),
                ElementType::Unknown => (Rgb([64, 64, 64]), self.settings.shell_line_width),
            }
        };
        
        // Рендерим в зависимости от типа элемента
        match element.element_type {
            ElementType::Shell => {
                // Для пластинчатых элементов рисуем контур
                if element_points_2d.len() >= 3 {
                    // Рисуем полигон
                    for i in 0..element_points_2d.len() {
                        let start = element_points_2d[i];
                        let end = element_points_2d[(i + 1) % element_points_2d.len()];
                        
                        // Рисуем несколько линий для имитации толщины
                        for offset in 0..line_width as i32 {
                            let offset_start = Point::new(start.x + offset, start.y + offset);
                            let offset_end = Point::new(end.x + offset, end.y + offset);
                            draw_line_segment_mut(image, (offset_start.x as f32, offset_start.y as f32), 
                                                 (offset_end.x as f32, offset_end.y as f32), color);
                        }
                    }
                }
            },
            ElementType::Beam | ElementType::Column => {
                // Для стержневых элементов рисуем толстые линии между узлами
                for i in 0..element_points_2d.len() - 1 {
                    let start = element_points_2d[i];
                    let end = element_points_2d[i + 1];
                    
                    // Рисуем толстую линию
                    for offset_x in -(line_width as i32 / 2)..=(line_width as i32 / 2) {
                        for offset_y in -(line_width as i32 / 2)..=(line_width as i32 / 2) {
                            let offset_start = Point::new(start.x + offset_x, start.y + offset_y);
                            let offset_end = Point::new(end.x + offset_x, end.y + offset_y);
                            draw_line_segment_mut(image, (offset_start.x as f32, offset_start.y as f32), 
                                                 (offset_end.x as f32, offset_end.y as f32), color);
                        }
                    }
                }
            },
            ElementType::Unknown => {
                // Для неизвестных элементов рисуем простые линии
                for i in 0..element_points_2d.len() - 1 {
                    let start = element_points_2d[i];
                    let end = element_points_2d[i + 1];
                    draw_line_segment_mut(image, (start.x as f32, start.y as f32), 
                                         (end.x as f32, end.y as f32), color);
                }
            },
        }
        
        Ok(())
    }
    
    /// Рендерит элемент с новой цветовой схемой
    fn render_element_with_adjacency(
        &self,
        image: &mut RgbImage,
        element: &LiraElement,
        projection: &Projection,
        all_elements: &[&LiraElement],
    ) -> Result<()> {
        let color = self.get_element_color_by_adjacency(element, all_elements);
        
        match element.element_type {
            ElementType::Beam | ElementType::Column => {
                // Стержневые элементы - толстые линии
                self.render_thick_line_element(image, element, projection, color)?;
            }
            ElementType::Shell => {
                // Пластинчатые элементы - заливка
                self.render_shell_element_filled(image, element, projection, color)?;
            }
            ElementType::Unknown => {
                // Рендерим как точку
                if let Some(coord) = element.coordinates.first() {
                    let point_2d = self.project_point(*coord, projection);
                    if point_2d.x >= 0 && point_2d.x < image.width() as i32 && 
                       point_2d.y >= 0 && point_2d.y < image.height() as i32 {
                        image.put_pixel(point_2d.x as u32, point_2d.y as u32, color);
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// Определяет цвет элемента по прилеганию к стержневым как в рабочем проекте
      fn get_element_color_by_adjacency(&self, element: &LiraElement, all_elements: &[&LiraElement]) -> Rgb<u8> {
          match element.element_type {
              ElementType::Shell => {
                  // Проверяем, прилегает ли пластинчатый элемент к стержневому
                  let is_adjacent_to_beam = self.is_adjacent_to_beam_by_nodes(element, all_elements);
                  
                  // Отладочный вывод для третьего этажа (z=6.6)
                  if !element.coordinates.is_empty() {
                      let z_coord = element.coordinates[0].z;
                      if (z_coord - 6.6).abs() < 0.1 {
                          let debug_line = format!("🔍 ОТЛАДКА ЭТАЖ 3: Элемент {} (z={:.1}), узлы: {:?}, прилегает к стержню: {}\n", 
                                   element.id, z_coord, element.nodes, is_adjacent_to_beam);
                          if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open("output/floor3_adjacency_debug.txt") {
                              use std::io::Write;
                              let _ = file.write_all(debug_line.as_bytes());
                          }
                      }
                  }
                  
                  if is_adjacent_to_beam {
                      Rgb([135, 206, 250]) // Светло-голубой (SkyBlue) для прилегающих к стержням
                  } else {
                      Rgb([255, 255, 0])   // Ярко-желтый для не прилегающих
                  }
              }
              ElementType::Beam | ElementType::Column => {
                  Rgb([139, 69, 19])   // Коричневый для стержневых элементов
              }
              ElementType::Unknown => {
                  Rgb([128, 128, 128]) // Серый для неизвестных
              }
          }
      }
    
    /// Проверяет, прилегает ли пластинчатый элемент к стержневому по узлам
    fn is_adjacent_to_beam_by_nodes(&self, shell_element: &LiraElement, all_elements: &[&LiraElement]) -> bool {
        // Проверяем все стержневые элементы
        for beam_element in all_elements {
            if matches!(beam_element.element_type, ElementType::Beam | ElementType::Column) {
                // Проверяем, есть ли общие узлы между пластинчатым и стержневым элементом
                // Стержневой элемент имеет 2 узла, пластинчатый - 3 или 4
                // Если у стержня есть 2 узла, которые есть у пластины - значит прилегают
                
                if beam_element.nodes.len() >= 2 {
                    let beam_nodes = &beam_element.nodes;
                    let shell_nodes = &shell_element.nodes;
                    
                    // Проверяем, есть ли у пластины сторона, совпадающая со стержнем
                    // Для этого ищем 2 соседних узла пластины, которые совпадают с узлами стержня
                    for i in 0..shell_nodes.len() {
                        let node1 = shell_nodes[i];
                        let node2 = shell_nodes[(i + 1) % shell_nodes.len()];
                        
                        // Проверяем, совпадает ли эта сторона пластины со стержнем
                        if (beam_nodes.contains(&node1) && beam_nodes.contains(&node2)) {
                            return true;
                        }
                    }
                }
            }
        }
        
        false
    }
    
    /// Рендерит стержневой элемент толстыми линиями
    fn render_thick_line_element(
        &self,
        image: &mut RgbImage,
        element: &LiraElement,
        projection: &Projection,
        color: Rgb<u8>,
    ) -> Result<()> {
        let element_points_2d: Vec<Point<i32>> = element.coordinates
            .iter()
            .map(|&coord| self.project_point(coord, projection))
            .collect();
        
        let line_width = self.settings.beam_line_width * 2.0; // Увеличиваем толщину
        
        for i in 0..element_points_2d.len() - 1 {
            let start = element_points_2d[i];
            let end = element_points_2d[i + 1];
            
            // Рисуем толстую линию
            for offset_x in -(line_width as i32 / 2)..=(line_width as i32 / 2) {
                for offset_y in -(line_width as i32 / 2)..=(line_width as i32 / 2) {
                    let offset_start = Point::new(start.x + offset_x, start.y + offset_y);
                    let offset_end = Point::new(end.x + offset_x, end.y + offset_y);
                    draw_line_segment_mut(image, (offset_start.x as f32, offset_start.y as f32), 
                                         (offset_end.x as f32, offset_end.y as f32), color);
                }
            }
        }
        
        Ok(())
    }
    
    /// Рендерит пластинчатый элемент точно как в рабочем проекте
        fn render_shell_element_filled(
            &self,
            image: &mut RgbImage,
            element: &LiraElement,
            projection: &Projection,
            color: Rgb<u8>,
        ) -> Result<()> {
            if element.coordinates.len() < 3 {
                return Ok(());
            }
            
            // Преобразуем координаты ТОЧНО как в рабочем проекте
              let points: Vec<imageproc::point::Point<f64>> = element.coordinates.iter().map(|coord| {
                  let normalized_x = coord.x as f64 - projection.min_x;
                  let normalized_y = coord.y as f64 - projection.min_y;
                  imageproc::point::Point::new(
                      normalized_x * projection.scale + projection.offset_x,
                      normalized_y * projection.scale + projection.offset_y
                  )
              }).collect();
            
            // Заливка четырехугольника/треугольника ТОЧНО как в рабочем проекте
            let polygon_points: Vec<imageproc::point::Point<i32>> = points.iter().map(|p| {
                imageproc::point::Point::new(p.x as i32, p.y as i32)
            }).collect();
            
            // Фильтруем ВСЕ дублирующиеся точки для предотвращения ошибки imageproc
            let mut unique_points = Vec::new();
            for point in polygon_points {
                if !unique_points.contains(&point) {
                    unique_points.push(point);
                }
            }
            
            // Дополнительная проверка: убираем первую точку если она равна последней
            if unique_points.len() > 2 && unique_points.first() == unique_points.last() {
                unique_points.pop();
            }
            
            // Рисуем только если есть достаточно уникальных точек
            if unique_points.len() >= 3 {
                // Используем переданный цвет для заливки
                draw_polygon_mut(image, &unique_points, color);
            } else {
                // Убрано: предупреждение о пропуске
            }
            
            // Контуры поверх заливки ТОЧНО как в рабочем проекте
             if unique_points.len() >= 3 {
                 // Рисуем контуры для всех уникальных точек
                 for i in 0..unique_points.len() {
                     let next = (i + 1) % unique_points.len();
                     let start = unique_points[i];
                     let end = unique_points[next];
                     
                     // Проверяем что линия не нулевой длины
                     if start.x != end.x || start.y != end.y {
                         draw_line_segment_mut(
                             image,
                             (start.x as f32, start.y as f32),
                             (end.x as f32, end.y as f32),
                             Rgb([0, 0, 0]), // Черные контуры
                         );
                     }
                 }
             }
            
            Ok(())
        }

    /// Вычисляет проекцию 3D координат в 2D как в рабочем проекте
    fn calculate_projection(&self, elements: &[LiraElement]) -> Result<Projection> {
        if elements.is_empty() {
            return Ok(Projection {
                points_2d: Vec::new(),
                min_x: 0.0,
                max_x: 0.0,
                min_y: 0.0,
                max_y: 0.0,
                scale: 1.0,
                offset_x: 0.0,
                offset_y: 0.0,
            });
        }
        
        // Находим границы всех элементов
        let mut min_x = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_y = f64::NEG_INFINITY;
        
        for element in elements {
              for coord in &element.coordinates {
                  min_x = min_x.min(coord.x as f64);
                  max_x = max_x.max(coord.x as f64);
                  min_y = min_y.min(coord.y as f64);
                  max_y = max_y.max(coord.y as f64);
              }
          }
        
        // Размеры контента
        let content_width = max_x - min_x;
        let content_height = max_y - min_y;
        
        // Параметры изображения как в рабочем проекте
        let margin_pixels = 50.0; // Отступы
        let available_width = self.settings.image_width as f64 - 2.0 * margin_pixels;
        let available_height = self.settings.image_height as f64 - 2.0 * margin_pixels;
        
        // Вычисляем масштаб как в рабочем проекте
        let scale_x = available_width / content_width;
        let scale_y = available_height / content_height;
        let coord_scale = scale_x.min(scale_y) * 0.968; // Безопасный масштаб как в рабочем проекте
        
        // Вычисляем смещения для центрирования
        let scaled_content_width = content_width * coord_scale;
        let scaled_content_height = content_height * coord_scale;
        let offset_x = margin_pixels + (available_width - scaled_content_width) / 2.0;
        let offset_y = margin_pixels + (available_height - scaled_content_height) / 2.0;
        
        Ok(Projection {
            points_2d: Vec::new(),
            min_x,
            max_x,
            min_y,
            max_y,
            scale: coord_scale,
            offset_x,
            offset_y,
        })
    }

    /// Проецирует 3D точку в 2D как в рабочем проекте
      fn project_point(&self, point_3d: Vec3, projection: &Projection) -> Point<i32> {
          // Нормализуем координаты относительно минимальных значений
          let normalized_x = point_3d.x as f64 - projection.min_x;
          let normalized_y = point_3d.y as f64 - projection.min_y;
          
          // Применяем масштаб и смещение
          let x = (normalized_x * projection.scale + projection.offset_x) as i32;
          let y = (normalized_y * projection.scale + projection.offset_y) as i32;
          
          Point::new(x, y)
      }

    /// Генерирует цвета для групп
    fn generate_group_colors(&mut self, groups: &[ConnectedGroup]) {
        self.group_colors.clear();
        
        for (i, group) in groups.iter().enumerate() {
            // Генерируем цвет на основе HSV для равномерного распределения
            let hue = (i as f32 * 360.0 / groups.len() as f32) % 360.0;
            let hsv = Hsv::new(hue, 0.8, 0.9);
            let rgb: Srgb = hsv.into_color();
            
            let color = Rgb([
                (rgb.red * 255.0) as u8,
                (rgb.green * 255.0) as u8,
                (rgb.blue * 255.0) as u8,
            ]);
            
            self.group_colors.insert(group.id, color);
        }
    }

    /// Добавляет заголовок к изображению
    fn add_title(&self, image: &mut RgbImage, title: &str) -> Result<()> {
        // Простая реализация - можно улучшить с помощью внешнего шрифта
        // Убрано избыточное логирование для ускорения работы
        Ok(())
    }

    /// Добавляет легенду к изображению
    fn add_legend(&self, image: &mut RgbImage, groups: &[ConnectedGroup]) -> Result<()> {
        // Убрано избыточное логирование для ускорения работы
        Ok(())
    }

    /// Добавляет статистику группы к изображению
    fn add_group_statistics(&self, image: &mut RgbImage, group: &ConnectedGroup) -> Result<()> {
        // Убрано избыточное логирование для ускорения работы
        Ok(())
    }

    /// Конвертирует изображение в PNG байты
    fn image_to_png_bytes(&self, image: RgbImage) -> Result<Vec<u8>> {
        let mut buffer = Vec::new();
        
        image.write_to(&mut std::io::Cursor::new(&mut buffer), image::ImageFormat::Png)
            .context("Ошибка кодирования PNG")?;
        
        Ok(buffer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_element() -> LiraElement {
        LiraElement {
            id: 1,
            element_type: ElementType::Shell,
            nodes: vec![1, 2, 3, 4],
            coordinates: vec![
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(1.0, 0.0, 0.0),
                Vec3::new(1.0, 1.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
            ],
            properties: ElementProperties::default(),
        }
    }

    #[test]
    fn test_calculate_projection() {
        let visualizer = Visualizer::new();
        let elements = vec![create_test_element()];
        
        let projection = visualizer.calculate_projection(&elements).unwrap();
        
        assert!(projection.scale > 0.0);
        assert_eq!(projection.offset, Vec3::new(0.5, 0.5, 0.0));
    }

    #[test]
    fn test_project_point() {
        let visualizer = Visualizer::new();
        let projection = Projection {
            points_2d: Vec::new(),
            scale: 100.0,
            offset: Vec3::ZERO,
        };
        
        let point_2d = visualizer.project_point(Vec3::new(1.0, 1.0, 0.0), &projection);
        
        // Проверяем, что точка проецируется в разумные координаты
        assert!(point_2d.x > 0);
        assert!(point_2d.y > 0);
    }
}