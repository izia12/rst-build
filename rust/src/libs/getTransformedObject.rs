use std::{cmp::Ordering, collections::HashMap};

use js_sys::{Array, Error, Map, Object, Reflect};
use ordered_float::OrderedFloat;
use wasm_bindgen::{ prelude::wasm_bindgen, JsValue};
use crate::GLOBAL_ENTITIES;

use super::parse::EntityWithXlsx;


#[wasm_bindgen]
pub fn get_horizontal_elements_object_js() -> JsValue {
    // Достаем данные из хранилища
    let entities = GLOBAL_ENTITIES.with(|cell| 
        cell.borrow()
            .as_ref()
            .expect("Данные не загружены!")
            .clone()
    );

    // 1. Группируем plates и собираем все z
    let mut plates_map:HashMap<OrderedFloat<f64>, Vec<&EntityWithXlsx>> = HashMap::new();
    let mut all_z = Vec::new();

    for item in &entities {
        let z = item.vertices[0].z;
        if item.vertices.iter().all(|v| v.z == z) {
            plates_map.entry(OrderedFloat(z)).or_insert_with(Vec::new).push(item);
            all_z.push(z);
        }
    }

    // 2. Сортируем z и удаляем дубликаты
    all_z.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    all_z.dedup();

    // 3. Группируем rods по ближайшему нижнему z
    let mut rods_map:HashMap<OrderedFloat<f64>, Vec<&EntityWithXlsx>> = HashMap::new();

    for item in &entities {
        let z = item.vertices[0].z;
        if plates_map.contains_key(&OrderedFloat(z)) {
            continue; // Пропускаем plates
        }

        // Ищем ближайший нижний z
        let lower_z = all_z.iter()
            .rev() // Идем от большего к меньшему
            .find(|&&cz| cz < z);

        if let Some(lower_z) = lower_z {
            rods_map.entry(OrderedFloat(*lower_z)).or_insert_with(Vec::new).push(item);
        }
    }

    // 4. Собираем результат в JS-объект
    let result = Object::new();

    for z in all_z {
        let level_obj = Object::new();
        let z_key = JsValue::from(z.to_string());

        // Добавляем plates
        if let Some(plates) = plates_map.get(&OrderedFloat(z)) {
            let js_plates = Array::new();
            for plate in plates {
                js_plates.push(&entity_to_js(plate));
            }
            Reflect::set(&level_obj, &"plates".into(), &js_plates).unwrap();
        }

        // Добавляем rods
        if let Some(rods) = rods_map.get(&OrderedFloat(z)) {
            let js_rods = Array::new();
            for rod in rods {
                js_rods.push(&entity_to_js(rod));
            }
            Reflect::set(&level_obj, &"rods".into(), &js_rods).unwrap();
        }

        Reflect::set(&result, &z_key, &level_obj).unwrap();
    }

    result.into()
}

// Преобразование EntityWithXlsx в JS-объект без сериализации
fn entity_to_js(entity: &EntityWithXlsx) -> JsValue {
    let obj = Object::new();

    // Вершины
    let vertices = Array::new();
    for v in &entity.vertices {
        let vertex_obj = Object::new();
        Reflect::set(&vertex_obj, &"x".into(), &v.x.into()).unwrap();
        Reflect::set(&vertex_obj, &"y".into(), &v.y.into()).unwrap();
        Reflect::set(&vertex_obj, &"z".into(), &v.z.into()).unwrap();
        vertices.push(&vertex_obj);
    }
    Reflect::set(&obj, &"vertices".into(), &vertices).unwrap();

    // RowData (если есть)
    if let Some(row) = &entity.row {
        let row_obj = Object::new();
        
        // Для каждого массива as1/as2/as3/as4:
        let convert_to_js_array = |data: &[f64]| {
            let arr = Array::new();
            for &value in data {
                arr.push(&JsValue::from_f64(value));
            }
            arr
        };

        Reflect::set(&row_obj, &"as1".into(), &convert_to_js_array(&row.as1)).unwrap();
        Reflect::set(&row_obj, &"as2".into(), &convert_to_js_array(&row.as2)).unwrap();
        Reflect::set(&row_obj, &"as3".into(), &convert_to_js_array(&row.as3)).unwrap();
        Reflect::set(&row_obj, &"as4".into(), &convert_to_js_array(&row.as4)).unwrap();
        
        Reflect::set(&obj, &"rowData".into(), &row_obj).unwrap();
    }

    obj.into()
}
// fn convert_entity_to_js(entity: &EntityWithXlsx) -> Result<JsValue, JsError> {
//     let obj = Object::new();
//     Reflect::set(&obj, &"id".into(), &JsValue::from(&entity.id)).unwrap();
//     // Добавьте остальные поля...
//     Ok(obj.into())
// }