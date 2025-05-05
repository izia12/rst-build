use std::collections::HashMap;
use crate::{libs::parse::EntityWithXlsx, string_log_two_params};
use ordered_float::OrderedFloat;

pub fn unification_data(
    planes: Vec<f32>,
    data: HashMap<OrderedFloat<f32>, Vec<EntityWithXlsx>>,
    group_name: &str
) -> HashMap<OrderedFloat<f32>, Vec<EntityWithXlsx>> {
	string_log_two_params(&serde_json::to_string(&planes.len()).expect("f"), &String::from("Это длина"));

    let mut result = HashMap::new();

    // Проходим по всем z значениям из planes
    for plane_z in planes {
        let plane_z_ordered = OrderedFloat(plane_z);
		string_log_two_params(&serde_json::to_string(&plane_z_ordered).expect("f"), &String::from("Это этажи z"));

        // Ищем соответствующую группу сущностей
        if let Some(entities) = data.get(&plane_z_ordered) {
			string_log_two_params(&serde_json::to_string(&entities.len()).expect("f"), &String::from("Это значения хешмапа"));
			string_log_two_params(&serde_json::to_string(&plane_z_ordered).expect("f"), &String::from("Это это привязанное ключ z к каждому значению"));
            let mut processed_entities = Vec::new();
            // Группируем сущности по совпадающим x,y координатам
            let mut xy_groups: HashMap<(OrderedFloat<f32>, OrderedFloat<f32>), Vec<&EntityWithXlsx>> = HashMap::new();
            for entity in entities {
                // Проверяем, что все вершины имеют одинаковые x,y
                if let Some(first_vertex) = entity.vertices.first() {
					let xy = (OrderedFloat(first_vertex.x as f32), OrderedFloat(first_vertex.y as f32));
					string_log_two_params(&serde_json::to_string(&first_vertex.x).expect("f"), &String::from("Это первое x"));
					string_log_two_params(&serde_json::to_string(&first_vertex.y).expect("f"), &String::from("Это первое y"));
                    // Проверяем, что все остальные вершины имеют те же x,y
                    if entity.vertices.iter().all(|v| 
                        OrderedFloat(v.x as f32) == xy.0 && 
                        OrderedFloat(v.y as f32) == xy.1
                    ) {
						string_log_two_params("", &String::from("Все значения которые соответстуют x y"));

                        xy_groups.entry(xy)
                            .or_insert_with(Vec::new)
                            .push(entity);
                    }
                }
            }
			string_log_two_params(&serde_json::to_string(&xy_groups.len()).expect("f"), &String::from("Это найденные xy длина"));

            // Для каждой группы с одинаковыми x,y находим максимальное as1
            for (_, group) in xy_groups {
                if group.len() > 1 {
                    // Находим максимальное значение as1 в группе
                    let max_as1 = group.iter()
                        .filter_map(|e| e.row.as_ref().map(|r| 
                            r.as1.iter()
                                .map(|&x| OrderedFloat(x))
                                .max()
                                .map(|x| x.into_inner())
                                .unwrap_or(0.0)
                        ))
                        .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
                        .unwrap_or(0.0);
                    // Создаем новые сущности с обновленными значениями as1
					string_log_two_params(&serde_json::to_string(&max_as1).expect("f"), &String::from("Это максимум as1"));

                    let mut new_group = Vec::new();
                    for entity in group {
                        let mut new_entity = entity.clone();
                        if let Some(row) = &mut new_entity.row {
                            row.as1 = vec![max_as1];
                        }
                        new_group.push(new_entity);
                    }
                    processed_entities.extend(new_group);
                } else {
					string_log_two_params("", &String::from(" Если в группе только одна сущность, добавляем её без изменений"));

                    // Если в группе только одна сущность, добавляем её без изменений
                    processed_entities.extend(group.into_iter().cloned());
                }
            }
            result.insert(plane_z_ordered, processed_entities);
        }else {
			string_log_two_params("", &String::from("Никаких сущностей не найдено"));

		}
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::libs::parse::{EntityWithXlsx, Vertex, RowData};
    use std::collections::HashMap;
    use ordered_float::OrderedFloat;

    fn create_test_entity(x: f64, y: f64, z: f64, as1: f64) -> EntityWithXlsx {
        EntityWithXlsx {
            entity_type: "TEST".to_string(),
            vertices: vec![
                Vertex { x, y, z },
                Vertex { x, y, z }, // Дублируем вершину для проверки
            ],
            row: Some(RowData {
                id: 1,
                as1: vec![as1],
                as2: vec![0.0],
                as3: vec![0.0],
                as4: vec![0.0],
            }),
        }
    }

    #[test]
    fn test_unification_data() {
        // Создаем тестовые данные
        let mut data = HashMap::new();
        
        // Добавляем сущности с одинаковыми x,y но разными as1
        let entities_z1 = vec![
            create_test_entity(1.0, 1.0, 1.0, 10.0),
            create_test_entity(1.0, 1.0, 1.0, 20.0),
            create_test_entity(2.0, 2.0, 1.0, 30.0),
        ];
        data.insert(OrderedFloat(1.0), entities_z1);

        // Добавляем сущности с разными x,y
        let entities_z2 = vec![
            create_test_entity(3.0, 3.0, 2.0, 40.0),
            create_test_entity(4.0, 4.0, 2.0, 50.0),
        ];
        data.insert(OrderedFloat(2.0), entities_z2);

        // Тестируем функцию
        let planes = vec![1.0, 2.0];
        let result = unification_data(planes, data, "test_group");

        // Проверяем результаты
        // Для z = 1.0
        if let Some(entities) = result.get(&OrderedFloat(1.0)) {
            // Проверяем, что сущности с x=1,y=1 имеют одинаковое максимальное as1
            let matching_entities: Vec<_> = entities.iter()
                .filter(|e| e.vertices[0].x == 1.0 && e.vertices[0].y == 1.0)
                .collect();
            
            assert_eq!(matching_entities.len(), 2);
            assert!(matching_entities.iter().all(|e| 
                e.row.as_ref().unwrap().as1[0] == 20.0
            ));
        }

        // Для z = 2.0
        if let Some(entities) = result.get(&OrderedFloat(2.0)) {
            // Проверяем, что сущности не изменились, так как у них разные x,y
            assert_eq!(entities.len(), 2);
            assert!(entities.iter().any(|e| e.row.as_ref().unwrap().as1[0] == 40.0));
            assert!(entities.iter().any(|e| e.row.as_ref().unwrap().as1[0] == 50.0));
        }
    }
}