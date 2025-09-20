use crate::types::*;
use log::{debug, info, warn};
use std::collections::{HashMap, HashSet, VecDeque};
use anyhow::Result;

/// Анализатор графов для поиска связных компонент
pub struct GraphAnalyzer {
    /// Граф связей элементов
    element_graph: HashMap<u32, HashSet<u32>>,
    /// Индекс узлов -> элементы
    node_to_elements: HashMap<u32, Vec<u32>>,
}

impl GraphAnalyzer {
    /// Создает новый анализатор
    pub fn new() -> Self {
        Self {
            element_graph: HashMap::new(),
            node_to_elements: HashMap::new(),
        }
    }

    /// Находит связные компоненты элементов
    /// Реализует алгоритм из переписки НР с ChatGPT
    pub fn find_connected_components(&mut self, elements: &[LiraElement]) -> Vec<ConnectedGroup> {
        // Поиск связных компонент
        
        // Ограничиваем количество элементов для больших моделей
        const MAX_ELEMENTS_FOR_ANALYSIS: usize = 10000;
        
        if elements.len() > MAX_ELEMENTS_FOR_ANALYSIS {
             let limited_elements = &elements[..MAX_ELEMENTS_FOR_ANALYSIS];
             return self.analyze_limited_elements(limited_elements);
         }
         
         // Этап 1: Построение индекса узлов
         self.build_node_index(elements);
         
         // Этап 2: Построение графа связей элементов
         self.build_element_graph();
         
         // Этап 3: Поиск связных компонент
         self.find_components_bfs(elements)
     }
     
     /// Анализ ограниченного количества элементов
     fn analyze_limited_elements(&mut self, elements: &[LiraElement]) -> Vec<ConnectedGroup> {
         info!("Анализ ограниченного набора из {} элементов", elements.len());
         
         // Этап 1: Построение индекса узлов
         self.build_node_index(elements);
         
         // Этап 2: Построение графа связей элементов
         self.build_element_graph();
         
         // Этап 3: Поиск связных компонент
          let components = self.find_components_bfs(elements);
            
            // Найдено связных компонент
        components
    }

    /// Этап 1: Построение индекса "узел -> список элементов"
    fn build_node_index(&mut self, elements: &[LiraElement]) {
        // Построение индекса узлов
        
        self.node_to_elements.clear();
        
        for element in elements {
            for &node_id in &element.nodes {
                self.node_to_elements
                    .entry(node_id)
                    .or_insert_with(Vec::new)
                    .push(element.id);
            }
        }
        
        debug!("Индекс построен: {} узлов", self.node_to_elements.len());
        
        // Логируем статистику
        let max_elements_per_node = self.node_to_elements
            .values()
            .map(|elements| elements.len())
            .max()
            .unwrap_or(0);
        
        debug!("Максимум элементов на узел: {}", max_elements_per_node);
    }

    /// Этап 2: Построение графа связей между элементами
    /// Если два элемента имеют общий узел, они связаны
    fn build_element_graph(&mut self) {
        debug!("Построение графа связей элементов...");
        
        self.element_graph.clear();
        
        // Ограничиваем количество элементов для обработки больших моделей
        const MAX_ELEMENTS_PER_NODE: usize = 100;
        let mut skipped_nodes = 0;
        
        // Для каждого узла соединяем все его элементы между собой
        for (node_id, elements_at_node) in &self.node_to_elements {
            // Если у узла только один элемент, связей нет
            if elements_at_node.len() < 2 {
                continue;
            }
            
            // Пропускаем узлы с слишком большим количеством элементов
            if elements_at_node.len() > MAX_ELEMENTS_PER_NODE {
                skipped_nodes += 1;
                debug!("Пропущен узел {} с {} элементами (превышен лимит {})", 
                       node_id, elements_at_node.len(), MAX_ELEMENTS_PER_NODE);
                continue;
            }
            
            // Соединяем каждый элемент с каждым
            for &elem1 in elements_at_node {
                for &elem2 in elements_at_node {
                    if elem1 != elem2 {
                        self.element_graph
                            .entry(elem1)
                            .or_insert_with(HashSet::new)
                            .insert(elem2);
                    }
                }
            }
        }
        
        if skipped_nodes > 0 {
            debug!("Пропущено {} узлов с большим количеством элементов", skipped_nodes);
        }
        
        let total_connections: usize = self.element_graph
            .values()
            .map(|connections| connections.len())
            .sum();
        
        debug!("Граф построен: {} элементов, {} связей", 
               self.element_graph.len(), total_connections / 2); // Делим на 2, т.к. связи двунаправленные
    }

