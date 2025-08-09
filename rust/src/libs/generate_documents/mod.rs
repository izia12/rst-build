//! Модуль для генерации документов
//! 
//! Этот модуль содержит функциональность для создания DOCX документов
//! с оптимизацией производительности и мониторингом.

pub mod docx_generator;
pub mod performance;
pub mod image_generation;
pub mod test_optimization;
pub mod wasm_time;

// Реэкспорт основных функций для удобства использования
pub use docx_generator::{
    create_docx_document,
    create_docx_document_optimized,
    create_docx_document_legacy,
    create_docx_for_selected_floors
};

pub use performance::{
    PerformanceConfig,
    PerformanceMetrics,
    PerformanceMonitor,
    log_performance_metrics,
    get_optimization_recommendations
};