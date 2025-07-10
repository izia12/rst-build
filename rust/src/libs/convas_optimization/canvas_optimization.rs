use std::collections::HashMap;
use serde::{Serialize};
use ordered_float::OrderedFloat;
use crate::libs::parse::{EntityWithXlsx, Vertex};

#[derive(Serialize)]
pub struct CanvasData {
    pub levels: Vec<LevelData>,
    pub total_shapes: usize,
    pub visible_levels: usize,
    pub total_levels: usize,
}

#[derive(Serialize)]
pub struct LevelData {
    pub z: f32,
    pub shapes: Vec<ShapeData>,
    pub shape_count: usize,
}

#[derive(Serialize)]
pub struct ShapeData {
    pub vertices: Vec<Vertex>,
    pub as_values: AsValues,
    pub entity_type: String,
}

#[derive(Serialize)]
pub struct AsValues {
    pub as1: Vec<f64>,
    pub as2: Vec<f64>,
    pub as3: Vec<f64>,
    pub as4: Vec<f64>,
}

#[derive(Serialize)]
pub struct CanvasStatistics {
    pub total_levels: usize,
    pub total_entities: usize,
    pub z_range: ZRange,
    pub levels_info: Vec<LevelInfo>,
}

#[derive(Serialize)]
pub struct ZRange {
    pub min: f32,
    pub max: f32,
}

#[derive(Serialize)]
pub struct LevelInfo {
    pub z: f32,
    pub entity_count: usize,
}

pub fn sort_by_same_z_optimized(data: Vec<EntityWithXlsx>) -> HashMap<OrderedFloat<f32>, Vec<EntityWithXlsx>> {
    let mut map: HashMap<OrderedFloat<f32>, Vec<EntityWithXlsx>> = HashMap::new();

    for item in data.into_iter() {
        if let Some(first_vertex) = item.vertices.first() {
            let z0 = first_vertex.z;
            // Проверяем, что все вершины имеют одинаковую Z координату
            if item.vertices.iter().all(|v| (v.z - z0).abs() < 0.001) {
                let z = OrderedFloat(z0 as f32);
                map.entry(z)
                    .or_insert_with(Vec::new)
                    .push(item);
            }
        }
    }
    map
}

pub fn get_optimized_canvas_data(
    data: Vec<EntityWithXlsx>,
    max_shapes_per_level: usize,
    max_total_shapes: usize,
    start_z: Option<f32>,
    end_z: Option<f32>
) -> CanvasData {
    // Сортируем данные по Z-уровням
    let sorted_data = sort_by_same_z_optimized(data);
    
    // Получаем отсортированные ключи Z
    let mut z_keys: Vec<_> = sorted_data.keys().collect();
    z_keys.sort();
    
    // Фильтруем по диапазону Z, если указан
    if let (Some(start), Some(end)) = (start_z, end_z) {
        z_keys.retain(|&&z| z.0 >= start && z.0 <= end);
    }
    
    let total_levels = sorted_data.len();
    let mut levels = Vec::new();
    let mut total_shapes = 0;
    let mut visible_levels = 0;
    
    for &z_key in &z_keys {
        if total_shapes >= max_total_shapes {
            break;
        }
        
        if let Some(entities) = sorted_data.get(z_key) {
            let mut shapes = Vec::new();
            let mut level_shape_count = 0;
            
            for entity in entities {
                if level_shape_count >= max_shapes_per_level || 
                   total_shapes >= max_total_shapes {
                    break;
                }
                
                if let Some(row) = &entity.row {
                    shapes.push(ShapeData {
                        vertices: entity.vertices.clone(),
                        as_values: AsValues {
                            as1: row.as1.clone(),
                            as2: row.as2.clone(),
                            as3: row.as3.clone(),
                            as4: row.as4.clone(),
                        },
                        entity_type: entity.entity_type.clone(),
                    });
                    
                    level_shape_count += 1;
                    total_shapes += 1;
                }
            }
            
            if !shapes.is_empty() {
                levels.push(LevelData {
                    z: z_key.0,
                    shapes,
                    shape_count: level_shape_count,
                });
                visible_levels += 1;
            }
        }
    }
    
    CanvasData {
        levels,
        total_shapes,
        visible_levels,
        total_levels,
    }
}

pub fn get_canvas_statistics(data: Vec<EntityWithXlsx>) -> CanvasStatistics {
    let sorted_data = sort_by_same_z_optimized(data);
    let total_levels = sorted_data.len();
    let total_entities = sorted_data.values().map(|v| v.len()).sum::<usize>();
    
    let z_range = if let (Some(min_z), Some(max_z)) = (
        sorted_data.keys().min(),
        sorted_data.keys().max()
    ) {
        ZRange {
            min: min_z.0,
            max: max_z.0,
        }
    } else {
        ZRange {
            min: 0.0,
            max: 0.0,
        }
    };
    
    let levels_info: Vec<LevelInfo> = sorted_data
        .iter()
        .map(|(z, entities)| LevelInfo {
            z: z.0,
            entity_count: entities.len(),
        })
        .collect();
    
    CanvasStatistics {
        total_levels,
        total_entities,
        z_range,
        levels_info,
    }
}