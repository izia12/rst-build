use std::collections::{HashMap, hash_map::DefaultHasher};
use std::hash::{Hash, Hasher};
use crate::{libs::parse::EntityWithXlsx, string_log_two_params};
use ordered_float::OrderedFloat;

impl EntityWithXlsx {
    fn shape_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        // Сортируем вершины по координатам перед хешированием
        let mut sorted_vertices: Vec<_> = self.vertices.iter()
            .map(|v| (OrderedFloat(v.x), OrderedFloat(v.y)))
            .collect();
        sorted_vertices.sort_by(|a, b| a.partial_cmp(b).unwrap());

        for (x, y) in sorted_vertices {
            x.hash(&mut hasher);
            y.hash(&mut hasher);
        }
        hasher.finish()
    }
}

pub fn unification_data(
    planes: Vec<f32>,
    data: HashMap<OrderedFloat<f32>, Vec<EntityWithXlsx>>,
    group_name: &str
) -> HashMap<OrderedFloat<f32>, Vec<EntityWithXlsx>> {
    let mut result = HashMap::new();
    let mut shape_map: HashMap<u64, Vec<&EntityWithXlsx>> = HashMap::new();
    
    // Собираем все сущности с одинаковой геометрией, но только для указанных planes
    for (z, entities) in data.iter() {
        if !planes.contains(z) { continue; }
        for entity in entities {
            shape_map.entry(entity.shape_hash()).or_default().push(entity);
        }
    }

    // Обработка всех z-уровней из исходных данных
    for (plane_z, entities) in data.iter() {
        if planes.contains(plane_z) {
            // Для z-уровней из planes выполняем унификацию
            let mut processed = Vec::new();
            for entity in entities {
                if let Some(group) = shape_map.get(&entity.shape_hash()) {
                    // Вычисление абсолютных максимумов
                    let (max_as1, max_as2, max_as3, max_as4) = group.iter()
                        .filter_map(|e| e.row.as_ref())
                        .fold((f64::MIN, f64::MIN, f64::MIN, f64::MIN),
                            |(a1, a2, a3, a4), row| (
                                row.as1.first().copied().unwrap_or(f64::MIN).max(a1),
                                row.as2.first().copied().unwrap_or(f64::MIN).max(a2),
                                row.as3.first().copied().unwrap_or(f64::MIN).max(a3),
                                row.as4.first().copied().unwrap_or(f64::MIN).max(a4)
                            ));

                    // Логирование исходных значений ДО изменений
                   

                    // Создаем обновленные сущности для всей группы
                    // string_log_two_params("----After Change --------", &String::from(group_name));
                    for e in group {
                        let mut new_entity = (*e).clone();
                        new_entity.changed = true;
                        if let Some(row) = &mut new_entity.row {
                            row.as1 = vec![max_as1.max(0.0)];
                            row.as2 = vec![max_as2.max(0.0)];
                            row.as3 = vec![max_as3.max(0.0)];
                            row.as4 = vec![max_as4.max(0.0)];
                        }
                        processed.push(new_entity);
                    }
                    
                }
            }
            result.insert(*plane_z, processed);
        } else {
            // Для остальных z-уровней просто копируем данные без изменений
            result.insert(*plane_z, entities.clone());
        }
    }
    result
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::libs::parse::{EntityWithXlsx, Vertex, RowData};
//     use std::collections::HashMap;
//     use ordered_float::OrderedFloat;

//     fn create_test_entity(x: f64, y: f64, z: f64, as1: f64) -> EntityWithXlsx {
//         EntityWithXlsx {
//             entity_type: "TEST".to_string(),
//             vertices: vec![
//                 Vertex { x, y, z },
//                 Vertex { x, y, z }, // Дублируем вершину для проверки
//             ],
//             row: Some(RowData {
//                 id: 1,
//                 as1: vec![as1],
//                 as2: vec![0.0],
//                 as3: vec![0.0],
//                 as4: vec![0.0],
//             }),
//         }
//     }

//     #[test]
//     fn test_unification_data() {
//         // Создаем тестовые данные
//         let mut data = HashMap::new();
        
//         // Добавляем сущности с одинаковыми x,y но разными as1
//         let entities_z1 = vec![
//             create_test_entity(1.0, 1.0, 1.0, 10.0),
//             create_test_entity(1.0, 1.0, 1.0, 20.0),
//             create_test_entity(2.0, 2.0, 1.0, 30.0),
//         ];
//         data.insert(OrderedFloat(1.0), entities_z1);

//         // Добавляем сущности с разными x,y
//         let entities_z2 = vec![
//             create_test_entity(3.0, 3.0, 2.0, 40.0),
//             create_test_entity(4.0, 4.0, 2.0, 50.0),
//         ];
//         data.insert(OrderedFloat(2.0), entities_z2);

//         // Тестируем функцию
//         let planes = vec![1.0, 2.0];
//         let result = unification_data(planes, data, "test_group");
//         // Проверяем результаты
//         // Для z = 1.0
//         if let Some(entities) = result.get(&OrderedFloat(1.0)) {
//             // Проверяем, что сущности с x=1,y=1 имеют одинаковое максимальное as1
//             let matching_entities: Vec<_> = entities.iter()
//                 .filter(|e| e.vertices[0].x == 1.0 && e.vertices[0].y == 1.0)
//                 .collect();
//             assert_eq!(matching_entities.len(), 2);
//             assert!(matching_entities.iter().all(|e| 
//                 e.row.as_ref().unwrap().as1[0] == 20.0
//             ));
//         }

//         // Для z = 2.0
//         if let Some(entities) = result.get(&OrderedFloat(2.0)) {
//             // Проверяем, что сущности не изменились, так как у них разные x,y
//             assert_eq!(entities.len(), 2);
//             assert!(entities.iter().any(|e| e.row.as_ref().unwrap().as1[0] == 40.0));
//             assert!(entities.iter().any(|e| e.row.as_ref().unwrap().as1[0] == 50.0));
//         }
//     }
// }
