
use std::{collections::HashMap, fs, path::Path, u32};
use dxf::Drawing;
// use image::io::Reader;
use serde::{Serialize, Deserialize};
use std::io::Cursor;
use xml::EventReader;
use xml::reader::XmlEvent;
use calamine::{ Data, Reader as clamineReader};
use serde_json;
// use web_sys::console;


use crate::{libs::lira_parse::LiraFile};
#[derive(Serialize,Deserialize, Debug, Clone)]
pub struct Vertex {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct SerializableEntity {
    pub entity_type: String,
    pub vertices: Vec<Vertex>,
    pub handle: String,
    pub layer: String,
    pub color_id: i32,
    pub node_id: usize,
    pub material_num: Option<usize>,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RowData {
    pub id: usize,
    pub as1: Vec<f64>,
    pub as2: Vec<f64>,
    pub as3: Vec<f64>,
    pub as4: Vec<f64>,
    pub asw1: Vec<f64>,
    pub asw2: Vec<f64>,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Material {
    pub material_num: Option<usize>,
    pub sg_type: Option<String>,
    pub b_or_d: Option<f64>,
    pub h_or_d: Option<f64>,
    pub h: Option<f64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EntityWithXlsx {
    pub entity_type: String,
    pub vertices: Vec<Vertex>,
    pub row: Option<RowData>,
    pub changed: bool,
    pub material: Option<Material>,
}
 impl EntityWithXlsx {
    pub fn get_value(&self, field: &str) -> Option<Vec<f64>> {
        match field {
            "as1" => Some(self.row.clone().unwrap().as1),
            "as2" => Some(self.row.clone().unwrap().as2),
            "as3" => Some(self.row.clone().unwrap().as3),
            "as4" => Some(self.row.clone().unwrap().as4),
            "asw1" => Some(self.row.clone().unwrap().asw1),
            "asw2" => Some(self.row.clone().unwrap().asw2),
            _ => None, // если ключ не найден
        }
    }
}
// #[wasm_bindgen]
// pub fn dxf_to_json(dxf_data: &str) -> String {
//     let mut cursor = Cursor::new(dxf_data);
//     let drawing = Drawing::load(&mut cursor).expect("Failed to parse DXF data");

//     // Collecting all entities
//     let entities: Vec<SerializableEntity> = drawing.entities().filter_map(|entity| {
//         match entity.specific {
//             // Handle LINE entities
//             dxf::entities::EntityType::Line(ref line) => Some(SerializableEntity {
//                 entity_type: "LINE".to_string(),
//                 vertices: vec![
//                     Vertex {
//                         x: line.p1.x,
//                         y: line.p1.y,
//                         z: line.p1.z,
//                     },
//                     Vertex {
//                         x: line.p2.x,
//                         y: line.p2.y,
//                         z: line.p2.z,
//                     },
//                 ],
//                 handle: entity.common.handle.clone().as_string(),
//                 layer: entity.common.layer.clone(),
//                 color_id: 0,
//                 node_id: 0,
//             }),
//             // Handle 3DFACE entities
//             dxf::entities::EntityType::Face3D(ref face3d) => Some(SerializableEntity {
//                 entity_type: "3DFACE".to_string(),
//                 vertices: vec![
//                     Vertex {
//                         x: face3d.first_corner.x,
//                         y: face3d.first_corner.y,
//                         z: face3d.first_corner.z,
//                     },
//                     Vertex {
//                         x: face3d.second_corner.x,
//                         y: face3d.second_corner.y,
//                         z: face3d.second_corner.z,
//                     },
//                     Vertex {
//                         x: face3d.third_corner.x,
//                         y: face3d.third_corner.y,
//                         z: face3d.third_corner.z,
//                     },
//                     Vertex {
//                         x: face3d.fourth_corner.x,
//                         y: face3d.fourth_corner.y,
//                         z: face3d.fourth_corner.z,
//                     },
//                 ],
//                 handle: entity.common.handle.clone().as_string(),
//                 layer: entity.common.layer.clone(),
//                 color_id: 0,
//                 node_id: 0,
//             }),
//             _ => None, // Ignore other types of entities
//         }
//     }).collect();
//     // Convert the entities into a JSON string
//     serde_json::to_string(&entities).expect("Failed to serialize to JSON")

// }

// #[wasm_bindgen]
pub fn sli_to_json(data: &str, tolerance: Option<f64>) -> String {
    let tolerance = tolerance.unwrap_or(0.005);
    let cursor = Cursor::new(data);
    let parser = EventReader::new(cursor);
    let mut points: Vec<Vertex> = Vec::new();
    let mut entities: Vec<SerializableEntity> = Vec::new();
    let mut planes: Vec<f64> = vec![];
    let mut node_id = 0;
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, attributes,  ..}) => {
                match name.local_name.as_str() {
                    "NodeCoords" => {
                        let vertices: Vertex = Vertex{
                            x: attributes.iter().find(|attr| attr.name.local_name == "NdX").unwrap().value.parse::<f64>().unwrap(),
                            y: attributes.iter().find(|attr| attr.name.local_name == "NdY").unwrap().value.parse::<f64>().unwrap(),
                            z: attributes.iter().find(|attr| attr.name.local_name == "NdZ").unwrap().value.parse::<f64>().unwrap(),
                        };
                        points.push(vertices)
                    },
                    "Element" => {
                        node_id += 1;
                        let entity_type = match attributes.iter().find(|attr| attr.name.local_name == "Type").unwrap().value.as_str() {
                            "1" => String::from("LINE"),
                            "2" => String::from("3DFACE"),
                            _ => String::from("UNKNOWN"),
                        };
                        let material_num = attributes.iter().find(|attr| attr.name.local_name == "Material").unwrap().value.parse::<usize>().unwrap();
                        let entity = SerializableEntity{
                            entity_type,
                            vertices: vec![],
                            handle: "".to_string(),
                            layer: "".to_string(),
                            color_id: 0,
                            node_id,
							material_num:Some(material_num),
                        };
                        entities.push(
                            entity
                        )
                    },
                    "Nodes" => {
                        let node_indexes = attributes.iter().map(|attr| attr.value.parse::<usize>().unwrap()).collect::<Vec<usize>>();
                        if let Some(entity) = entities.iter_mut().last() {
                            for index in node_indexes {
                                if let Some(vertex) = points.get(index - 1) {
                                    entity.vertices.push(vertex.clone());
                                }
                            }
                            if is_in_same_plane(&entity.vertices, tolerance) {
                                if let Some(vertex) = entity.vertices.first() {
                                    let plane = (vertex.z / tolerance).round() * tolerance;
                                    if planes.contains(&plane){
                                        if let Some(color_id) = planes.iter().position(|&p| p == plane){
                                            entity.color_id = color_id as i32 + 1;
                                        }
                                    }else {
                                        planes.push(plane);
                                        entity.color_id = planes.len() as i32;
                                    }
                                }

                            }
                            entity.entity_type = match entity.vertices.len() as i32 {
                                2 => String::from("LINE"),
                                3 => String::from("3DFACE_TRIANGLE"),
                                _ => String::from("3DFACE"),
                            };
                        }
                    }
                    _ => {}
                }
            }
            Ok(XmlEvent::EndElement {..}) => {}
            Err(e) => {
                eprintln!("Error: {e}");
                break;
            }
            _ => {}
        }
    }
    let mut colored_entities: HashMap<i32, HashMap<&String, Vec<f64>>> = HashMap::new();
    for entity in entities.iter_mut() {
        let mut coords: Vec<f64> = entity.vertices.iter().flat_map(|v| vec![v.x, v.y, v.z]).collect();
        colored_entities.entry(entity.color_id).or_insert(HashMap::new()).entry(&entity.entity_type).or_insert(vec![]).append(&mut coords);
    }
    serde_json::to_string(&colored_entities).expect("Failed to serialize to JSON")
}

