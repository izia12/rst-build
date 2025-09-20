use crate::types::*;
use anyhow::{Result, Context};
use docx_rs::*;
use log::{debug, info};
use std::io::Cursor;

/// Генератор DOCX документов
pub struct DocxGenerator {
    /// Настройки документа
    settings: DocumentSettings,
}

/// Настройки документа
#[derive(Debug, Clone)]
pub struct DocumentSettings {
    /// Заголовок документа
    pub title: String,
    /// Автор
    pub author: String,
    /// Размер изображений (в EMU - English Metric Units)
    pub image_width_emu: u32,
    pub image_height_emu: u32,
    /// Размеры страницы
    pub page_width: u32,
    pub page_height: u32,
    /// Поля страницы
    pub margins: PageMargins,
}

#[derive(Debug, Clone)]
pub struct PageMargins {
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
    pub left: i32,
}

impl Default for DocumentSettings {
    fn default() -> Self {
        Self {
            title: "Анализ связных компонент LIRA".to_string(),
            author: "LIRA Analyzer".to_string(),
            // Размеры изображения: 800x600 пикселей = ~6x4.5 дюйма при 150 DPI
            image_width_emu: 6 * 914400,  // 6 дюймов в EMU
            image_height_emu: 4 * 914400, // 4 дюйма в EMU
            // A4 в альбомной ориентации
            page_width: 16838,  // ~11.7 дюйма
            page_height: 11906, // ~8.3 дюйма
            margins: PageMargins {
                top: 720,    // 0.5 дюйма
                right: 720,
                bottom: 720,
                left: 720,
            },
        }
    }
}

impl DocxGenerator {
    /// Создает новый генератор
    pub fn new() -> Self {
        Self {
            settings: DocumentSettings::default(),
        }
    }

    /// Создает генератор с настройками
    pub fn with_settings(settings: DocumentSettings) -> Self {
        Self { settings }
    }

    /// Генерирует отчёт в формате DOCX
    pub fn generate_report(
        &mut self,
        elements: &[LiraElement],
        groups: &[ConnectedGroup],
        images: &[Vec<u8>],
    ) -> Result<Vec<u8>> {
        info!("Генерация DOCX отчёта: {} элементов, {} групп, {} изображений", 
              elements.len(), groups.len(), images.len());

        let mut doc = Docx::new()
            .page_size(self.settings.page_width, self.settings.page_height)
            .page_orient(PageOrientationType::Landscape)
            .page_margin(PageMargin {
                top: self.settings.margins.top,
                right: self.settings.margins.right,
                bottom: self.settings.margins.bottom,
                left: self.settings.margins.left,
                header: 0,
                footer: 0,
                gutter: 0,
            });

        // 1. Титульная страница
        doc = self.add_title_page(doc, elements, groups)?;

        // 2. Изображения этажей как в рабочем проекте
        doc = self.add_floor_images_section(doc, images)?;

        // 3. Анализ по группам (если есть значимые группы)
        if !groups.is_empty() {
            for (i, group) in groups.iter().enumerate() {
                // Используем первое изображение для каждой группы
                let image_data = if !images.is_empty() { &images[0] } else { &Vec::new() };
                doc = self.add_group_section(doc, group, image_data, i + 1)?;
            }
        }

        // 4. Детальная статистика
        doc = self.add_statistics_section(doc, elements, groups)?;

        // Создаем буфер и записываем документ
        let mut buffer = Cursor::new(Vec::new());
        doc.build()
            .pack(&mut buffer)
            .context("Ошибка создания DOCX документа")?;

        info!("DOCX отчёт создан успешно");
        Ok(buffer.into_inner())
    }

    /// Добавляет титульную страницу
    fn add_title_page(
        &self,
        mut doc: Docx,
        elements: &[LiraElement],
        groups: &[ConnectedGroup],
    ) -> Result<Docx> {
        debug!("Добавление титульной страницы");

        // Заголовок
        doc = doc.add_paragraph(
            Paragraph::new()
                .align(AlignmentType::Center)
                .add_run(
                    Run::new()
                        .add_text(&self.settings.title)
                        .size(48)
                        .bold()
                ),
        );

        // Подзаголовок
        doc = doc.add_paragraph(
            Paragraph::new()
                .align(AlignmentType::Center)
                .add_run(
                    Run::new()
                        .add_text("Отчёт о поиске связных компонент")
                        .size(24)
                        .italic()
                ),
        );

        // Пустая строка
        doc = doc.add_paragraph(Paragraph::new());

        // Общая статистика
        doc = doc.add_paragraph(
            Paragraph::new()
                .add_run(
                    Run::new()
                        .add_text("Общая статистика:")
                        .size(20)
                        .bold()
                ),
        );

        let shell_count = elements.iter().filter(|e| e.is_shell()).count();
        let beam_count = elements.iter().filter(|e| e.is_beam()).count();

        doc = doc.add_paragraph(
            Paragraph::new()
                .add_run(
                    Run::new()
                        .add_text(&format!("• Всего элементов: {}", elements.len()))
                        .size(16)
                ),
        );

        doc = doc.add_paragraph(
            Paragraph::new()
                .add_run(
                    Run::new()
                        .add_text(&format!("• Пластинчатые элементы: {}", shell_count))
                        .size(16)
                ),
        );

        doc = doc.add_paragraph(
            Paragraph::new()
                .add_run(
                    Run::new()
                        .add_text(&format!("• Стержневые элементы: {}", beam_count))
                        .size(16)
                ),
        );

        doc = doc.add_paragraph(
            Paragraph::new()
                .add_run(
                    Run::new()
                        .add_text(&format!("• Найдено связных групп: {}", groups.len()))
                        .size(16)
                ),
        );

        // Разрыв страницы
        doc = doc.add_paragraph(
            Paragraph::new().add_run(
                Run::new().add_break(BreakType::Page)
            )
        );

        Ok(doc)
    }

