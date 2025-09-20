use clap::{Arg, Command};
use log::{info, error};
use std::path::Path;
use std::fs;
use std::env;
use anyhow::Result;

mod lir_parser;
mod graph_analyzer;
mod visualizer;
mod docx_generator;
mod types;
mod test_shapes;

use crate::lir_parser::LirParser;
use crate::graph_analyzer::GraphAnalyzer;
use crate::visualizer::Visualizer;
use crate::docx_generator::DocxGenerator;

fn main() -> Result<()> {
    env_logger::init();
    
    let args: Vec<String> = env::args().collect();
    
    // Проверяем команду тестирования
    if args.len() > 1 && (args[1] == "--test" || args[1] == "-t") {
        return test_shapes::run_test().map_err(|e| anyhow::anyhow!("Ошибка тестирования: {}", e));
    }
    
    let matches = Command::new("LIRA Analyzer")
        .version("0.1.0")
        .author("Structural Analysis Team")
        .about("Анализирует файлы LIRA для поиска связных компонент оболочек")
        .arg(
            Arg::new("input")
                .help("Путь к .lir файлу")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .help("Папка для выходных файлов")
                .default_value("output"),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Подробный вывод")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let input_file = matches.get_one::<String>("input").unwrap();
    let output_dir = matches.get_one::<String>("output").unwrap();
    let verbose = matches.get_flag("verbose");

    // Убрано: verbose вывод

    // Создаем выходную папку если её нет
    std::fs::create_dir_all(output_dir)?;

    // Этап 1: Парсинг файла
    // info!("Парсинг файла: {}", input_file);
    let mut parser = LirParser::new()?;
    
    // Определяем тип файла по расширению
    let elements = if input_file.ends_with(".sli") {
        parser.parse_sli_file(input_file)?
    } else {
        parser.parse_file(input_file)?
    };

    // Этап 2: Анализ связности
    // info!("Поиск связных компонент...");
    let mut analyzer = GraphAnalyzer::new();
    let groups = analyzer.find_connected_components(&elements);
    
    // Фильтруем только значимые группы (>1 элемента)
    let significant_groups: Vec<_> = groups.into_iter()
        .filter(|g| g.elements.len() > 1)
        .collect();
    
    // Ограничиваем количество групп для быстрой работы
    const MAX_GROUPS_TO_VISUALIZE: usize = 20;
    let groups: Vec<_> = significant_groups.into_iter().take(MAX_GROUPS_TO_VISUALIZE).collect();

    // Этап 3: Визуализация
    let mut visualizer = Visualizer::new();
    let output_path = Path::new(&output_dir);
    
    // Создаем изображения по этажам
    let mut image_paths = visualizer.create_floor_images(&elements, output_path)?;
    
    // Создаем изображения по группам
    let group_images = visualizer.generate_images(&elements, &groups)?;
    
    // Сохраняем изображения групп
    for (i, image_data) in group_images.iter().enumerate() {
        let filename = if i == 0 {
            "general_view.png".to_string()
        } else {
            format!("group_{}.png", i)
        };
        let path = output_path.join(&filename);
        std::fs::write(&path, image_data)?;
        image_paths.push(path);
    }
    
    // Убрано: verbose вывод путей

    // Этап 4: Генерация DOCX отчёта
    // Убрано: info лог
    let mut docx_gen = DocxGenerator::new();
    
    // Читаем изображения как байты
    let mut images_data = Vec::new();
    for path in &image_paths {
        let image_bytes = fs::read(path)?;
        images_data.push(image_bytes);
    }
    
    let docx_data = docx_gen.generate_report(&elements, &groups, &images_data)?;
    
    let docx_path = output_path.join("analysis_report.docx");
    fs::write(&docx_path, docx_data)?;
    
    // Убрано: финальные сообщения
    
    // Анализируем отличия третьего этажа
    analyze_third_floor_differences(&elements);
    
    Ok(())
}

fn analyze_third_floor_differences(elements: &[types::LiraElement]) {
    // Убрано: весь анализ этажей
}