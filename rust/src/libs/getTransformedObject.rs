use std:: collections::{HashMap, HashSet};
use js_sys::{Array, Object, Reflect};
use ordered_float::OrderedFloat;
use wasm_bindgen::{ prelude::wasm_bindgen, JsValue};
use crate::{ GLOBAL_ENTITIES};
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
    // 4. Собираем результат в JS-объект
    let result = Object::new();
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

	fn max_f64(a: f64, b: f64) -> f64 {
		// Обрабатываем NaN и другие особые случаи
		if a.is_nan() {
			return b;
		}
		if b.is_nan() {
			return a;
		}
		if a >= b { a } else { b }
	}

    for z in all_z {
        let level_obj = Object::new();
        let z_key = JsValue::from(z.to_string());
		let mut current_z_material:HashSet<usize> = HashSet::new();
        // Добавляем plates
        if let Some(plates) = plates_map.get(&OrderedFloat(z)) {
			
            let js_plates = Array::new();
			let mut max_as1 = f64::NEG_INFINITY;
			let mut max_as2 = f64::NEG_INFINITY;
			let mut max_as3 = f64::NEG_INFINITY;
			let mut max_as4 = f64::NEG_INFINITY;
			let mut max_asw1 = f64::NEG_INFINITY;
			let mut max_asw2 = f64::NEG_INFINITY;

		for plate in plates.iter() {
		    if let Some(row) = &plate.row {
		        // Безопасное получение значений (используем 0.0 если элемента нет)
		        let as1_0 = *row.as1.get(0).unwrap_or(&0.0);
		        let as1_1 = *row.as1.get(1).unwrap_or(&0.0);
		        let as2_0 = *row.as2.get(0).unwrap_or(&0.0);
		        let as2_1 = *row.as2.get(1).unwrap_or(&0.0);
		        let as3_0 = *row.as3.get(0).unwrap_or(&0.0);
		        let as3_1 = *row.as3.get(1).unwrap_or(&0.0);
		        let as4_0 = *row.as4.get(0).unwrap_or(&0.0);
		        let as4_1 = *row.as4.get(1).unwrap_or(&0.0);
		        let asw1_0 = *row.asw1.get(0).unwrap_or(&0.0);
		        let asw1_1 = *row.asw1.get(1).unwrap_or(&0.0);
		        let asw2_0 = *row.asw2.get(0).unwrap_or(&0.0);
		        let asw2_1 = *row.asw2.get(1).unwrap_or(&0.0);
		        // Вычисляем текущие максимумы
		        let current_as1 = if as1_0 > as1_1 { as1_0 } else { as1_1 };
		        let current_as2 = if as2_0 > as2_1 { as2_0 } else { as2_1 };
		        let current_as3 = if as3_0 > as3_1 { as3_0 } else { as3_1 };
		        let current_as4 = if as4_0 > as4_1 { as4_0 } else { as4_1 };
		        let current_asw1 = if asw1_0 > asw1_1 { asw1_0 } else { asw1_1 };
		        let current_asw2 = if asw2_0 > asw2_1 { asw2_0 } else { asw2_1 };
		        
		        // Обновляем общие максимумы
		        if current_as1 > max_as1 { max_as1 = current_as1; }
		        if current_as2 > max_as2 { max_as2 = current_as2; }
		        if current_as3 > max_as3 { max_as3 = current_as3; }
		        if current_as4 > max_as4 { max_as4 = current_as4; }
		        if current_asw1 > max_asw1 { max_asw1 = current_asw1; }
		        if current_asw2 > max_asw2 { max_asw2 = current_asw2; }
		    }
		}

			// Заменяем невалидные значения на 0.0
			if max_as1 == f64::NEG_INFINITY { max_as1 = 0.0; }
			if max_as2 == f64::NEG_INFINITY { max_as2 = 0.0; }
			if max_as3 == f64::NEG_INFINITY { max_as3 = 0.0; }
			if max_as4 == f64::NEG_INFINITY { max_as4 = 0.0; }
			if max_asw1 == f64::NEG_INFINITY { max_asw1 = 0.0; }
			if max_asw2 == f64::NEG_INFINITY { max_asw2 = 0.0; }

				// Устанавливаем значения
			Reflect::set(&level_obj, &"maxAs1".into(), &JsValue::from_f64(max_as1)).unwrap_or_default();
			Reflect::set(&level_obj, &"maxAs2".into(), &JsValue::from_f64(max_as2)).unwrap_or_default();
			Reflect::set(&level_obj, &"maxAs3".into(), &JsValue::from_f64(max_as3)).unwrap_or_default();
			Reflect::set(&level_obj, &"maxAs4".into(), &JsValue::from_f64(max_as4)).unwrap_or_default();
			Reflect::set(&level_obj, &JsValue::from_str("maxAsw1"), &JsValue::from_f64(max_asw1)).unwrap();
            Reflect::set(&level_obj, &JsValue::from_str("maxAsw2"), &JsValue::from_f64(max_asw2)).unwrap();
            for plate in plates {
                js_plates.push(&entity_to_js(plate));
				current_z_material.insert(plate.material.clone().unwrap().material_num.unwrap());
            }
            Reflect::set(&level_obj, &JsValue::from_str("maxAs1"), &JsValue::from_f64(max_as1)).unwrap();
            Reflect::set(&level_obj, &JsValue::from_str("maxAs2"), &JsValue::from_f64(max_as2)).unwrap();
            Reflect::set(&level_obj, &JsValue::from_str("maxAs3"), &JsValue::from_f64(max_as3)).unwrap();
            Reflect::set(&level_obj, &JsValue::from_str("maxAs4"), &JsValue::from_f64(max_as4)).unwrap();
            Reflect::set(&level_obj, &JsValue::from_str("maxAsw1"), &JsValue::from_f64(max_asw1)).unwrap();
            Reflect::set(&level_obj, &JsValue::from_str("maxAsw2"), &JsValue::from_f64(max_asw2)).unwrap();
            Reflect::set(&level_obj, &"plates".into(), &js_plates).unwrap();
        }

        // Добавляем rods
        if let Some(rods) = rods_map.get(&OrderedFloat(z)) {
            let js_rods = Array::new();
            for rod in rods {
                js_rods.push(&entity_to_js(rod));
				current_z_material.insert(rod.material.clone().unwrap().material_num.unwrap());
            }
            Reflect::set(&level_obj, &"rods".into(), &js_rods).unwrap();
        }
		 let materials_arr = Array::new();
		 for material in current_z_material{
			materials_arr.push(&JsValue::from(material));
		 }
		Reflect::set(&level_obj, &JsValue::from("Materials"), &JsValue::from(materials_arr)).unwrap();
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
        Reflect::set(&row_obj, &"asw1".into(), &convert_to_js_array(&row.asw1)).unwrap();
        Reflect::set(&row_obj, &"asw2".into(), &convert_to_js_array(&row.asw2)).unwrap();
        Reflect::set(&obj, &"rowData".into(), &row_obj).unwrap();
    }

    obj.into()
}
