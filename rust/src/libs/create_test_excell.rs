// Импортируем необходимые компоненты
use rust_xlsxwriter::{Chart, ChartType, Color, Format, IntoColor, Workbook, Worksheet, XlsxError};

fn main() -> Result<(), XlsxError> {
    // Создаем новый файл Excel
    let mut workbook = Workbook::new();
    
    // Добавляем лист
    let worksheet = workbook.add_worksheet();
    
    // Добавляем данные для диаграммы
    let categories = ["A", "B", "C", "D", "E"];
    let values = [10, 40, 50, 20, 10];
    
    // Записываем заголовки
    worksheet.write(0, 0, "Категория")?;
    worksheet.write(0, 1, "Значение")?;
    
    // Записываем данные
    for i in 0..categories.len() {
        worksheet.write(i as u32 + 1, 0, categories[i])?;
        worksheet.write(i as u32 + 1, 1, values[i])?;
    }
    
    // Создаем столбчатую диаграмму
    let mut chart = Chart::new(ChartType::BarStacked);
    
    // Настраиваем диаграмму
    chart.title().set_name("Гистограмма с градиентной заливкой");
    chart.x_axis().set_name("Категории");
    chart.y_axis().set_name("Значения");
    
    // Добавляем серию данных с градиентной заливкой
  let mut series = chart.add_series();
		series.set_categories("Sheet1!$A$2:$A$6"); // Предполагая, что данные начинаются со строки 2
		series.set_values("Sheet1!$B$2:$B$6"); // Предполагая, что данные начинаются со строки 2
		series.set_name("Sheet1!$B$1");
    
    // Устанавливаем градиентную заливку для серии
    // Переход от одного цвета к другому (например, от синего к красному)
    // series.set_gradient_fill(&[Color::RGB(0x36, 0x8A, 0xCE), Color::RGB(0xC0, 0x50, 0x4D)]);
	// let color =IntoColor::new_color(&[color]);
    series.set_point_colors(&["#FF000", "#FFC000", "#FFFF00"]);
    // Вставляем диаграмму в лист
    worksheet.insert_chart(1, 3, &chart)?;
    
    // Сохраняем файл
    workbook.save("gradient_chart.xlsx")?;
    
    Ok(())
}