fn is_in_same_plane(points: &Vec<Vertex>, tolerance: f64) -> bool {
    if let Some(first) = points.first(){
        points.iter().all(|point| point.z - first.z <= tolerance)
    }else {
        false
    }
}

pub fn parse_xlsx_wasm(data: &[u8]) -> Vec<RowData> {
	// string_log_data(&serde_json::to_string(&data).unwrap());
    // match parse_xlsx_from_bytes(data) {
    //     Ok(parsed) => {
    //         // console::log_1(&format!("✅ Успешный парсинг: {} записей", parsed.len()).into());
    //         parsed
    //     }
    //     Err(err) => {
    //         // console::log_1(&format!("❌ Ошибка парсинга: {}", err).into());
    //         Vec::new()
    //     }
    // }
	parse_xlsx_from_bytes(data).unwrap()
}

fn parse_xlsx_from_bytes(data: &[u8]) -> Result<Vec<RowData>, String> {
    let cursor = std::io::Cursor::new(data);
    let mut workbook = calamine::open_workbook_auto_from_rs(cursor)
        .map_err(|e| format!("Ошибка загрузки: {}", e))?;

    let sheet_names = workbook.sheet_names().to_vec();
    let mut results = Vec::new();
    // !!! Тут вводились изменения из за разного формата таблиц и их расширения(xlsx xls)
    for sheet_name in sheet_names {
        let range = workbook.worksheet_range(&sheet_name).map_err(|e| e.to_string())?;
        // Собираем все строки в вектор для доступа к соседним строкам
        let rows: Vec<&[Data]> = range.rows().collect();
        let mut current_row: Option<RowData> = None;
        
        for (row_index, row) in rows.iter().enumerate() {
            // Обработка ID
            let id = match row.get(0) {
                Some(Data::Float(f)) => *f as usize,
                Some(Data::Int(i)) => *i as usize,
                Some(Data::String(s)) => s.parse().unwrap_or(0),
                _ => {
                    continue;
                }
            };
            // Если нашли новый ID - сохраняем предыдущий ряд
            if let Some(prev) = current_row.take() {
                results.push(prev);
            }

            // Парсим значения столбцов с учетом ячейки ниже
            let asw1_values = parse_column_with_below(&rows, row_index, 5);
            let asw2_values = parse_column_with_below(&rows, row_index, 6);
            // Логируем результаты для asw1 и asw2
            current_row = Some(RowData {
                id,
                as1:  parse_column_with_below(&rows, row_index, 1),
                as2:  parse_column_with_below(&rows, row_index, 2),
                as3:  parse_column_with_below(&rows, row_index, 3),
                as4:  parse_column_with_below(&rows, row_index, 4),
                asw1: asw1_values,
                asw2: asw2_values,
            });
        }

        if let Some(last) = current_row.take() {
            results.push(last);
        }
    }

    Ok(results)
}

