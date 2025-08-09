pub mod parse;
pub mod types; 
pub mod drawItem;
pub mod createDxf;
pub mod getTransformedObject;
pub mod unification_data;
pub mod arm_combination;

pub mod generate_documents;
pub mod gpu_renderer;

mod constants {          // папка constants
    pub mod arm_consts;  // файл arm_consts.rs внутри
}
pub mod final_report {
    pub mod custom_sortament;
    pub mod sortament_data;  // Добавить эту строку
}
pub mod convas_optimization {
    pub mod canvas_optimization;
}