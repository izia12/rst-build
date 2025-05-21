use dxf::entities::*;
use dxf::enums::AcadVersion;
use dxf::tables::Layer;
use dxf::Block;
use dxf::Drawing;
use dxf::Point;

fn main() {
    let mut drawing = Drawing::new();
    // drawing.header.version = AcadVersion::R2007;

    // Создание слоев
    drawing.add_layer( Layer::default());
    drawing.add_layer(Layer::default());

    // Создание блока
    let mut block = Block::default();
    block.name = "MyBlock".into();

    // Добавление элементов в блок
    block.entities.push(Entity {
        common: Default::default(),
        specific: EntityType::Line(Line::new(Point { x: (0.0), y: (0.0), z: (0.0) }, Point { x: (0.0), y: (0.0), z: (0.0) })),
    });

    // Сохранение
    drawing.save_file("output.dxf").unwrap();
}