    /// Этап 3: Поиск связных компонент с помощью BFS
    fn find_components_bfs(&self, elements: &[LiraElement]) -> Vec<ConnectedGroup> {
        debug!("Поиск связных компонент с помощью BFS...");
        
        let mut visited = HashSet::new();
        let mut components = Vec::new();
        let mut component_id = 0;
        
        // Создаем множество всех элементов для обработки
        let all_element_ids: HashSet<u32> = elements.iter().map(|e| e.id).collect();
        
        for &element_id in &all_element_ids {
            if visited.contains(&element_id) {
                continue;
            }
            
            // Начинаем новую компоненту
            let mut component_elements = Vec::new();
            let mut queue = VecDeque::new();
            
            queue.push_back(element_id);
            visited.insert(element_id);
            
            // BFS для поиска всех связанных элементов
            while let Some(current_element) = queue.pop_front() {
                component_elements.push(current_element);
                
                // Добавляем всех соседей в очередь
                if let Some(neighbors) = self.element_graph.get(&current_element) {
                    for &neighbor in neighbors {
                        if !visited.contains(&neighbor) {
                            visited.insert(neighbor);
                            queue.push_back(neighbor);
                        }
                    }
                }
            }
            
            // Создаем группу и обновляем её статистику
            let mut group = ConnectedGroup::new(component_id, component_elements);
            group.update_statistics(elements);
            
            debug!("Компонента {}: {} элементов, тип={:?}", 
                   component_id, group.elements.len(), group.group_type);
            
            components.push(group);
            component_id += 1;
        }
        
        components
    }

    /// Анализирует связность конкретного элемента
    pub fn analyze_element_connectivity(&self, element_id: u32) -> ElementConnectivity {
        let direct_connections = self.element_graph
            .get(&element_id)
            .map(|connections| connections.len())
            .unwrap_or(0);
        
        let shared_nodes = self.get_shared_nodes(element_id);
        
        ElementConnectivity {
            element_id,
            direct_connections,
            shared_nodes: shared_nodes.len(),
            connection_strength: self.calculate_connection_strength(element_id),
        }
    }

    /// Получает узлы, которые элемент разделяет с другими элементами
    fn get_shared_nodes(&self, element_id: u32) -> Vec<u32> {
        let mut shared_nodes = Vec::new();
        
        for (&node_id, elements_at_node) in &self.node_to_elements {
            if elements_at_node.contains(&element_id) && elements_at_node.len() > 1 {
                shared_nodes.push(node_id);
            }
        }
        
        shared_nodes
    }

    /// Вычисляет "силу" связности элемента
    fn calculate_connection_strength(&self, element_id: u32) -> f32 {
        let direct_connections = self.element_graph
            .get(&element_id)
            .map(|connections| connections.len())
            .unwrap_or(0) as f32;
        
        let shared_nodes = self.get_shared_nodes(element_id).len() as f32;
        
        // Простая формула: среднее количество связей на узел
        if shared_nodes > 0.0 {
            direct_connections / shared_nodes
        } else {
            0.0
        }
    }

    /// Находит "мостовые" элементы - элементы, удаление которых разделит компоненту
    pub fn find_bridge_elements(&self, group: &ConnectedGroup) -> Vec<u32> {
        let mut bridges = Vec::new();
        
        for &element_id in &group.elements {
            if self.is_bridge_element(element_id, group) {
                bridges.push(element_id);
            }
        }
        
        bridges
    }

    /// Проверяет, является ли элемент мостовым
    fn is_bridge_element(&self, element_id: u32, group: &ConnectedGroup) -> bool {
        // Временно удаляем элемент из графа и проверяем связность
        let group_elements: HashSet<u32> = group.elements.iter().cloned().collect();
        let remaining_elements: HashSet<u32> = group_elements
            .iter()
            .filter(|&&id| id != element_id)
            .cloned()
            .collect();
        
        if remaining_elements.len() < 2 {
            return false; // Слишком мало элементов для проверки
        }
        
        // Проверяем, остается ли граф связным без этого элемента
        let components_count = self.count_components_in_subgraph(&remaining_elements);
        components_count > 1
    }