    /// Добавляет секцию с изображениями этажей как в рабочем проекте
    fn add_floor_images_section(&self, mut doc: Docx, images: &[Vec<u8>]) -> Result<Docx> {
        debug!("Добавление секции изображений этажей");

        // Заголовок секции
        doc = doc.add_paragraph(
            Paragraph::new()
                .add_run(
                    Run::new()
                        .add_text("1. Изображения этажей")
                        .size(24)
                        .bold()
                ),
        );

        // Пустая строка
        doc = doc.add_paragraph(Paragraph::new());

        // Добавляем все изображения этажей
        for (i, image_data) in images.iter().enumerate() {
            // Заголовок этажа
            doc = doc.add_paragraph(
                Paragraph::new()
                    .add_run(
                        Run::new()
                            .add_text(&format!("Этаж {}", i + 1))
                            .size(18)
                            .bold()
                    ),
            );

            // Изображение этажа
            doc = self.add_image_to_doc(doc, image_data, &format!("Этаж {}", i + 1))?;

            // Разрыв страницы (кроме последнего изображения)
            if i < images.len() - 1 {
                doc = doc.add_paragraph(
                    Paragraph::new().add_run(
                        Run::new().add_break(BreakType::Page)
                    )
                );
            }
        }

        // Разрыв страницы после всех изображений
        doc = doc.add_paragraph(
            Paragraph::new().add_run(
                Run::new().add_break(BreakType::Page)
            )
        );

        Ok(doc)
    }

    /// Добавляет секцию для группы
    fn add_group_section(
        &self,
        mut doc: Docx,
        group: &ConnectedGroup,
        image_data: &[u8],
        group_number: usize,
    ) -> Result<Docx> {
        debug!("Добавление секции группы {}", group_number);

        // Заголовок секции
        doc = doc.add_paragraph(
            Paragraph::new()
                .add_run(
                    Run::new()
                        .add_text(&format!("{}. Группа {} (ID: {})", group_number + 1, group_number, group.id))
                        .size(24)
                        .bold()
                ),
        );

        // Статистика группы
        doc = doc.add_paragraph(
            Paragraph::new()
                .add_run(
                    Run::new()
                        .add_text("Характеристики группы:")
                        .size(16)
                        .bold()
                ),
        );

        doc = doc.add_paragraph(
            Paragraph::new()
                .add_run(
                    Run::new()
                        .add_text(&format!("• Тип группы: {:?}", group.group_type))
                        .size(14)
                ),
        );

        doc = doc.add_paragraph(
            Paragraph::new()
                .add_run(
                    Run::new()
                        .add_text(&format!("• Всего элементов: {}", group.elements.len()))
                        .size(14)
                ),
        );

        doc = doc.add_paragraph(
            Paragraph::new()
                .add_run(
                    Run::new()
                        .add_text(&format!("• Пластинчатые: {}", group.stats.shell_count))
                        .size(14)
                ),
        );

        doc = doc.add_paragraph(
            Paragraph::new()
                .add_run(
                    Run::new()
                        .add_text(&format!("• Стержневые: {}", group.stats.beam_count))
                        .size(14)
                ),
        );

        doc = doc.add_paragraph(
            Paragraph::new()
                .add_run(
                    Run::new()
                        .add_text(&format!("• Колонны: {}", group.stats.column_count))
                        .size(14)
                ),
        );

        doc = doc.add_paragraph(
            Paragraph::new()
                .add_run(
                    Run::new()
                        .add_text(&format!("• Уникальных узлов: {}", group.stats.total_nodes))
                        .size(14)
                ),
        );

        // Изображение группы
        doc = self.add_image_to_doc(doc, image_data, &format!("Группа {}", group_number))?;

        // Список элементов (если группа небольшая)
        if group.elements.len() <= 20 {
            doc = doc.add_paragraph(
                Paragraph::new()
                    .add_run(
                        Run::new()
                            .add_text("Элементы в группе:")
                            .size(14)
                            .bold()
                    ),
            );

            let elements_text = group.elements
                .iter()
                .map(|id| id.to_string())
                .collect::<Vec<_>>()
                .join(", ");

            doc = doc.add_paragraph(
                Paragraph::new()
                    .add_run(
                        Run::new()
                            .add_text(&elements_text)
                            .size(12)
                    ),
            );
        }

        // Разрыв страницы (кроме последней группы)
        doc = doc.add_paragraph(
            Paragraph::new().add_run(
                Run::new().add_break(BreakType::Page)
            )
        );

        Ok(doc)
    }

