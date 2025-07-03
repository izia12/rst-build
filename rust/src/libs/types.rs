use std::{collections::HashMap, path::Path};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
#[derive(Debug, Clone)]
pub struct Coordinate {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone)]
pub struct Element {
    pub element_type: u32,
    pub material: u32,
    pub coordinates: Vec<Coordinate>,
}
// Структура для хранения документа
#[derive(Debug, Clone)]
pub struct Document {
    pub id: String,      // Идентификатор документа
    pub content: String, // Содержимое документа
}

// Главная структура для хранения всех данных из файла
#[derive(Debug)]
pub struct LiraFile {
    pub documents: HashMap<String, Document>, // Все документы
    pub coordinates: Vec<Coordinate>,         // Все координаты из документа 4
    pub elements: Vec<Element>,               // Все элементы из документа 1
}
impl LiraFile {
    // Создание новой структуры LiraFile
    pub fn new() -> Self {
        LiraFile {
            documents: HashMap::new(),
            coordinates: Vec::new(),
            elements: Vec::new(),
        }
    }

    // Парсинг файла
    pub fn parse_file<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let mut current_document: Option<String> = None;
        let mut current_content = String::new();
        let mut is_first_line_in_document = true;

        // Читаем файл построчно
        for line in reader.lines() {
            let line = line?;
            let trimmed_line = line.trim();
            self.process_line(
                trimmed_line,
                &mut current_document,
                &mut current_content,
                &mut is_first_line_in_document,
            );
        }
        // Обрабатываем документы
        self.process_documents();
        Ok(())
    }

    // Парсинг данных из строки
    pub fn parse_file_from_string(&mut self, content: &str) -> io::Result<()> {
        let mut current_document: Option<String> = None;
        let mut current_content = String::new();
        let mut is_first_line_in_document = true;

        // Читаем строки из переданного содержимого
        for line in content.lines() {
            let trimmed_line = line.trim();

            self.process_line(
                trimmed_line,
                &mut current_document,
                &mut current_content,
                &mut is_first_line_in_document,
            );
        }

        // Обрабатываем документы
        self.process_documents();

        Ok(())
    }

    // Обработка одной строки при парсинге
    fn process_line(
        &mut self,
        trimmed_line: &str,
        current_document: &mut Option<String>,
        current_content: &mut String,
        is_first_line_in_document: &mut bool,
    ) {
        // Проверяем, начинается ли строка с открывающей скобки
        if trimmed_line.starts_with("(") && trimmed_line.contains("/") {
            // Извлекаем ID документа
            let parts: Vec<&str> = trimmed_line.split('/').collect();
            if parts.len() > 0 {
                let doc_id = parts[0].trim().trim_start_matches('(').trim();
                *current_document = Some(doc_id.to_string());
                current_content.clear();
                *is_first_line_in_document = true;

                // Добавляем оставшуюся часть строки после "/" в содержимое
                if parts.len() > 1 {
                    let mut content_part = String::new();
                    for i in 1..parts.len() {
                        if i > 1 {
                            content_part.push('/');
                        }
                        content_part.push_str(parts[i]);
                    }
                    let content_part = content_part.trim();
                    if !content_part.is_empty() {
                        current_content.push_str(content_part);
                        *is_first_line_in_document = false;
                    }
                }
            }
        }
        // Проверяем, заканчивается ли строка закрывающей скобкой
        else if trimmed_line == ")" && current_document.is_some() {
            // Сохраняем документ
            if let Some(doc_id) = current_document.take() {
                self.documents.insert(
                    doc_id.clone(),
                    Document {
                        id: doc_id.clone(),
                        content: current_content.trim().to_string(),
                    },
                );
                // Не обрабатываем документы здесь, только сохраняем их
            }
            current_content.clear();
            *is_first_line_in_document = true;
        }
        // Добавляем строку к содержимому текущего документа
        else if current_document.is_some() {
            if !*is_first_line_in_document {
                // Добавляем разделитель между элементами
                current_content.push('/');
            }
            current_content.push_str(trimmed_line);
            *is_first_line_in_document = false;
        }
    }

    // Обработка документов после парсинга
    fn process_documents(&mut self) {
        // Обрабатываем документы
        if let Some(doc4) = self.documents.get("4") {
            self.parse_coordinates(&doc4.content.clone());
        }

        if let Some(doc1) = self.documents.get("1") {
            self.parse_elements(&doc1.content.clone());
        }

        // После того как все документы прочитаны, обрабатываем их в нужном порядке
        // Сначала обрабатываем координаты (документ 4)
        if let Some(doc) = self.documents.get("4") {
            self.parse_coordinates(&doc.content.clone());
            println!("Прочитано координат: {}", self.coordinates.len());
        }

        // Затем обрабатываем элементы (документ 1)
        if let Some(doc) = self.documents.get("1") {
            self.parse_elements(&doc.content.clone());
            println!("Прочитано элементов: {}", self.elements.len());
        }

        // Ok(())
    }

    // ... existing code ...

    // Парсинг координат из документа 4
    // Парсинг координат из документа 4
    fn parse_coordinates(&mut self, content: &str) {
        // self.coordinates.clear();

        // Добавляем фиктивную координату с индексом 0, так как индексы в файле начинаются с 1
        self.coordinates.push(Coordinate {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        });

        // Разделяем содержимое на отдельные координаты по символу '/'
        let coords: Vec<&str> = content.split('/').collect();

        for coord_str in coords {
            let coord_str = coord_str.trim();
            if coord_str.is_empty() {
                continue;
            }
            let values: Vec<&str> = coord_str.split_whitespace().collect();
            if values.len() >= 3 {
                let x = values[0].parse::<f64>().unwrap_or(0.0);
                let y = values[1].parse::<f64>().unwrap_or(0.0);
                let z = values[2].parse::<f64>().unwrap_or(0.0);
                self.coordinates.push(Coordinate { x, y, z });
                println!(
                    "Добавлена координата {}: x={}, y={}, z={}",
                    self.coordinates.len() - 1,
                    x,
                    y,
                    z
                );
            }
        }

        println!("Прочитано координат: {}", self.coordinates.len() - 1); // Вычитаем фиктивную координату
    }

    // Парсинг элементов из документа 1
    // Парсинг элементов из документа 1
    // Парсинг элементов из документа 1
    fn parse_elements(&mut self, content: &str) {
        self.elements.clear();

        // Разделяем содержимое на отдельные элементы по символу '/'
        let elements: Vec<&str> = content.split('/').collect();
        println!("Элементов для парсинга: {}", elements.len());
        println!("Всего координат: {}", self.coordinates.len());

        for (idx, elem_str) in elements.iter().enumerate() {
            let elem_str = elem_str.trim();
            if elem_str.is_empty() {
                continue;
            }

            let values: Vec<&str> = elem_str.split_whitespace().collect();

            if values.len() >= 2 {
                // Минимум тип и материал
                let element_type = values[0].parse::<u32>().unwrap_or(0);
                let material = values[1].parse::<u32>().unwrap_or(0);

                let mut coordinates = Vec::new();

                // Обрабатываем координаты (индексы в документе 4)
                for i in 2..values.len() {
                    if let Ok(coord_index) = values[i].parse::<usize>() {
                        // Получаем координату по индексу (индексы в файле начинаются с 1)
                        // Важно: coord_index - это индекс в файле, который начинается с 1
                        if coord_index > 0 && coord_index < self.coordinates.len() {
                            coordinates.push(self.coordinates[coord_index].clone());
                            println!(
                                "Добавлена координата {} для элемента {}: x={}, y={}, z={}",
                                coord_index,
                                idx,
                                self.coordinates[coord_index].x,
                                self.coordinates[coord_index].y,
                                self.coordinates[coord_index].z
                            );
                        } else {
                            println!("Предупреждение: индекс координаты {} выходит за пределы массива (размер {})", 
                                 coord_index, self.coordinates.len());
                        }
                    }
                }

                println!(
                    "Элемент {}: тип {}, материал {}, координат {}",
                    idx,
                    element_type,
                    material,
                    coordinates.len()
                );

                self.elements.push(Element {
                    element_type,
                    material,
                    coordinates,
                });
            }
        }
    }

    // Получение всех элементов
    pub fn get_elements(&self) -> &Vec<Element> {
        &self.elements
    }

    // Получение координаты по индексу
    pub fn get_coordinate(&self, index: usize) -> Option<&Coordinate> {
        self.coordinates.get(index)
    }

    // Получение всех координат
    pub fn get_coordinates(&self) -> &Vec<Coordinate> {
        &self.coordinates
    }
}

// Другие общие структуры...
