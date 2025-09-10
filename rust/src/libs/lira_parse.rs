use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

// use crate::libs::parse::{get_indexes, SerializableEntity1};
use rst_build::libs::{
    parse::{get_indexes, SerializableEntity1},
    types::{Element, LiraFile},
};
// Пример использования
pub fn parse_lira_file(file_path: &str) -> io::Result<LiraFile> {
    let mut lira_file = LiraFile::new();
    lira_file.parse_file(file_path)?;
    Ok(lira_file)
}
pub fn main() {
    let _ = verify_element_indices("src/cafe_muslim04.txt", "src/cafe_muslim04.sli");
    // let lira_file = parse_lira_file("src/cafe_muslim04.txt").unwrap();

    // // Создаем CSV файл
    // let file_path = "lira_elements.csv";
    // let mut file = match std::fs::File::create(file_path) {
    //     Ok(file) => file,
    //     Err(e) => {
    //         eprintln!("Ошибка при создании файла: {}", e);
    //         return;
    //     }
    // };

    // // Записываем заголовок CSV
    // if let Err(e) = writeln!(file, "Номер,Индекс координаты,Координаты X,Координаты Y,Координаты Z,Тип,Материал") {
    //     eprintln!("Ошибка при записи заголовка: {}", e);
    //     return;
    // }

    // // Получение всех элементов
    // let elements = lira_file.get_elements();
    // println!("Всего элементов: {}", elements.len());

    // for (i, element) in elements.iter().enumerate() {
    //     if element.coordinates.is_empty() {
    //         // Если координат нет, записываем строку с нулевыми координатами
    //         if let Err(e) = writeln!(file, "{},0,0.0,0.0,0.0,{},{}",
    //                                i, element.element_type, element.material) {
    //             eprintln!("Ошибка при записи элемента {}: {}", i, e);
    //         }
    //     } else {
    //         // Записываем каждую координату элемента в отдельной строке
    //         for (j, coord) in element.coordinates.iter().enumerate() {
    //             if let Err(e) = writeln!(file, "{},{},{},{},{},{},{}",
    //                                    i, j, coord.x, coord.y, coord.z,
    //                                    element.element_type, element.material) {
    //                 eprintln!("Ошибка при записи элемента {} координаты {}: {}", i, j, e);
    //                 continue;
    //             }
    //         }
    //     }
    // }

    // println!("CSV файл успешно создан: {}", file_path);
}
// Добавьте эту функцию в файл lira_parse.rs

