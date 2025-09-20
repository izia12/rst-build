use serde::{Deserialize, Serialize};
use glam::Vec3;

/// Тип элемента в LIRA
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ElementType {
    /// Пластинчатый элемент (оболочка)
    Shell,
    /// Стержневой элемент (балка, ригель)
    Beam,
    /// Колонна
    Column,
    /// Неизвестный тип
    Unknown,
}

/// Элемент конструкции
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiraElement {
    /// Номер элемента
    pub id: u32,
    /// Тип элемента
    pub element_type: ElementType,
    /// Номера узлов
    pub nodes: Vec<u32>,
    /// Координаты узлов
    pub coordinates: Vec<Vec3>,
    /// Дополнительные свойства
    pub properties: ElementProperties,
}

/// Свойства элемента
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ElementProperties {
    /// Материал
    pub material_id: Option<u32>,
    /// Толщина (для пластинчатых)
    pub thickness: Option<f32>,
    /// Сечение (для стержневых)
    pub section_id: Option<u32>,
    /// Дополнительные параметры
    pub extra: std::collections::HashMap<String, String>,
}

/// Узел конструкции
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiraNode {
    /// Номер узла
    pub id: u32,
    /// Координаты
    pub position: Vec3,
    /// Связанные элементы
    pub connected_elements: Vec<u32>,
}

/// Группа связанных элементов
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectedGroup {
    /// Номер группы
    pub id: usize,
    /// Элементы в группе
    pub elements: Vec<u32>,
    /// Тип группы (преобладающий тип элементов)
    pub group_type: ElementType,
    /// Ограничивающий прямоугольник
    pub bounding_box: BoundingBox,
    /// Статистика
    pub stats: GroupStatistics,
}

/// Ограничивающий прямоугольник
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundingBox {
    pub min: Vec3,
    pub max: Vec3,
}

/// Статистика группы
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GroupStatistics {
    /// Количество пластинчатых элементов
    pub shell_count: usize,
    /// Количество стержневых элементов
    pub beam_count: usize,
    /// Количество колонн
    pub column_count: usize,
    /// Общее количество узлов
    pub total_nodes: usize,
    /// Площадь (для пластинчатых)
    pub total_area: Option<f32>,
    /// Общая длина (для стержневых)
    pub total_length: Option<f32>,
}

/// Результат парсинга
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseResult {
    /// Все элементы
    pub elements: Vec<LiraElement>,
    /// Все узлы
    pub nodes: Vec<LiraNode>,
    /// Метаданные файла
    pub metadata: FileMetadata,
}

/// Метаданные файла
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileMetadata {
    /// Имя файла
    pub filename: String,
    /// Версия LIRA
    pub lira_version: Option<String>,
    /// Дата создания
    pub created_date: Option<String>,
    /// Комментарии
    pub comments: Vec<String>,
}

impl LiraElement {
    /// Проверяет, является ли элемент пластинчатым
    pub fn is_shell(&self) -> bool {
        matches!(self.element_type, ElementType::Shell)
    }

    /// Проверяет, является ли элемент стержневым
    pub fn is_beam(&self) -> bool {
        matches!(self.element_type, ElementType::Beam | ElementType::Column)
    }

    /// Вычисляет центр элемента
    pub fn center(&self) -> Vec3 {
        if self.coordinates.is_empty() {
            return Vec3::ZERO;
        }
        
        let sum = self.coordinates.iter().fold(Vec3::ZERO, |acc, &coord| acc + coord);
        sum / self.coordinates.len() as f32
    }

    /// Вычисляет ограничивающий прямоугольник элемента
    pub fn bounding_box(&self) -> BoundingBox {
        if self.coordinates.is_empty() {
            return BoundingBox {
                min: Vec3::ZERO,
                max: Vec3::ZERO,
            };
        }

        let mut min = self.coordinates[0];
        let mut max = self.coordinates[0];

        for &coord in &self.coordinates {
            min = min.min(coord);
            max = max.max(coord);
        }

        BoundingBox { min, max }
    }
}

impl BoundingBox {
    /// Объединяет два ограничивающих прямоугольника
    pub fn union(&self, other: &BoundingBox) -> BoundingBox {
        BoundingBox {
            min: self.min.min(other.min),
            max: self.max.max(other.max),
        }
    }

    /// Размеры прямоугольника
    pub fn size(&self) -> Vec3 {
        self.max - self.min
    }

    /// Центр прямоугольника
    pub fn center(&self) -> Vec3 {
        (self.min + self.max) * 0.5
    }
}

impl ConnectedGroup {
    /// Создает новую группу
    pub fn new(id: usize, elements: Vec<u32>) -> Self {
        Self {
            id,
            elements,
            group_type: ElementType::Unknown,
            bounding_box: BoundingBox {
                min: Vec3::ZERO,
                max: Vec3::ZERO,
            },
            stats: GroupStatistics::default(),
        }
    }

    /// Обновляет статистику группы
    pub fn update_statistics(&mut self, elements: &[LiraElement]) {
        self.stats = GroupStatistics::default();
        
        let group_elements: Vec<_> = elements
            .iter()
            .filter(|e| self.elements.contains(&e.id))
            .collect();

        for element in &group_elements {
            match element.element_type {
                ElementType::Shell => self.stats.shell_count += 1,
                ElementType::Beam => self.stats.beam_count += 1,
                ElementType::Column => self.stats.column_count += 1,
                ElementType::Unknown => {},
            }
        }

        // Определяем преобладающий тип
        self.group_type = if self.stats.shell_count > self.stats.beam_count + self.stats.column_count {
            ElementType::Shell
        } else if self.stats.beam_count > self.stats.column_count {
            ElementType::Beam
        } else if self.stats.column_count > 0 {
            ElementType::Column
        } else {
            ElementType::Unknown
        };

        // Вычисляем ограничивающий прямоугольник
        if !group_elements.is_empty() {
            self.bounding_box = group_elements[0].bounding_box();
            for element in group_elements.iter().skip(1) {
                self.bounding_box = self.bounding_box.union(&element.bounding_box());
            }
        }

        // Подсчитываем уникальные узлы
        let mut unique_nodes = std::collections::HashSet::new();
        for element in &group_elements {
            for &node_id in &element.nodes {
                unique_nodes.insert(node_id);
            }
        }
        self.stats.total_nodes = unique_nodes.len();
    }
}