    /// Добавляет секцию со статистикой
    fn add_statistics_section(
        &self,
        mut doc: Docx,
        elements: &[LiraElement],
        groups: &[ConnectedGroup],
    ) -> Result<Docx> {
        debug!("Добавление секции статистики");

        // Заголовок секции
        doc = doc.add_paragraph(
            Paragraph::new()
                .add_run(
                    Run::new()
                        .add_text(&format!("{}. Детальная статистика", groups.len() + 2))
                        .size(24)
                        .bold()
                ),
        );

        // Таблица с группами
        doc = doc.add_paragraph(
            Paragraph::new()
                .add_run(
                    Run::new()
                        .add_text("Сводная таблица групп:")
                        .size(16)
                        .bold()
                ),
        );

        // Создаем таблицу
        let mut table = Table::new(vec![
            TableRow::new(vec![
                TableCell::new().add_paragraph(
                    Paragraph::new().add_run(
                        Run::new().add_text("Группа").bold()
                    )
                ),
                TableCell::new().add_paragraph(
                    Paragraph::new().add_run(
                        Run::new().add_text("Элементов").bold()
                    )
                ),
                TableCell::new().add_paragraph(
                    Paragraph::new().add_run(
                        Run::new().add_text("Тип").bold()
                    )
                ),
                TableCell::new().add_paragraph(
                    Paragraph::new().add_run(
                        Run::new().add_text("Пластинчатые").bold()
                    )
                ),
                TableCell::new().add_paragraph(
                    Paragraph::new().add_run(
                        Run::new().add_text("Стержневые").bold()
                    )
                ),
            ])
        ]);

        // Добавляем строки для каждой группы
        for (i, group) in groups.iter().enumerate() {
            let row = TableRow::new(vec![
                TableCell::new().add_paragraph(
                    Paragraph::new().add_run(
                        Run::new().add_text(&(i + 1).to_string())
                    )
                ),
                TableCell::new().add_paragraph(
                    Paragraph::new().add_run(
                        Run::new().add_text(&group.elements.len().to_string())
                    )
                ),
                TableCell::new().add_paragraph(
                    Paragraph::new().add_run(
                        Run::new().add_text(&format!("{:?}", group.group_type))
                    )
                ),
                TableCell::new().add_paragraph(
                    Paragraph::new().add_run(
                        Run::new().add_text(&group.stats.shell_count.to_string())
                    )
                ),
                TableCell::new().add_paragraph(
                    Paragraph::new().add_run(
                        Run::new().add_text(&(group.stats.beam_count + group.stats.column_count).to_string())
                    )
                ),
            ]);
            table = table.add_row(row);
        }

        doc = doc.add_table(table);

        Ok(doc)
    }

    /// Добавляет изображение в документ
    fn add_image_to_doc(&self, mut doc: Docx, image_data: &[u8], _alt_text: &str) -> Result<Docx> {
        // Создаем изображение
        let pic = Pic::new(image_data)
            .size(self.settings.image_width_emu, self.settings.image_height_emu);

        // Добавляем параграф с изображением
        doc = doc.add_paragraph(
            Paragraph::new()
                .align(AlignmentType::Center)
                .add_run(Run::new().add_image(pic))
        );

        // Пустая строка после изображения
        doc = doc.add_paragraph(Paragraph::new());

        Ok(doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::Vec3;

    fn create_test_data() -> (Vec<LiraElement>, Vec<ConnectedGroup>) {
        let elements = vec![
            LiraElement {
                id: 1,
                element_type: ElementType::Shell,
                nodes: vec![1, 2, 3, 4],
                coordinates: vec![Vec3::ZERO; 4],
                properties: ElementProperties::default(),
            },
            LiraElement {
                id: 2,
                element_type: ElementType::Beam,
                nodes: vec![5, 6],
                coordinates: vec![Vec3::ZERO; 2],
                properties: ElementProperties::default(),
            },
        ];

        let mut group = ConnectedGroup::new(0, vec![1, 2]);
        group.update_statistics(&elements);
        let groups = vec![group];

        (elements, groups)
    }

    #[test]
    fn test_generate_report() {
        let mut generator = DocxGenerator::new();
        let (elements, groups) = create_test_data();
        let images = vec![vec![0u8; 100]; 2]; // Фиктивные изображения

        let result = generator.generate_report(&elements, &groups, &images);
        assert!(result.is_ok());
        
        let docx_data = result.unwrap();
        assert!(!docx_data.is_empty());
    }
}