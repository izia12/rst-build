use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use std::collections::HashSet;

use std::fs;

use crate::libs::parse::{get_indexes, SerializableEntity};



// Структура для хранения координат
#[derive(Debug, Clone)]
pub struct Coordinate {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

// Структура для хранения элемента
#[derive(Debug, Clone)]
pub struct Element {
    pub element_type: u32,      // Тип элемента (например, 44 - четырехугольник)
    pub material: u32,          // Материал элемента
    pub coordinates: Vec<Coordinate>, // Координаты элемента
}

// Структура для хранения документа
#[derive(Debug, Clone)]
pub struct Document {
    pub id: String,             // Идентификатор документа
    pub content: String,        // Содержимое документа
}

// Главная структура для хранения всех данных из файла
#[derive(Debug)]
pub struct LiraFile {
    pub documents: HashMap<String, Document>,  // Все документы
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
   // Парсинг файла
// ... existing code ...

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
        
        self.process_line(trimmed_line, &mut current_document, &mut current_content, &mut is_first_line_in_document);
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
        
        self.process_line(trimmed_line, &mut current_document, &mut current_content, &mut is_first_line_in_document);
    }
    
    // Обрабатываем документы
    self.process_documents();
    
    Ok(())
}

// Обработка одной строки при парсинге
fn process_line(&mut self, trimmed_line: &str, current_document: &mut Option<String>, current_content: &mut String, is_first_line_in_document: &mut bool) {
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
                self.documents.insert(doc_id.clone(), Document {
                    id: doc_id.clone(),
                    content: current_content.trim().to_string(),
                });
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
    self.coordinates.push(Coordinate { x: 0.0, y: 0.0, z: 0.0 });
    
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
            println!("Добавлена координата {}: x={}, y={}, z={}", 
                     self.coordinates.len() - 1, x, y, z);
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
        
        if values.len() >= 2 { // Минимум тип и материал
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
                        println!("Добавлена координата {} для элемента {}: x={}, y={}, z={}", 
                                 coord_index, idx, 
                                 self.coordinates[coord_index].x,
                                 self.coordinates[coord_index].y,
                                 self.coordinates[coord_index].z);
                    } else {
                        println!("Предупреждение: индекс координаты {} выходит за пределы массива (размер {})", 
                                 coord_index, self.coordinates.len());
                    }
                }
            }
            
            println!("Элемент {}: тип {}, материал {}, координат {}", 
                     idx, element_type, material, coordinates.len());
            
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

// Пример использования
pub fn parse_lira_file(file_path: &str) -> io::Result<LiraFile> {
    let mut lira_file = LiraFile::new();
    lira_file.parse_file(file_path)?;
    Ok(lira_file)
}
pub fn main() {
	verify_element_indices("src/cafe_muslim04.txt", "src/cafe_muslim04.sli");
    // let lira_file = parse_lira_file("src/cafe_muslim04.txt").unwrap();

    // // Создаем CSV файл
    // let file_path = "lira_elements.csv";
    // let mut file = match std::fs::File::create(file_path) {
    //     Ok(file) => file,
    //     Err(e) => {
    //         eprintln!("Ошибка при создании файла: {}", e);
    //         return;
    //     }
    // };

    // // Записываем заголовок CSV
    // if let Err(e) = writeln!(file, "Номер,Индекс координаты,Координаты X,Координаты Y,Координаты Z,Тип,Материал") {
    //     eprintln!("Ошибка при записи заголовка: {}", e);
    //     return;
    // }

    // // Получение всех элементов
    // let elements = lira_file.get_elements();
    // println!("Всего элементов: {}", elements.len());

    // for (i, element) in elements.iter().enumerate() {
    //     if element.coordinates.is_empty() {
    //         // Если координат нет, записываем строку с нулевыми координатами
    //         if let Err(e) = writeln!(file, "{},0,0.0,0.0,0.0,{},{}", 
    //                                i, element.element_type, element.material) {
    //             eprintln!("Ошибка при записи элемента {}: {}", i, e);
    //         }
    //     } else {
    //         // Записываем каждую координату элемента в отдельной строке
    //         for (j, coord) in element.coordinates.iter().enumerate() {
    //             if let Err(e) = writeln!(file, "{},{},{},{},{},{},{}", 
    //                                    i, j, coord.x, coord.y, coord.z, 
    //                                    element.element_type, element.material) {
    //                 eprintln!("Ошибка при записи элемента {} координаты {}: {}", i, j, e);
    //                 continue;
    //             }
    //         }
    //     }
    // }

    // println!("CSV файл успешно создан: {}", file_path);
}
// Добавьте эту функцию в файл lira_parse.rs