// Вспомогательная функция для парсинга столбцов
// fn parse_column(row: &[Data], index: usize) -> Vec<f64> {
//     row.get(index).map_or_else(
//         || vec![0.0],
//         |cell| match cell {
//             Data::Float(f) => vec![*f],
//             Data::Int(i) => vec![*i as f64],
//             Data::String(s) => s.split(',')
//                 .filter_map(|part| part.trim().parse().ok())
//                 .collect(),
//             _ => vec![0.0]
//         }
//     )
// }

// Функция для парсинга столбцов с учетом ячейки ниже
fn parse_column_with_below(rows: &[&[Data]], row_index: usize, col_index: usize) -> Vec<f64> {
    let mut result = Vec::new();
    
    // Получаем значение из текущей ячейки
    if let Some(row) = rows.get(row_index) {
        if let Some(cell) = row.get(col_index) {
            match cell {
                Data::Float(f) => {
                    result.push(*f);
                },
                Data::Int(i) => {
                    result.push(*i as f64);
                },
				Data::String(s) => {
					// ЗАМЕНА ТОЛЬКО ЭТОГО БЛОКА!
					let normalized = s.replace(',', ".");
					match normalized.parse::<f64>() {
						Ok(val) => {
							result.push(val);
						}
						Err(_) => {
							// Резервный вариант для нескольких значений
							for part in normalized.split(',') {
								if let Ok(val) = part.trim().parse() {
									result.push(val);
								}
							}
						}
					}
				},
                _ => {
                    result.push(0.0);
                }
            }
        }
    }
    
    // Получаем значение из ячейки ниже
    if let Some(row) = rows.get(row_index + 1) {
        if let Some(cell) = row.get(col_index) {
            match cell {
                Data::Float(f) => {
                    result.push(*f);
                },
                Data::Int(i) => {
                    result.push(*i as f64);
                },
				Data::String(s) => {
					// ЗАМЕНА ТОЛЬКО ЭТОГО БЛОКА!
					let normalized = s.replace(',', ".");
					match normalized.parse::<f64>() {
						Ok(val) => {
							result.push(val);
						}
						Err(_) => {
							// Резервный вариант для нескольких значений
							for part in normalized.split(',') {
								if let Ok(val) = part.trim().parse() {
									result.push(val);
								}
							}
						}
					}
				},
                _ => result.push(0.0)
            }
        }
    }
    
    if result.is_empty() {
        vec![0.0]
    } else {
        result
    }
}
pub fn get_indexes(sli_data: &str, txt_data: &str) ->(Vec<SerializableEntity>, HashMap<usize, Material>) {
    // Парсим TXT файл
    let mut lira_file = LiraFile::new();
    lira_file.parse_file_from_string(txt_data);
    let txt_elements = lira_file.get_elements();
    
    // Создаем хэш-мапу для TXT элементов для быстрого поиска по координатам
    let mut txt_elements_map: HashMap<String, usize> = HashMap::new();
    
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
    
    // Парсим SLI файл
    let cursor = Cursor::new(sli_data);
    let parser = EventReader::new(cursor);
    let mut points: Vec<Vertex> = Vec::new();
    let mut entities: Vec<SerializableEntity> = Vec::new();
    let mut materials: HashMap<usize, Material> = HashMap::new();
    let mut in_materials_array = false;
    let mut current_material_num: Option<usize> = None;
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, attributes,  ..}) => {
                match name.local_name.as_str() {
                    "NodeCoords" => {
                        let vertices: Vertex = Vertex{
                            x: attributes.iter().find(|attr| attr.name.local_name == "NdX").unwrap().value.parse::<f64>().unwrap(),
                            y: attributes.iter().find(|attr| attr.name.local_name == "NdY").unwrap().value.parse::<f64>().unwrap(),
                            z: attributes.iter().find(|attr| attr.name.local_name == "NdZ").unwrap().value.parse::<f64>().unwrap(),
                        };
                        points.push(vertices)
                    },
                    "Element" => {
                        let entity_type = match attributes.iter().find(|attr| attr.name.local_name == "Type").unwrap().value.as_str() {
                            "1" => String::from("LINE"),
                            "2" => String::from("3DFACE"),
                            _ => String::from("UNKNOWN"),
                        };
                        let material_num = attributes.iter()
                            .find(|attr| attr.name.local_name == "Material")
                            .and_then(|attr| attr.value.parse::<usize>().ok());
                        // Создаем сущность с временным node_id, который будет заменен позже
                        let entity = SerializableEntity{
                            entity_type,
                            vertices: vec![],
                            handle: "".to_string(),
                            layer: "".to_string(),
                            color_id: 0,
                            node_id: 0, // Временное значение, будет заменено на node_id из TXT файла
                            material_num,
                        };
                        entities.push(
                            entity
                        )
                    },

                    "Nodes" => {
                        let node_indexes = attributes.iter().map(|attr| attr.value.parse::<usize>().unwrap()).collect::<Vec<usize>>();
                        if let Some(entity) = entities.iter_mut().last() {
                            for index in node_indexes {
                                if let Some(vertex) = points.get(index - 1) {
                                    entity.vertices.push(vertex.clone());
                                }
                            }
                            entity.entity_type = match entity.vertices.len() as i32 {
                                2 => String::from("LINE"),
                                3 => String::from("3DFACE_TRIANGLE"),
                                4 => String::from("3DFACE"),
                                _ => String::from("UNKNOWN"),
                            };
                        }
                    }
					"MaterialsArray" => {
                        in_materials_array = true;
                    },
                    "Material" => {
                        let num = attributes.iter().find(|attr| attr.name.local_name == "Num").unwrap().value.parse::<usize>().unwrap();
                        let h = attributes.iter().find(|attr| attr.name.local_name == "H").unwrap().value.parse::<f64>().unwrap();
                        current_material_num = Some(num);
                        materials.insert(num, Material {
                            material_num: Some(num),
                            sg_type: None,
                            b_or_d: None,
                            h_or_d: None,
                            h:Some(h),
                        });
                    },
                    "SectGeom" if current_material_num.is_some() => {
                        let sg_type = attributes.iter()
                            .find(|attr| attr.name.local_name == "SGType")
                            .map(|attr| attr.value.clone());
                        let b_or_d = attributes.iter()
                            .find(|attr| attr.name.local_name == "b_OR_D")
                            .and_then(|attr| attr.value.parse::<f64>().ok());
                        let h_or_d = attributes.iter()
                            .find(|attr| attr.name.local_name == "h_OR_d")
                            .and_then(|attr| attr.value.parse::<f64>().ok());
                        if let Some(num) = current_material_num {
                            if let Some(material) = materials.get_mut(&num) {
                                material.sg_type = sg_type;
                                material.b_or_d = b_or_d;
                                material.h_or_d = h_or_d;
                            }
                        }
                    },
                    _ => {}
                }
            }
			Ok(XmlEvent::EndElement { name }) => {},
            Err(e) => {
                eprintln!("Error: {e}");
                break;
            },
            _ => {}
        }
    }
    // Сопоставляем элементы из SLI и TXT файлов по координатам
    // и устанавливаем node_id из TXT файла
    let mut sli_elements_map: HashMap<String, usize> = HashMap::new();
    // Заполняем хэш-мапу для SLI элементов
    for (i, entity) in entities.iter().enumerate() {
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
    // Устанавливаем node_id из TXT файла для соответствующих элементов SLI
    for (key, txt_index) in &txt_elements_map {
        if let Some(&sli_index) = sli_elements_map.get(key) {
            // Устанавливаем node_id из TXT файла
            entities[sli_index].node_id = txt_index + 1; // +1 потому что индексы в TXT файле начинаются с 1
        }
    }
	(entities, materials)
}

