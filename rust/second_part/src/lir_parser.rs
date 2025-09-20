use crate::types::*;
use anyhow::{Result, Context};
use glam::Vec3;
use log::{debug, info, warn};
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::io::{BufRead, BufReader};
use std::fs::File;

// Импортируем структуры из рабочего проекта
// Временно создадим алиас, чтобы избежать конфликта имён
use std::io;

#[derive(Debug, Clone)]
pub struct WorkingProjectCoordinate {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone)]
pub struct WorkingProjectElement {
    pub element_type: u32,
    pub material: u32,
    pub coordinates: Vec<WorkingProjectCoordinate>,
}

#[derive(Debug)]
pub struct WorkingProjectLiraFile {
    pub documents: HashMap<String, WorkingProjectDocument>,
    pub coordinates: Vec<WorkingProjectCoordinate>,
    pub elements: Vec<WorkingProjectElement>,
}

#[derive(Debug, Clone)]
pub struct WorkingProjectDocument {
    pub id: String,
    pub content: String,
}

impl WorkingProjectLiraFile {
    pub fn new() -> Self {
        WorkingProjectLiraFile {
            documents: HashMap::new(),
            coordinates: Vec::new(),
            elements: Vec::new(),
        }
    }

    pub fn parse_file<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let mut current_document: Option<String> = None;
        let mut current_content = String::new();
        let mut is_first_line_in_document = true;

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
        self.process_documents();
        Ok(())
    }

    fn process_line(
        &mut self,
        trimmed_line: &str,
        current_document: &mut Option<String>,
        current_content: &mut String,
        is_first_line_in_document: &mut bool,
    ) {
        if trimmed_line.starts_with("(") && trimmed_line.contains("/") {
            let parts: Vec<&str> = trimmed_line.split('/').collect();
            if !parts.is_empty() {
                let doc_id = parts[0].trim().trim_start_matches('(').trim();
                *current_document = Some(doc_id.to_string());
                current_content.clear();
                *is_first_line_in_document = true;

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
        } else if trimmed_line == ")" && current_document.is_some() {
            if let Some(doc_id) = current_document.take() {
                self.documents.insert(
                    doc_id.clone(),
                    WorkingProjectDocument {
                        id: doc_id.clone(),
                        content: current_content.trim().to_string(),
                    },
                );
            }
            current_content.clear();
            *is_first_line_in_document = true;
        } else if current_document.is_some() {
            if !*is_first_line_in_document {
                current_content.push('/');
            }
            current_content.push_str(trimmed_line);
            *is_first_line_in_document = false;
        }
    }

    fn process_documents(&mut self) {
        if let Some(doc4) = self.documents.get("4") {
            self.parse_coordinates(&doc4.content.clone());
        }

        if let Some(doc1) = self.documents.get("1") {
            self.parse_elements(&doc1.content.clone());
        }
    }

    fn parse_coordinates(&mut self, content: &str) {
        self.coordinates.clear();
        self.coordinates.push(WorkingProjectCoordinate {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        });

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
                self.coordinates.push(WorkingProjectCoordinate { x, y, z });
            }
        }
    }

    fn parse_elements(&mut self, content: &str) {
        self.elements.clear();
        let elements: Vec<&str> = content.split('/').collect();

        for elem_str in elements {
            let elem_str = elem_str.trim();
            if elem_str.is_empty() {
                continue;
            }

            let values: Vec<&str> = elem_str.split_whitespace().collect();
            if values.len() >= 2 {
                let element_type = values[0].parse::<u32>().unwrap_or(0);
                let material = values[1].parse::<u32>().unwrap_or(0);
                let mut coordinates = Vec::new();

                for i in 2..values.len() {
                    if let Ok(coord_index) = values[i].parse::<usize>() {
                        if coord_index > 0 && coord_index < self.coordinates.len() {
                            coordinates.push(self.coordinates[coord_index].clone());
                        }
                    }
                }

                self.elements.push(WorkingProjectElement {
                    element_type,
                    material,
                    coordinates,
                });
            }
        }
    }

    pub fn parse_file_from_string(&mut self, content: &str) -> io::Result<()> {
        let mut current_document: Option<String> = None;
        let mut current_content = String::new();
        let mut is_first_line_in_document = true;

        for line in content.lines() {
            let trimmed_line = line.trim();
            self.process_line(
                trimmed_line,
                &mut current_document,
                &mut current_content,
                &mut is_first_line_in_document,
            );
        }
        self.process_documents();
        Ok(())
    }

    pub fn get_elements(&self) -> &Vec<WorkingProjectElement> {
        &self.elements
    }
}

