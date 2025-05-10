use std::fs::File;
use std::{collections::HashMap, fs, path::Path};
use dxf::{Drawing};
// use image::io::Reader;
use quick_xml::name::{LocalName, QName};
use serde::{Serialize, Deserialize};
use std::io::{BufRead, BufReader, Cursor};
use xml::EventReader;
use xml::reader::XmlEvent;
use wasm_bindgen::prelude::*;
use calamine::{ Data, DataType, Xlsx, Reader as clamineReader};
use serde_json;
use quick_xml::Reader as QuickXMLReader;
use quick_xml::events::{Event, BytesStart};
use js_sys::{Array, Error, Map, Object, Reflect, JSON};

use crate::string_log_two_params;

// use web_sys::console;

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
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RowData {
    pub id: usize,
    pub as1: Vec<f64>,
    pub as2: Vec<f64>,
    pub as3: Vec<f64>,
    pub as4: Vec<f64>,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EntityWithXlsx {
    pub entity_type: String,
    pub vertices: Vec<Vertex>,
    pub row: Option<RowData >,
	pub changed:bool
}
 impl EntityWithXlsx {
    pub fn get_value(&self, field: &str) -> Option<Vec<f64>> {
        match field {
            "as1" => Some(self.row.clone().unwrap().as1),
            "as2" => Some(self.row.clone().unwrap().as2),
            "as3" => Some(self.row.clone().unwrap().as3),
            "as4" => Some(self.row.clone().unwrap().as4),
            _ => None, // если ключ не найден
        }
    }
}
// #[wasm_bindgen]
pub fn dxf_to_json(dxf_data: &str) -> String {
    let mut cursor = Cursor::new(dxf_data);
    let drawing = Drawing::load(&mut cursor).expect("Failed to parse DXF data");

    // Collecting all entities
    let entities: Vec<SerializableEntity> = drawing.entities().filter_map(|entity| {
        match entity.specific {
            // Handle LINE entities
            dxf::entities::EntityType::Line(ref line) => Some(SerializableEntity {
                entity_type: "LINE".to_string(),
                vertices: vec![
                    Vertex {
                        x: line.p1.x,
                        y: line.p1.y,
                        z: line.p1.z,
                    },
                    Vertex {
                        x: line.p2.x,
                        y: line.p2.y,
                        z: line.p2.z,
                    },
                ],
                handle: entity.common.handle.clone().as_string(),
                layer: entity.common.layer.clone(),
                color_id: 0,
                node_id: 0,
            }),
            // Handle 3DFACE entities
            dxf::entities::EntityType::Face3D(ref face3d) => Some(SerializableEntity {
                entity_type: "3DFACE".to_string(),
                vertices: vec![
                    Vertex {
                        x: face3d.first_corner.x,
                        y: face3d.first_corner.y,
                        z: face3d.first_corner.z,
                    },
                    Vertex {
                        x: face3d.second_corner.x,
                        y: face3d.second_corner.y,
                        z: face3d.second_corner.z,
                    },
                    Vertex {
                        x: face3d.third_corner.x,
                        y: face3d.third_corner.y,
                        z: face3d.third_corner.z,
                    },
                    Vertex {
                        x: face3d.fourth_corner.x,
                        y: face3d.fourth_corner.y,
                        z: face3d.fourth_corner.z,
                    },
                ],
                handle: entity.common.handle.clone().as_string(),
                layer: entity.common.layer.clone(),
                color_id: 0,
                node_id: 0,
            }),
            _ => None, // Ignore other types of entities
        }
    }).collect();
    // Convert the entities into a JSON string
    serde_json::to_string(&entities).expect("Failed to serialize to JSON")

}

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
                        let entity = SerializableEntity{
                            entity_type,
                            vertices: vec![],
                            handle: "".to_string(),
                            layer: "".to_string(),
                            color_id: 0,
                            node_id,
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

    for sheet_name in sheet_names {
        let range = workbook.worksheet_range(&sheet_name).map_err(|e| e.to_string())?;
        // string_log_two_params(&sheet_name, &String::from("Название листа"));

        let mut current_row: Option<RowData> = None;
        
        for (row_idx, row) in range.rows().enumerate() {
            // Логируем всю строку для отладки
            let row_debug: Vec<String> = row.iter().enumerate().map(|(i, cell)| {
                format!("Col{}: {}", i+1, cell.to_string())
            }).collect();
            // string_log_two_params(&row_debug.join(" | "), &format!("Строка {}", row_idx + 1));

            // Обработка ID
            let id = match row.get(0) {
                Some(Data::Float(f)) => *f as usize,
                Some(Data::Int(i)) => *i as usize,
                Some(Data::String(s)) => s.parse().unwrap_or(0),
                _ => {
                    // string_log_two_params("", &String::from("Пропуск строки с невалидным ID"));
                    continue;
                }
            };

            // Если нашли новый ID - сохраняем предыдущий ряд
            if let Some(prev) = current_row.take() {
                results.push(prev);
            }

            // Парсим значения столбцов
            current_row = Some(RowData {
                id,
                as1: parse_column(&row, 1),
                as2: parse_column(&row, 2),
                as3: parse_column(&row, 3),
                as4: parse_column(&row, 4),
            });
        }

        if let Some(last) = current_row.take() {
            results.push(last);
        }
    }

    Ok(results)
}

// Вспомогательная функция для парсинга столбцов
fn parse_column(row: &[Data], index: usize) -> Vec<f64> {
    row.get(index).map_or_else(
        || vec![0.0],
        |cell| match cell {
            Data::Float(f) => vec![*f],
            Data::Int(i) => vec![*i as f64],
            Data::String(s) => s.split(',')
                .filter_map(|part| part.trim().parse().ok())
                .collect(),
            _ => vec![0.0]
        }
    )
}
pub fn get_indexes(data: &str) -> Vec<SerializableEntity> {
    let cursor = Cursor::new(data);
    let parser = EventReader::new(cursor);
    let mut points: Vec<Vertex> = Vec::new();
    let mut entities: Vec<SerializableEntity> = Vec::new();
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
                        let entity = SerializableEntity{
                            entity_type,
                            vertices: vec![],
                            handle: "".to_string(),
                            layer: "".to_string(),
                            color_id: 0,
                            node_id,

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
    entities
}


// pub fn get_entity_by_index(entities: Vec<SerializableEntity>, index: usize) -> Option<&SerializableEntity> {
//     entities.get(index - 1)
// }
//////////////////////////////////////////quickXML/////////////////////////////
// pub fn get_indexes1(data: &str) -> Vec<SerializableEntity> {
//     let mut reader = QuickXMLReader::from_str(data);
//     // let mut reader = BufReader::new(data);
//     // reader.trim_text(true); // Очистка текста от лишних пробелов, если это нужно

//     let mut points: Vec<Vertex> = Vec::new();
//     let mut entities: Vec<SerializableEntity> = Vec::new();
//     let mut node_id = 0;
//     let mut missed_elements: Vec<String> = Vec::new();
//     let mut buf = Vec::new();

//     loop {
//         match reader.read_event_into(&mut buf) {
//             Ok(Event::Start(ref e)) => {
//                 match e.name() {
//                     QName(b"NodeCoords") => {
//                         let mut x = 0.0;
//                         let mut y = 0.0;
//                         let mut z = 0.0;
//                         for attr in e.attributes() {
//                             match attr {
//                                 Ok(attr) => {
//                                     match attr.key {
//                                         QName(b"NdX") => x = attr.decode_and_unescape_value(reader.decoder()).unwrap().parse::<f64>().unwrap(),
//                                         QName(b"NdY") => y = attr.decode_and_unescape_value(reader.decoder()).unwrap().parse::<f64>().unwrap(),
//                                         QName(b"NdZ") => z = attr.decode_and_unescape_value(reader.decoder()).unwrap().parse::<f64>().unwrap(),
//                                         _ => {},
//                                     }
//                                 },
//                                 Err(_) => {}
//                             }
//                         }

//                         let vertex = Vertex { x, y, z };
//                         points.push(vertex);
//                     },
//                     QName(b"Element") => {
//                         node_id += 1;
//                         let mut entity_type = String::from("UNKNOWN");

//                         for attr in e.attributes() {
//                             match attr {
//                                 Ok(attr) => {
//                                     if attr.key == QName(b"Type") {
//                                         entity_type = match attr.decode_and_unescape_value(reader.decoder()).unwrap().to_string().as_str() {
//                                             "1" => String::from("LINE"),
//                                             "2" => String::from("3DFACE"),
//                                             _ => String::from("UNKNOWN"),
//                                         };
//                                     }
//                                 },
//                                 Err(_) => {}
//                             }
//                         }

//                         let entity = SerializableEntity {
//                             entity_type,
//                             vertices: vec![],
//                             handle: "".to_string(),
//                             layer: "".to_string(),
//                             color_id: 0,
//                             node_id,
//                         };

//                         entities.push(entity);
//                     },
//                     QName(b"Nodes") => {
//                         let mut node_indexes: Vec<usize> = Vec::new();

//                         for attr in e.attributes() {
//                             match attr {
//                                 Ok(attr) => {
//                                     node_indexes.push(attr.decode_and_unescape_value(reader.decoder()).unwrap().parse::<usize>().unwrap());
//                                 },
//                                 Err(_) => {}
//                             }
//                         }

//                         if let Some(entity) = entities.last_mut() {
//                             for index in node_indexes {
//                                 if let Some(vertex) = points.get(index - 1) {
//                                     entity.vertices.push(vertex.clone());
//                                 } else {
//                                     missed_elements.push(format!("Node index {} not found", index));
//                                 }
//                             }

//                             // Определяем тип на основе количества вершин
//                             entity.entity_type = match entity.vertices.len() {
//                                 2 => String::from("LINE"),
//                                 3 => String::from("3DFACE_TRIANGLE"),
//                                 4 => String::from("3DFACE"),
//                                 _ => String::from("UNKNOWN"),
//                             };
//                         }
//                     },
//                     _ => {}
//                 }
//             },
//             Ok(Event::Eof) => break,
//             Err(_) => break,
//             _ => {}
//         }

//         buf.clear();
//     }

//     // Логирование пропущенных элементов
//     if !missed_elements.is_empty() {
//         for element in missed_elements {
//             eprintln!("Missed element: {}", element);
//         }
//     }

//     entities
// }


// #[derive(Debug)]
// struct NodeCoords {
//     x: f64,
//     y: f64,
//     z: f64,
// }

// #[derive(Debug)]
// struct ElementNodes {
//     nd1: String,
//     nd2: String,
//     nd3: String,
//     nd4: String,
// }
//  fn parse_node_coords(e: &BytesStart) -> Result<Vertex, Box<dyn std::error::Error>> {
//     let mut x = None;
//     let mut y = None;
//     let mut z = None;

//     for attr in e.attributes() {
//         let attr = attr?;
//         match attr.key.as_ref() {
//             b"NdX" => x = Some(attr.unescape_value()?.parse()?),
//             b"NdY" => y = Some(attr.unescape_value()?.parse()?),
//             b"NdZ" => z = Some(attr.unescape_value()?.parse()?),
//             _ => (),
//         }
//     }

//     Ok(Vertex {
//         x: x.unwrap_or_default(),
//         y: y.unwrap_or_default(),
//         z: z.unwrap_or_default(),
//     })
// }

// fn parse_element<B: BufRead>(
//     reader: &mut QuickXMLReader<B>,
// ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
//     let mut nd1 = None;
//     let mut nd2 = None;
//     let mut nd3 = None;
//     let mut nd4 = None;
//     let mut buf = Vec::new();
// 	let mut number_of_vertices = Vec::new();
//     loop {
//         match reader.read_event_into(&mut buf)? {
//             Event::Empty(e) | Event::Start(e) => {
//                 if e.name().as_ref() == b"Nodes" {
//                     for attr in e.attributes() {
//                         let attr = attr?;
//                         match attr.key.as_ref() {
//                             b"Nd1" => nd1 = Some(attr.unescape_value()?.to_string()),
//                             b"Nd2" => nd2 = Some(attr.unescape_value()?.to_string()),
//                             b"Nd3" => nd3 = Some(attr.unescape_value()?.to_string()),
//                             b"Nd4" => nd4 = Some(attr.unescape_value()?.to_string()),
//                             _ => (),
//                         }
//                     }
//                     break;
//                 }
//             }
//             Event::End(e) if e.name().as_ref() == b"Element" => break,
//             _ => (),
//         }
//         buf.clear();
//     }
// 	number_of_vertices.push(nd1.unwrap_or_default());
// 	number_of_vertices.push(nd2.unwrap_or_default());
// 	number_of_vertices.push(nd3.unwrap_or_default());
// 	number_of_vertices.push(nd4.unwrap_or_default());
//     Ok(number_of_vertices)
// }
// pub fn parse_xml(data: &str) -> Result<(Vec<Vertex>, Vec<ElementNodes>), Box<dyn std::error::Error>> {
//     let mut xml_reader = QuickXMLReader::from_reader(Cursor::new(data.as_bytes()));
//     xml_reader.config_mut().trim_text(true);

//     let mut nodes = Vec::new();
//     let mut elements = Vec::new();
//     let mut buf = Vec::new();
// 	let mut entities: Vec<SerializableEntity> = Vec::new();
// 	let mut node_id=0;
//     loop {
//         match xml_reader.read_event_into(&mut buf)? {
//             Event::Start(e) | Event::Empty(e) => match e.name().as_ref() {
//                 b"NodeCoords" => {
//                     let coords = parse_node_coords(&e)?;
//                     nodes.push(coords);
//                 }
//                 b"Element" => {
//                     let nodes_data = parse_element(&mut xml_reader)?;
// 					node_id+=1;
//                     elements.push(nodes_data);
// 					let vertex = nodes.get(12);
// 					for v in nodes_data{
						
// 					}
// 					let entity = SerializableEntity{
// 						color_id:0,
// 						node_id,
// 						vertices:"",
// 						entity_type:"3DFACE",
// 						handle:"".to_lowercase(),
// 						layer:"3DFACE_TRIANGLE".to_uppercase(),
// 					}
//                 }
//                 _ => (),
//             },
//             Event::Eof => break,
//             _ => (),
//         }
//         buf.clear();
//     }

//     Ok((nodes, elements))
// }


// #[wasm_bindgen]
pub fn convert_sli_xsl_to_json(sli_data: &str, data: &[u8]) -> Vec<EntityWithXlsx>{
    let entities = get_indexes(sli_data);
    let xlsx = parse_xlsx_wasm(data);
    let mut entities_with_xlsx: Vec<EntityWithXlsx> = Vec::new();
	for (index, entity) in entities.iter().enumerate(){
		if let Some(row) = xlsx.iter().find(|row_item| row_item.id == entity.node_id) {
			entities_with_xlsx.push(EntityWithXlsx{
				entity_type: entity.entity_type.clone(),
				vertices: entity.vertices.clone(),
				row: Some(row.clone()),
				changed:false
			})
		}else{
			entities_with_xlsx.push(
				EntityWithXlsx{
					entity_type:"hello".to_string(),
					vertices:entity.vertices.clone(),
					row:Some(RowData { id: (4294967295), as1: vec!(0.0,0.0), as2: vec!(0.0,0.0), as3: vec!(0.0,0.0), as4: vec!(0.0,0.0) }),
					changed:false
				}
			);
		};
	}
    entities_with_xlsx
    // serde_json::to_string(&entities_with_xlsx).expect("Failed to serialize to JSON")
}
pub fn get_data_from_xlsx_sli(path_sli:&Path, path_xlsx:&Path)->Vec<EntityWithXlsx>{
	let vec_sli_data = fs::read(path_sli).unwrap();
	let sli_data = std::str::from_utf8(&vec_sli_data).unwrap();
	let xlsx_data = fs::read(path_xlsx).unwrap();
	convert_sli_xsl_to_json(sli_data, &xlsx_data)
}

// #[wasm_bindgen]
// pub fn convert_sli_xsl_to_json_string(data:String)->String{
// 	// let data = convert_sli_xsl_to_json(sli_data,data);
// 	data
// 	// serde_json::to_string(&data).expect("Failed to serialize to JSON")
// }

// Новая функция для обработки столбцов
// fn parse_column(row: &[Data], index: usize) -> Vec<f64> {
//     let mut result = Vec::new();
//     if let Some(cell) = row.get(index) {
//         match cell {
//             Data::Float(f) => result.push(*f),
//             Data::Int(i) => result.push(*i as f64),
//             Data::String(s) => {
//                 if let Ok(num) = s.replace(',', ".").parse::<f64>() {
//                     result.push(num);
//                 } else {
//                     string_log_two_params(s, &format!("Ошибка конвертации в столбце {}", index + 1));
//                 }
//             }
//             _ => {
//                 string_log_two_params(&format!("{:?}", cell), &format!("Неизвестный формат в столбце {}", index + 1));
//             }
//         }
//     }
//     if result.is_empty() {
//         string_log_two_params("", &format!("Пустые данные в столбце {}", index + 1));
//     }
//     result
// }