// #[wasm_bindgen]
pub fn convert_sli_xsl_to_json(sli_data: &str, txt_data: &str, xls_data: &[u8]) -> Vec<EntityWithXlsx>{
    let (entities, materials) = get_indexes(sli_data, txt_data);
    let xlsx = parse_xlsx_wasm(xls_data);
    let mut entities_with_xlsx: Vec<EntityWithXlsx> = Vec::new();
	for (_, entity) in entities.iter().enumerate(){
		if let Some(row) = xlsx.iter().find(|row_item| row_item.id == entity.node_id) {
			let material = entity.material_num.and_then(|num| materials.get(&num).cloned());
			entities_with_xlsx.push(EntityWithXlsx{
				entity_type: entity.entity_type.clone(),
				vertices: entity.vertices.clone(),
				row: Some(row.clone()),
				changed:false,
				material,
			})
		}else{
			let material = entity.material_num.and_then(|num| materials.get(&num).cloned());
			entities_with_xlsx.push(
				EntityWithXlsx{
					entity_type:"hello".to_string(),
					vertices:entity.vertices.clone(),
					row:Some(RowData { id: (4294967295), as1: vec!(0.0,0.0), as2: vec!(0.0,0.0), as3: vec!(0.0,0.0), as4: vec!(0.0,0.0), asw1:vec!(0.0,0.0), asw2:vec!(0.0,0.0) }),
					changed: false,
                    material,
				}
			);
		};
	}
    entities_with_xlsx
    // serde_json::to_string(&entities_with_xlsx).expect("Failed to serialize to JSON")
}
// pub fn get_data_from_xlsx_sli(path_sli:&Path, path_xlsx:&Path)->Vec<EntityWithXlsx>{
// 	let vec_sli_data = fs::read(path_sli).unwrap();
// 	let sli_data = std::str::from_utf8(&vec_sli_data).unwrap();
// 	let xlsx_data = fs::read(path_xlsx).unwrap();
// 	convert_sli_xsl_to_json(sli_data, &xlsx_data)
// }