/// Парсер .lir файлов (поддерживает как текстовый, так и бинарный формат LIRA)
pub struct LirParser {
    /// Кэш регулярных выражений для текстового формата
    regexes: Option<ParserRegexes>,
    /// Документы из LIRA файла
    documents: HashMap<String, String>,
    /// Координаты (документ 4)
    coordinates: Vec<Vec3>,
    /// Элементы (документ 1)
    raw_elements: Vec<RawElement>,
}

/// Сырой элемент из LIRA файла
#[derive(Debug, Clone)]
struct RawElement {
    element_type: u32,
    material: u32,
    node_indices: Vec<usize>,
}

/// Регулярные выражения для парсинга текстового формата
struct ParserRegexes {
    /// Узел: номер и координаты
    node_pattern: Regex,
    /// Элемент: номер, тип, узлы
    element_pattern: Regex,
    /// Комментарий
    comment_pattern: Regex,
    /// Секция
    section_pattern: Regex,
}

impl LirParser {
    /// Создает новый парсер
    pub fn new() -> Result<Self> {
        Ok(Self {
            regexes: None,
            documents: HashMap::new(),
            coordinates: Vec::new(),
            raw_elements: Vec::new(),
        })
    }
    
    /// Парсит SLI файл и возвращает элементы
    pub fn parse_sli_file<P: AsRef<Path>>(&mut self, file_path: P) -> Result<Vec<LiraElement>> {
        let content = fs::read_to_string(file_path)?;
        self.parse_sli_content(&content)
    }
    