    /// Подсчитывает количество компонент в подграфе
    fn count_components_in_subgraph(&self, elements: &HashSet<u32>) -> usize {
        let mut visited = HashSet::new();
        let mut components = 0;
        
        for &element_id in elements {
            if visited.contains(&element_id) {
                continue;
            }
            
            // BFS для одной компоненты
            let mut queue = VecDeque::new();
            queue.push_back(element_id);
            visited.insert(element_id);
            
            while let Some(current) = queue.pop_front() {
                if let Some(neighbors) = self.element_graph.get(&current) {
                    for &neighbor in neighbors {
                        if elements.contains(&neighbor) && !visited.contains(&neighbor) {
                            visited.insert(neighbor);
                            queue.push_back(neighbor);
                        }
                    }
                }
            }
            
            components += 1;
        }
        
        components
    }
}

/// Информация о связности элемента
#[derive(Debug, Clone)]
pub struct ElementConnectivity {
    /// ID элемента
    pub element_id: u32,
    /// Количество прямых связей
    pub direct_connections: usize,
    /// Количество общих узлов
    pub shared_nodes: usize,
    /// Сила связности (0.0 - 1.0+)
    pub connection_strength: f32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::Vec3;

    fn create_test_elements() -> Vec<LiraElement> {
        vec![
            LiraElement {
                id: 1,
                element_type: ElementType::Shell,
                nodes: vec![10, 11, 12, 13],
                coordinates: vec![Vec3::ZERO; 4],
                properties: ElementProperties::default(),
            },
            LiraElement {
                id: 2,
                element_type: ElementType::Shell,
                nodes: vec![12, 13, 14, 15], // Общие узлы 12, 13 с элементом 1
                coordinates: vec![Vec3::ZERO; 4],
                properties: ElementProperties::default(),
            },
            LiraElement {
                id: 3,
                element_type: ElementType::Shell,
                nodes: vec![20, 21, 22, 23], // Изолированный элемент
                coordinates: vec![Vec3::ZERO; 4],
                properties: ElementProperties::default(),
            },
            LiraElement {
                id: 4,
                element_type: ElementType::Beam,
                nodes: vec![13, 16, 17, 18], // Общий узел 13 с элементами 1 и 2
                coordinates: vec![Vec3::ZERO; 4],
                properties: ElementProperties::default(),
            },
        ]
    }

    #[test]
    fn test_find_connected_components() {
        let mut analyzer = GraphAnalyzer::new();
        let elements = create_test_elements();
        
        let components = analyzer.find_connected_components(&elements);
        
        // Должно быть 2 компоненты: {1, 2, 4} и {3}
        assert_eq!(components.len(), 2);
        
        let large_component = components.iter().find(|c| c.elements.len() == 3).unwrap();
        let small_component = components.iter().find(|c| c.elements.len() == 1).unwrap();
        
        assert!(large_component.elements.contains(&1));
        assert!(large_component.elements.contains(&2));
        assert!(large_component.elements.contains(&4));
        
        assert!(small_component.elements.contains(&3));
    }

    #[test]
    fn test_build_node_index() {
        let mut analyzer = GraphAnalyzer::new();
        let elements = create_test_elements();
        
        analyzer.build_node_index(&elements);
        
        // Узел 13 должен быть связан с элементами 1, 2, 4
        let elements_at_node_13 = &analyzer.node_to_elements[&13];
        assert_eq!(elements_at_node_13.len(), 3);
        assert!(elements_at_node_13.contains(&1));
        assert!(elements_at_node_13.contains(&2));
        assert!(elements_at_node_13.contains(&4));
        
        // Узел 20 должен быть связан только с элементом 3
        let elements_at_node_20 = &analyzer.node_to_elements[&20];
        assert_eq!(elements_at_node_20.len(), 1);
        assert!(elements_at_node_20.contains(&3));
    }
}