pub fn verify_element_indices(txt_file_path: &str, sli_file_path: &str) -> io::Result<()> {
    // Парсим TXT файл
    let lira_file = parse_lira_file(txt_file_path)?;
    let txt_elements = lira_file.get_elements();
    
    // Парсим SLI файл
    let sli_content = fs::read_to_string(sli_file_path)?;
    let txt_content = fs::read_to_string(txt_file_path)?;
    let (sli_entities, _) = get_indexes(&sli_content, &txt_content);
    
    println!("Количество элементов в TXT: {}", txt_elements.len());
    println!("Количество элементов в SLI: {}", sli_entities.len());
    
    // Создаем хэш-мапы для быстрого поиска элементов по координатам
    let mut txt_elements_map: HashMap<String, usize> = HashMap::new();
    let mut sli_elements_map: HashMap<String, usize> = HashMap::new();
    
    // Заполняем хэш-мапу для TXT элементов
    for (i, element) in txt_elements.iter().enumerate() {
        // Создаем ключ из отсортированных координат
        let mut coords_set: Vec<(f64, f64, f64)> = element.coordinates.iter()
            .map(|c| (c.x, c.y, c.z))
            .collect();
        coords_set.sort_by(|a, b| {
            a.0.partial_cmp(&b.0).unwrap()
                .then(a.1.partial_cmp(&b.1).unwrap())
                .then(a.2.partial_cmp(&b.2).unwrap())
        });
        let key = coords_set.iter()
            .map(|(x, y, z)| format!("{:.5},{:.5},{:.5}", x, y, z))
            .collect::<Vec<String>>()
            .join("|");
        txt_elements_map.insert(key, i);
    }
    // Заполняем хэш-мапу для SLI элементов
    for (i, entity) in sli_entities.iter().enumerate() {
        // Создаем ключ из отсортированных координат
        let mut coords_set: Vec<(f64, f64, f64)> = entity.vertices.iter()
            .map(|v| (v.x, v.y, v.z))
            .collect();
        coords_set.sort_by(|a, b| {
            a.0.partial_cmp(&b.0).unwrap()
                .then(a.1.partial_cmp(&b.1).unwrap())
                .then(a.2.partial_cmp(&b.2).unwrap())
        });
        let key = coords_set.iter()
            .map(|(x, y, z)| format!("{:.5},{:.5},{:.5}", x, y, z))
            .collect::<Vec<String>>()
            .join("|");
        sli_elements_map.insert(key, i);
    }

    // Создаем имя CSV файла на основе имен входных файлов
     let txt_filename = Path::new(txt_file_path).file_stem().unwrap().to_str().unwrap();
     let sli_filename = Path::new(sli_file_path).file_stem().unwrap().to_str().unwrap();
     let csv_filename = format!("{}_vs_{}_mismatches.csv", txt_filename, sli_filename);

     // Создаем CSV файл
     let mut csv_file = File::create(&csv_filename)?;
     
     // Записываем заголовок CSV
     writeln!(csv_file, "Тип несоответствия,TXT индекс,SLI индекс,Элемент из txt,Элемент из sli")?;
     // Добавляем пустую строку после заголовков
     writeln!(csv_file, ",,,,")?;
    
    // Проверяем соответствие индексов
    let mut mismatches = 0;
    let mut total_checked = 0;
    let mut txt_only = 0;
    let mut sli_only = 0;
    
    for (key, txt_index) in &txt_elements_map {
        if let Some(sli_index) = sli_elements_map.get(key) {
            total_checked += 1;
            if txt_index != sli_index {
                mismatches += 1;
                println!("Несоответствие индексов: TXT={}, SLI={}, координаты={}", txt_index, sli_index, key);
                // Получаем координаты элементов в виде строк
                let txt_coords = format_element_coords(&txt_elements[*txt_index]);
                let sli_coords = format_entity_coords(&sli_entities[*sli_index]);
                // Записываем в CSV
                writeln!(csv_file, "Несоответствие индексов,{},{},{},{}", txt_index, sli_index, txt_coords, sli_coords)?;
            }
        } else {
            txt_only += 1;
            println!("Элемент с координатами {} найден в TXT (индекс {}), но отсутствует в SLI", key, txt_index);
            // Получаем координаты элемента в виде строки
            let txt_coords = format_element_coords(&txt_elements[*txt_index]);
            // Записываем в CSV
            writeln!(csv_file, "Только в TXT,{},N/A,{},", txt_index, txt_coords)?;
        }
    }
    // Проверяем элементы, которые есть в SLI, но отсутствуют в TXT
    for (key, sli_index) in &sli_elements_map {
        if !txt_elements_map.contains_key(key) {
            sli_only += 1;
            println!("Элемент с координатами {} найден в SLI (индекс {}), но отсутствует в TXT", key, sli_index);
            // Получаем координаты элемента в виде строки
            let sli_coords = format_entity_coords(&sli_entities[*sli_index]);
            // Записываем в CSV
            writeln!(csv_file, "Только в SLI,N/A,{},,{}", sli_index, sli_coords)?;
        }
    }
    // Записываем статистику в CSV
    writeln!(csv_file, "\nСтатистика:,,")?;
    writeln!(csv_file, "Всего элементов в TXT,{},", txt_elements.len())?;
    writeln!(csv_file, "Всего элементов в SLI,{},", sli_entities.len())?;
    writeln!(csv_file, "Проверено элементов,{},", total_checked)?;
    writeln!(csv_file, "Несоответствий индексов,{},", mismatches)?;
    writeln!(csv_file, "Только в TXT,{},", txt_only)?;
    writeln!(csv_file, "Только в SLI,{},", sli_only)?;
    println!("Проверка завершена. Проверено элементов: {}", total_checked);
    println!("Найдено несоответствий индексов: {}", mismatches);
    println!("Элементов только в TXT: {}", txt_only);
    println!("Элементов только в SLI: {}", sli_only);
    if mismatches == 0 && txt_only == 0 && sli_only == 0 {
        println!("Все элементы имеют одинаковые индексы в обоих файлах!");
    } else {
        println!("Обнаружены проблемы с индексами элементов!");
    }
    println!("CSV файл с результатами создан: {}", csv_filename);
    Ok(())
}

// Функция для форматирования координат элемента из TXT файла
fn format_element_coords(element: &Element) -> String {
    element.coordinates.iter()
        .map(|c| format!("{:.5} | {:.5} | {:.5}", c.x, c.y, c.z))
        .collect::<Vec<String>>()
        .join("      ")
}

// Функция для форматирования координат элемента из SLI файла
fn format_entity_coords(entity: &SerializableEntity) -> String {
    entity.vertices.iter()
        .map(|v| format!("{:.5} | {:.5} | {:.5}", v.x, v.y, v.z))
        .collect::<Vec<String>>()
        .join("      ")
}