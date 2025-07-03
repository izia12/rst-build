use std::collections::HashMap;

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
// Другие общие структуры...