pub fn verify_element_indices(txt_file_path: &str, sli_file_path: &str) -> io::Result<()> {
    // Парсим TXT файл
    let lira_file = parse_lira_file(txt_file_path)?;
    let txt_elements = lira_file.get_elements();

    // Парсим SLI файл
    let sli_content = fs::read_to_string(sli_file_path)?;
    let txt_content = fs::read_to_string(txt_file_path)?;
    let (sli_entities, _) = get_indexes(&sli_content, &txt_content);

    println!("Количество элементов в TXT: {}", txt_elements.len());
    println!("Количество элементов в SLI: {}", sli_entities.len());

    // Создаем хэш-мапы для быстрого поиска элементов по координатам
    let mut txt_elements_map: HashMap<String, usize> = HashMap::new();
    let mut sli_elements_map: HashMap<String, usize> = HashMap::new();

    // Заполняем хэш-мапу для TXT элементов
    for (i, element) in txt_elements.iter().enumerate() {
        // Создаем ключ из отсортированных координат
        let mut coords_set: Vec<(f64, f64, f64)> = element
            .coordinates
            .iter()
            .map(|c| (c.x, c.y, c.z))
            .collect();
        coords_set.sort_by(|a, b| {
            a.0.partial_cmp(&b.0)
                .unwrap()
                .then(a.1.partial_cmp(&b.1).unwrap())
                .then(a.2.partial_cmp(&b.2).unwrap())
        });
        let key = coords_set
            .iter()
            .map(|(x, y, z)| format!("{:.5},{:.5},{:.5}", x, y, z))
            .collect::<Vec<String>>()
            .join("|");
        txt_elements_map.insert(key, i);
    }
    // Заполняем хэш-мапу для SLI элементов
    for (i, entity) in sli_entities.iter().enumerate() {
        // Создаем ключ из отсортированных координат
        let mut coords_set: Vec<(f64, f64, f64)> =
            entity.vertices.iter().map(|v| (v.x, v.y, v.z)).collect();
        coords_set.sort_by(|a, b| {
            a.0.partial_cmp(&b.0)
                .unwrap()
                .then(a.1.partial_cmp(&b.1).unwrap())
                .then(a.2.partial_cmp(&b.2).unwrap())
        });
        let key = coords_set
            .iter()
            .map(|(x, y, z)| format!("{:.5},{:.5},{:.5}", x, y, z))
            .collect::<Vec<String>>()
            .join("|");
        sli_elements_map.insert(key, i);
    }

    // Создаем имя CSV файла на основе имен входных файлов
    let txt_filename = Path::new(txt_file_path)
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap();
    let sli_filename = Path::new(sli_file_path)
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap();
    let csv_filename = format!("{}_vs_{}_mismatches.csv", txt_filename, sli_filename);

    // Создаем CSV файл
    let mut csv_file = File::create(&csv_filename)?;

    // Записываем заголовок CSV
    writeln!(
        csv_file,
        "Тип несоответствия,TXT индекс,SLI индекс,Элемент из txt,Элемент из sli"
    )?;
    // Добавляем пустую строку после заголовков
    writeln!(csv_file, ",,,,")?;

    // Проверяем соответствие индексов
    let mut mismatches = 0;
    let mut total_checked = 0;
    let mut txt_only = 0;
    let mut sli_only = 0;

    for (key, txt_index) in &txt_elements_map {
        if let Some(sli_index) = sli_elements_map.get(key) {
            total_checked += 1;
            if txt_index != sli_index {
                mismatches += 1;
                println!(
                    "Несоответствие индексов: TXT={}, SLI={}, координаты={}",
                    txt_index, sli_index, key
                );
                // Получаем координаты элементов в виде строк
                let txt_coords = format_element_coords(&txt_elements[*txt_index]);
                let sli_coords = format_entity_coords(&sli_entities[*sli_index]);
                // Записываем в CSV
                writeln!(
                    csv_file,
                    "Несоответствие индексов,{},{},{},{}",
                    txt_index, sli_index, txt_coords, sli_coords
                )?;
            }
        } else {
            txt_only += 1;
            println!(
                "Элемент с координатами {} найден в TXT (индекс {}), но отсутствует в SLI",
                key, txt_index
            );
            // Получаем координаты элемента в виде строки
            let txt_coords = format_element_coords(&txt_elements[*txt_index]);
            // Записываем в CSV
            writeln!(csv_file, "Только в TXT,{},N/A,{},", txt_index, txt_coords)?;
        }
    }
    // Проверяем элементы, которые есть в SLI, но отсутствуют в TXT
    for (key, sli_index) in &sli_elements_map {
        if !txt_elements_map.contains_key(key) {
            sli_only += 1;
            println!(
                "Элемент с координатами {} найден в SLI (индекс {}), но отсутствует в TXT",
                key, sli_index
            );
            // Получаем координаты элемента в виде строки
            let sli_coords = format_entity_coords(&sli_entities[*sli_index]);
            // Записываем в CSV
            writeln!(csv_file, "Только в SLI,N/A,{},,{}", sli_index, sli_coords)?;
        }
    }
    // Записываем статистику в CSV
    writeln!(csv_file, "\nСтатистика:,,")?;
    writeln!(csv_file, "Всего элементов в TXT,{},", txt_elements.len())?;
    writeln!(csv_file, "Всего элементов в SLI,{},", sli_entities.len())?;
    writeln!(csv_file, "Проверено элементов,{},", total_checked)?;
    writeln!(csv_file, "Несоответствий индексов,{},", mismatches)?;
    writeln!(csv_file, "Только в TXT,{},", txt_only)?;
    writeln!(csv_file, "Только в SLI,{},", sli_only)?;
    // println!("Проверка завершена. Проверено элементов: {}", total_checked);
    // println!("Найдено несоответствий индексов: {}", mismatches);
    // println!("Элементов только в TXT: {}", txt_only);
    // println!("Элементов только в SLI: {}", sli_only);
    if mismatches == 0 && txt_only == 0 && sli_only == 0 {
        println!("Все элементы имеют одинаковые индексы в обоих файлах!");
    } else {
        println!("Обнаружены проблемы с индексами элементов!");
    }
    println!("CSV файл с результатами создан: {}", csv_filename);
    Ok(())
}

// Функция для форматирования координат элемента из TXT файла
fn format_element_coords(element: &Element) -> String {
    element
        .coordinates
        .iter()
        .map(|c| format!("{:.5} | {:.5} | {:.5}", c.x, c.y, c.z))
        .collect::<Vec<String>>()
        .join("      ")
}

// fn format_entity_coords(entity:  &SerializableEntity1) -> String {

fn format_entity_coords(entity: &SerializableEntity1) -> String {
    entity
        .vertices
        .iter()
        .map(|v| format!("{:.5} | {:.5} | {:.5}", v.x, v.y, v.z))
        .collect::<Vec<String>>()
        .join("      ")
}