    /// Парсит содержимое SLI файла (XML формат)
    fn parse_sli_content(&mut self, content: &str) -> Result<Vec<LiraElement>> {
        use xml::reader::{EventReader, XmlEvent};
        use std::io::Cursor;
        
        let cursor = Cursor::new(content);
        let parser = EventReader::new(cursor);
        let mut points: Vec<Vec3> = Vec::new();
        let mut elements = Vec::new();
        let mut element_id = 0;
        
        for event in parser {
            match event {
                Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                    match name.local_name.as_str() {
                        "NodeCoords" => {
                            let x = attributes
                                .iter()
                                .find(|attr| attr.name.local_name == "NdX")
                                .unwrap()
                                .value
                                .parse::<f32>()?;
                            let y = attributes
                                .iter()
                                .find(|attr| attr.name.local_name == "NdY")
                                .unwrap()
                                .value
                                .parse::<f32>()?;
                            let z = attributes
                                .iter()
                                .find(|attr| attr.name.local_name == "NdZ")
                                .unwrap()
                                .value
                                .parse::<f32>()?;
                            points.push(Vec3::new(x, y, z));
                        }
                        "Element" => {
                            element_id += 1;
                            let element_type = match attributes
                                .iter()
                                .find(|attr| attr.name.local_name == "Type")
                                .unwrap()
                                .value
                                .as_str()
                            {
                                "1" => ElementType::Beam,
                                "2" => ElementType::Shell,
                                _ => ElementType::Unknown,
                            };
                            
                            // Создаем элемент без узлов пока
                            let element = LiraElement {
                                id: element_id,
                                element_type,
                                nodes: Vec::new(),
                                coordinates: Vec::new(),
                                properties: ElementProperties::default(),
                            };
                            elements.push(element);
                        }
                        "Nodes" => {
                            if let Some(element) = elements.last_mut() {
                                let node_indices: Vec<u32> = attributes
                                    .iter()
                                    .map(|attr| attr.value.parse::<u32>().unwrap_or(0))
                                    .filter(|&x| x > 0)
                                    .collect();
                                
                                element.nodes = node_indices.clone();
                                
                                // Добавляем координаты узлов
                                for &node_index in &node_indices {
                                    if let Some(point) = points.get((node_index - 1) as usize) {
                                        element.coordinates.push(*point);
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
                Ok(XmlEvent::EndElement { .. }) => {}
                Err(e) => {
                    warn!("Ошибка парсинга XML: {}", e);
                    break;
                }
                _ => {}
            }
        }
        
        // info!("Парсинг SLI завершен: {} точек, {} элементов", points.len(), elements.len());
        Ok(elements)
    }
    
    /// Инициализирует регулярные выражения для текстового формата
    fn init_text_regexes(&mut self) -> Result<()> {
        let regexes = ParserRegexes {
            // Узел: "1 0.0 0.0 0.0" или "NODE 1 X=0.0 Y=0.0 Z=0.0"
            node_pattern: Regex::new(r"(?i)(?:node\s+)?(\d+)\s+(?:x=)?([+-]?\d*\.?\d+)\s+(?:y=)?([+-]?\d*\.?\d+)\s+(?:z=)?([+-]?\d*\.?\d+)")?,
            
            // Элемент: "ELEMENT 1 TYPE=SHELL NODES=1,2,3,4" или "1 SHELL 1 2 3 4"
            element_pattern: Regex::new(r"(?i)(?:element\s+)?(\d+)\s+(?:type=)?(\w+)\s+(?:nodes=)?([\d,\s]+)")?,
            
            // Комментарий: "! комментарий" или "# комментарий"
            comment_pattern: Regex::new(r"^\s*[!#](.*)$")?,
            
            // Секция: "[NODES]" или "*NODES"
            section_pattern: Regex::new(r"^\s*[\[\*]([A-Z_]+)[\]\s]*$")?,
        };
        
        self.regexes = Some(regexes);
        Ok(())
    }

    /// Парсит .lir файл
    pub fn parse_file<P: AsRef<Path>>(&mut self, file_path: P) -> Result<Vec<LiraElement>> {
        let file_path = file_path.as_ref();
        // info!("Парсинг файла: {:?}", file_path);

        // Сначала пытаемся прочитать как текст
        let content = match fs::read_to_string(file_path) {
            Ok(content) => content,
            Err(_) => {
                // Пытаемся прочитать как бинарные данные и декодировать
                // info!("Не удалось прочитать как UTF-8, пытаемся другие кодировки");
                let data = fs::read(file_path)
                    .with_context(|| format!("Не удалось прочитать файл: {:?}", file_path))?;
                
                // Пытаемся Windows-1251 или lossy UTF-8
                String::from_utf8_lossy(&data).to_string()
            }
        };
        
        // Проверяем формат содержимого
         if content.contains("[NODES]") || content.contains("[ELEMENTS]") {
             // Текстовый формат
             info!("Обнаружен текстовый формат");
             self.init_text_regexes()?;
             let parse_result = self.parse_text_content(&content, file_path.to_string_lossy().to_string())?;
             info!("Парсинг завершён: {} элементов, {} узлов", 
                   parse_result.elements.len(), parse_result.nodes.len());
             Ok(parse_result.elements)
         } else if content.contains("(") && content.contains(")") {
             // Формат LIRA с документами в скобках
             info!("Обнаружен формат LIRA с документами");
             let mut lira_file = WorkingProjectLiraFile::new();
             lira_file.parse_file_from_string(&content)?;
             let elements = self.convert_from_working_project(&lira_file)?;
             info!("Парсинг завершён: {} элементов", elements.len());
             Ok(elements)
         } else {
             // Неизвестный текстовый формат
             info!("Неизвестный текстовый формат");
             Ok(Vec::new())
         }
    }

    /// Парсит LIRA документы (формат с документами в скобках)
    fn parse_lira_documents(&mut self, content: &str) -> Result<()> {
        self.documents.clear();
        
        let mut current_document: Option<String> = None;
        let mut current_content = String::new();
        let mut is_first_line_in_document = true;
        
        for line in content.lines() {
            let trimmed_line = line.trim();
            
            // Проверяем начало документа: (номер/
            if trimmed_line.starts_with("(") && trimmed_line.contains("/") {
                let parts: Vec<&str> = trimmed_line.split('/').collect();
                if !parts.is_empty() {
                    let doc_id = parts[0].trim().trim_start_matches('(').trim();
                    current_document = Some(doc_id.to_string());
                    current_content.clear();
                    is_first_line_in_document = true;
                    
                    // Добавляем оставшуюся часть строки после "/"
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
                            is_first_line_in_document = false;
                        }
                    }
                }
            }
            // Проверяем конец документа: )
            else if trimmed_line == ")" && current_document.is_some() {
                if let Some(doc_id) = current_document.take() {
                    self.documents.insert(doc_id, current_content.trim().to_string());
                }
                current_content.clear();
                is_first_line_in_document = true;
            }
            // Добавляем строку к содержимому документа
            else if current_document.is_some() {
                if !is_first_line_in_document {
                    current_content.push('/');
                }
                current_content.push_str(trimmed_line);
                is_first_line_in_document = false;
            }
        }
        
        // Обрабатываем документы
        debug!("Найдено документов: {}", self.documents.len());
        for (doc_id, content) in &self.documents {
            debug!("Документ {}: {} символов", doc_id, content.len());
        }
        
        self.process_lira_documents()?;
        
        Ok(())
    }
    
    /// Обрабатывает документы LIRA
    fn process_lira_documents(&mut self) -> Result<()> {
        // Сначала парсим координаты (документ 4)
        if let Some(doc4) = self.documents.get("4").cloned() {
            self.parse_lira_coordinates(&doc4)?;
            debug!("Прочитано координат: {}", self.coordinates.len());
        }
        
        // Затем парсим элементы (документ 1)
        if let Some(doc1) = self.documents.get("1").cloned() {
            self.parse_lira_elements(&doc1)?;
            debug!("Прочитано элементов: {}", self.raw_elements.len());
        }
        
        Ok(())
    }
    
    /// Парсит координаты из документа 4
    fn parse_lira_coordinates(&mut self, content: &str) -> Result<()> {
        self.coordinates.clear();
        
        // Добавляем фиктивную координату с индексом 0
        self.coordinates.push(Vec3::ZERO);
        
        let coords: Vec<&str> = content.split('/').collect();
        
        for coord_str in coords {
            let coord_str = coord_str.trim();
            if coord_str.is_empty() {
                continue;
            }
            
            let values: Vec<&str> = coord_str.split_whitespace().collect();
            if values.len() >= 3 {
                let x = values[0].parse::<f32>().unwrap_or(0.0);
                let y = values[1].parse::<f32>().unwrap_or(0.0);
                let z = values[2].parse::<f32>().unwrap_or(0.0);
                self.coordinates.push(Vec3::new(x, y, z));
                debug!("Координата {}: ({}, {}, {})", self.coordinates.len() - 1, x, y, z);
            }
        }
        
        Ok(())
    }
    
    /// Парсит элементы из документа 1
    fn parse_lira_elements(&mut self, content: &str) -> Result<()> {
        self.raw_elements.clear();
        
        let elements: Vec<&str> = content.split('/').collect();
        
        for elem_str in elements {
            let elem_str = elem_str.trim();
            if elem_str.is_empty() {
                continue;
            }
            
            let values: Vec<&str> = elem_str.split_whitespace().collect();
            
            if values.len() >= 2 {
                let element_type = values[0].parse::<u32>().unwrap_or(0);
                let material = values[1].parse::<u32>().unwrap_or(0);
                let mut node_indices = Vec::new();
                
                // Собираем индексы узлов
                for i in 2..values.len() {
                    if let Ok(node_index) = values[i].parse::<usize>() {
                        node_indices.push(node_index);
                    }
                }
                
                let node_count = node_indices.len();
                
                self.raw_elements.push(RawElement {
                    element_type,
                    material,
                    node_indices,
                });
                
                debug!("Элемент: тип={}, материал={}, узлов={}", 
                       element_type, material, node_count);
            }
        }
        
        Ok(())
    }
    
    /// Конвертирует сырые элементы в LiraElement
    fn convert_to_lira_elements(&self) -> Result<Vec<LiraElement>> {
        let mut elements = Vec::new();
        
        for (id, raw_elem) in self.raw_elements.iter().enumerate() {
            let mut coordinates = Vec::new();
            
            // Получаем координаты по индексам
            for &node_index in &raw_elem.node_indices {
                if node_index > 0 && node_index < self.coordinates.len() {
                    coordinates.push(self.coordinates[node_index]);
                } else {
                    warn!("Индекс узла {} выходит за пределы массива координат", node_index);
                }
            }
            
            // Определяем тип элемента
            let element_type = match raw_elem.element_type {
                1..=10 => ElementType::Shell,  // Пластинчатые
                11..=20 => ElementType::Beam,  // Балки
                21..=30 => ElementType::Column, // Колонны
                _ => ElementType::Unknown,
            };
            
            elements.push(LiraElement {
                id: id as u32 + 1,
                element_type,
                nodes: raw_elem.node_indices.iter().map(|&i| i as u32).collect(),
                coordinates,
                properties: ElementProperties::default(),
            });
        }
        
        Ok(elements)
    }
    
    /// Парсит бинарный LIRA файл используя готовую структуру из рабочего проекта
    fn parse_binary_lira_file<P: AsRef<Path>>(&mut self, file_path: P) -> Result<Vec<LiraElement>> {
        let file_path = file_path.as_ref();
        
        // Используем готовую структуру LiraFile из рабочего проекта
        let mut lira_file = WorkingProjectLiraFile::new();
        lira_file.parse_file(file_path)
            .with_context(|| format!("Не удалось парсить файл: {:?}", file_path))?;
        
        // Конвертируем в наш формат
        let elements = self.convert_from_working_project(&lira_file)?;
        
        info!("Успешно парсили через рабочий проект: {} элементов", elements.len());
        Ok(elements)
    }
    
    /// Конвертирует данные из рабочего проекта в наш формат
    fn convert_from_working_project(&self, lira_file: &WorkingProjectLiraFile) -> Result<Vec<LiraElement>> {
        let mut elements = Vec::new();
        
        for (id, wp_element) in lira_file.get_elements().iter().enumerate() {
            let mut coordinates = Vec::new();
            
            // Конвертируем координаты
            for wp_coord in &wp_element.coordinates {
                coordinates.push(Vec3::new(
                    wp_coord.x as f32,
                    wp_coord.y as f32,
                    wp_coord.z as f32,
                ));
            }
            
            // Определяем тип элемента согласно правилам LIRA
            let element_type = match wp_element.element_type {
                5 | 10 => ElementType::Beam,     // Стержневые элементы
                43 => ElementType::Shell,        // Треугольные пластинчатые
                44 => ElementType::Shell,        // Четырехугольные пластинчатые
                _ => {
                    // Дополнительная логика на основе количества координат
                    match wp_element.coordinates.len() {
                        2 => ElementType::Beam,      // 2 координаты = линия (стержень)
                        3 => ElementType::Shell,     // 3 координаты = треугольник
                        4 => ElementType::Shell,     // 4 координаты = четырехугольник
                        _ => ElementType::Unknown,
                    }
                }
            };
            
            // Создаем узлы с реальными ID (начинаем с базового ID + смещение)
            let base_node_id = (id * 10) as u32 + 1; // Уникальные ID узлов для каждого элемента
            let nodes: Vec<u32> = (0..coordinates.len()).map(|i| base_node_id + i as u32).collect();
            let coord_count = coordinates.len();
            
            elements.push(LiraElement {
                id: id as u32 + 1,
                element_type: element_type.clone(),
                nodes,
                coordinates,
                properties: ElementProperties::default(),
            });
            
            // debug!("Конвертирован элемент {}: тип={:?}, координат={}", 
            //        id + 1, element_type, coord_count);
        }
        
        Ok(elements)
    }
    
    /// Пытается декодировать бинарные данные в разных кодировках
    fn try_decode_binary(&self, data: &[u8]) -> Result<String> {
        debug!("Размер файла: {} байт", data.len());
        debug!("Первые 50 байт: {:?}", &data[..data.len().min(50)]);
        
        // Пропускаем заголовок LIRA-SAPR если есть
        let start_pos = if data.starts_with(b"$LIRA-SAPR") {
            debug!("Найден заголовок LIRA-SAPR");
            // Ищем начало данных после заголовка
            data.iter().position(|&b| b == b'(').unwrap_or(0)
        } else {
            0
        };
        
        debug!("Начальная позиция данных: {}", start_pos);
        let data_slice = &data[start_pos..];
        
        // Пытаемся разные кодировки
        if let Ok(content) = String::from_utf8(data_slice.to_vec()) {
            debug!("Успешно декодировано как UTF-8");
            return Ok(content);
        }
        
        // Пытаемся Windows-1251 (часто используется в российских программах)
        // Для простоты пока используем lossy конвертацию
        let content = String::from_utf8_lossy(data_slice).to_string();
        debug!("Декодировано как UTF-8 lossy, длина: {}", content.len());
        debug!("Первые 200 символов: {}", &content[..content.len().min(200)]);
        
        if content.contains("(") && content.contains(")") {
            debug!("Найдены скобки документов");
            Ok(content)
        } else {
            debug!("Скобки документов не найдены");
            Err(anyhow::anyhow!("Не удалось декодировать файл в поддерживаемой кодировке"))
        }
    }
    
    /// Парсит содержимое текстового файла
    fn parse_text_content(&mut self, content: &str, filename: String) -> Result<ParseResult> {
        let mut nodes = HashMap::new();
        let mut elements = Vec::new();
        let mut metadata = FileMetadata {
            filename,
            ..Default::default()
        };

        let mut current_section = String::new();
        let mut line_number = 0;

        for line in content.lines() {
            line_number += 1;
            let line = line.trim();
            
            // Пропускаем пустые строки
            if line.is_empty() {
                continue;
            }

            // Обрабатываем комментарии
            if let Some(ref regexes) = self.regexes {
                if let Some(captures) = regexes.comment_pattern.captures(line) {
                    let comment = captures.get(1).unwrap().as_str().trim();
                    metadata.comments.push(comment.to_string());
                    debug!("Комментарий: {}", comment);
                    continue;
                }

                // Обрабатываем секции
                if let Some(captures) = regexes.section_pattern.captures(line) {
                    current_section = captures.get(1).unwrap().as_str().to_uppercase();
                    debug!("Секция: {}", current_section);
                    continue;
                }
            }

            // Парсим узлы
            if current_section == "NODES" || current_section == "NODE" {
                if let Some(node) = self.parse_node_line(line)? {
                    nodes.insert(node.id, node);
                }
                continue;
            }

            // Парсим элементы
            if current_section == "ELEMENTS" || current_section == "ELEMENT" {
                if let Some(element) = self.parse_element_line(line, &nodes)? {
                    elements.push(element);
                }
                continue;
            }

            // Пытаемся автоматически определить тип строки
            if let Some(node) = self.parse_node_line(line)? {
                nodes.insert(node.id, node);
            } else if let Some(element) = self.parse_element_line(line, &nodes)? {
                elements.push(element);
            } else {
                debug!("Неизвестная строка {}: {}", line_number, line);
            }
        }

        // Обновляем связи узлов с элементами
        self.update_node_connections(&mut nodes, &elements);

        let nodes_vec: Vec<LiraNode> = nodes.into_values().collect();
        
        Ok(ParseResult {
            elements,
            nodes: nodes_vec,
            metadata,
        })
    }

    /// Парсит строку с узлом
    fn parse_node_line(&self, line: &str) -> Result<Option<LiraNode>> {
        if let Some(ref regexes) = self.regexes {
            if let Some(captures) = regexes.node_pattern.captures(line) {
            let id: u32 = captures.get(1).unwrap().as_str().parse()
                .with_context(|| format!("Неверный номер узла: {}", line))?;
            
            let x: f32 = captures.get(2).unwrap().as_str().parse()
                .with_context(|| format!("Неверная X координата: {}", line))?;
            
            let y: f32 = captures.get(3).unwrap().as_str().parse()
                .with_context(|| format!("Неверная Y координата: {}", line))?;
            
            let z: f32 = captures.get(4).unwrap().as_str().parse()
                .with_context(|| format!("Неверная Z координата: {}", line))?;

                debug!("Узел {}: ({}, {}, {})", id, x, y, z);

                return Ok(Some(LiraNode {
                    id,
                    position: Vec3::new(x, y, z),
                    connected_elements: Vec::new(),
                }));
            }
        }
        Ok(None)
    }

    /// Парсит строку с элементом
    fn parse_element_line(&self, line: &str, nodes: &HashMap<u32, LiraNode>) -> Result<Option<LiraElement>> {
        if let Some(ref regexes) = self.regexes {
            if let Some(captures) = regexes.element_pattern.captures(line) {
            let id: u32 = captures.get(1).unwrap().as_str().parse()
                .with_context(|| format!("Неверный номер элемента: {}", line))?;
            
            let type_str = captures.get(2).unwrap().as_str().to_uppercase();
            let element_type = self.parse_element_type(&type_str);
            
            let nodes_str = captures.get(3).unwrap().as_str();
            let node_ids = self.parse_node_list(nodes_str)
                .with_context(|| format!("Неверный список узлов: {}", line))?;

            // Получаем координаты узлов
            let mut coordinates = Vec::new();
            for &node_id in &node_ids {
                if let Some(node) = nodes.get(&node_id) {
                    coordinates.push(node.position);
                } else {
                    warn!("Узел {} не найден для элемента {}", node_id, id);
                }
            }

                debug!("Элемент {}: тип={:?}, узлы={:?}", id, element_type, node_ids);

                return Ok(Some(LiraElement {
                    id,
                    element_type,
                    nodes: node_ids,
                    coordinates,
                    properties: ElementProperties::default(),
                }));
            }
        }
        Ok(None)
    }

    /// Определяет тип элемента по строке
    fn parse_element_type(&self, type_str: &str) -> ElementType {
        match type_str {
            "SHELL" | "PLATE" | "MEMBRANE" | "QUAD" | "TRIANGLE" => ElementType::Shell,
            "BEAM" | "BAR" | "TRUSS" | "FRAME" => ElementType::Beam,
            "COLUMN" | "PILLAR" => ElementType::Column,
            _ => {
                debug!("Неизвестный тип элемента: {}", type_str);
                ElementType::Unknown
            }
        }
    }

    /// Парсит список номеров узлов
    fn parse_node_list(&self, nodes_str: &str) -> Result<Vec<u32>> {
        let mut node_ids = Vec::new();
        
        // Разделяем по запятым и пробелам
        for part in nodes_str.split(&[',', ' ', '\t']) {
            let part = part.trim();
            if !part.is_empty() {
                let node_id: u32 = part.parse()
                    .with_context(|| format!("Неверный номер узла: {}", part))?;
                node_ids.push(node_id);
            }
        }
        
        Ok(node_ids)
    }

    /// Обновляет связи узлов с элементами
    fn update_node_connections(&self, nodes: &mut HashMap<u32, LiraNode>, elements: &[LiraElement]) {
        for element in elements {
            for &node_id in &element.nodes {
                if let Some(node) = nodes.get_mut(&node_id) {
                    node.connected_elements.push(element.id);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_node_line() {
        let parser = LirParser::new().unwrap();
        
        // Тест простого формата
        let node = parser.parse_node_line("1 0.0 1.5 -2.3").unwrap().unwrap();
        assert_eq!(node.id, 1);
        assert_eq!(node.position, Vec3::new(0.0, 1.5, -2.3));
        
        // Тест формата с ключевыми словами
        let node = parser.parse_node_line("NODE 42 X=10.5 Y=-5.0 Z=0.0").unwrap().unwrap();
        assert_eq!(node.id, 42);
        assert_eq!(node.position, Vec3::new(10.5, -5.0, 0.0));
    }

    #[test]
    fn test_parse_element_line() {
        let parser = LirParser::new().unwrap();
        let nodes = HashMap::new();
        
        // Тест элемента оболочки
        let element = parser.parse_element_line("1 SHELL 1 2 3 4", &nodes).unwrap().unwrap();
        assert_eq!(element.id, 1);
        assert!(element.is_shell());
        assert_eq!(element.nodes, vec![1, 2, 3, 4]);
        
        // Тест стержневого элемента
        let element = parser.parse_element_line("ELEMENT 5 TYPE=BEAM NODES=10,20", &nodes).unwrap().unwrap();
        assert_eq!(element.id, 5);
        assert!(element.is_beam());
        assert_eq!(element.nodes, vec![10, 20]);
    }

    #[test]
    fn test_parse_element_type() {
        let parser = LirParser::new().unwrap();
        
        assert_eq!(parser.parse_element_type("SHELL"), ElementType::Shell);
        assert_eq!(parser.parse_element_type("BEAM"), ElementType::Beam);
        assert_eq!(parser.parse_element_type("COLUMN"), ElementType::Column);
        assert_eq!(parser.parse_element_type("UNKNOWN"), ElementType::Unknown);
    }
}