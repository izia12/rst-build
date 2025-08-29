//! Модуль для мониторинга и оптимизации производительности генерации DOCX

use std::time::Duration;
use serde::{Deserialize, Serialize};
use super::wasm_time::WasmInstant;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub total_generation_time: Duration,
    pub image_generation_time: Duration,
    pub docx_creation_time: Duration,
    pub elements_count: usize,
    pub images_count: usize,
    pub avg_time_per_image: Duration,
    pub memory_usage_mb: f64,
}

#[derive(Debug, Clone)]
pub struct PerformanceConfig {
    pub enable_parallel_rendering: bool,
    pub enable_gpu_acceleration: bool,
    pub max_image_size: (u32, u32),
    pub compression_quality: u8,
    pub enable_caching: bool,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            enable_parallel_rendering: true,
            enable_gpu_acceleration: false, // Отключено по умолчанию для стабильности
            max_image_size: (1200, 900), // Уменьшенные размеры для быстрой генерации
            compression_quality: 85,
            enable_caching: true,
        }
    }
}

pub struct PerformanceMonitor {
    start_time: WasmInstant,
    image_start_time: Option<WasmInstant>,
    docx_start_time: Option<WasmInstant>,
    config: PerformanceConfig,
}

impl PerformanceMonitor {
    pub fn new(config: PerformanceConfig) -> Self {
        Self {
            start_time: WasmInstant::now(),
            image_start_time: None,
            docx_start_time: None,
            config,
        }
    }

    pub fn start_image_generation(&mut self) {
        self.image_start_time = Some(WasmInstant::now());
    }

    pub fn end_image_generation(&mut self) -> Duration {
        self.image_start_time
            .map(|start| start.elapsed())
            .unwrap_or_default()
    }

    pub fn start_docx_creation(&mut self) {
        self.docx_start_time = Some(WasmInstant::now());
    }

    pub fn end_docx_creation(&mut self) -> Duration {
        self.docx_start_time
            .map(|start| start.elapsed())
            .unwrap_or_default()
    }

    pub fn get_config(&self) -> &PerformanceConfig {
        &self.config
    }

    pub fn finish(
        &self,
        image_time: Duration,
        docx_time: Duration,
        elements_count: usize,
        images_count: usize,
    ) -> PerformanceMetrics {
        let total_time = self.start_time.elapsed();
        let avg_time_per_image = if images_count > 0 {
            image_time / images_count as u32
        } else {
            Duration::default()
        };

        PerformanceMetrics {
            total_generation_time: total_time,
            image_generation_time: image_time,
            docx_creation_time: docx_time,
            elements_count,
            images_count,
            avg_time_per_image,
            memory_usage_mb: Self::estimate_memory_usage(elements_count, images_count),
        }
    }

    fn estimate_memory_usage(elements_count: usize, images_count: usize) -> f64 {
        // Примерная оценка использования памяти
        let element_size_kb = 0.5; // ~500 байт на элемент
        let image_size_mb = 15.0; // ~15 МБ на изображение (2500x2000x3 байта)
        
        (elements_count as f64 * element_size_kb / 1024.0) + (images_count as f64 * image_size_mb)
    }

    pub fn should_use_parallel_rendering(&self) -> bool {
        self.config.enable_parallel_rendering
    }

    pub fn should_use_gpu_acceleration(&self, elements_count: usize) -> bool {
		self.config.enable_gpu_acceleration && elements_count > 100
	}
}

/// Утилита для логирования производительности
pub fn log_performance_metrics(metrics: &PerformanceMetrics) {
    // Логирование отключено для улучшения производительности
}

/// Рекомендации по оптимизации на основе метрик
pub fn get_optimization_recommendations(metrics: &PerformanceMetrics) -> Vec<String> {
    let mut recommendations = Vec::new();

    if metrics.avg_time_per_image.as_secs_f64() > 10.0 {
        recommendations.push("⚡ Consider enabling GPU acceleration for large datasets".to_string());
    }

    if metrics.memory_usage_mb > 500.0 {
        recommendations.push("💾 High memory usage detected - consider reducing image size".to_string());
    }

    if metrics.image_generation_time.as_secs_f64() > metrics.docx_creation_time.as_secs_f64() * 5.0 {
        recommendations.push("🖼️ Image generation is the bottleneck - focus on image optimization".to_string());
    }

    if metrics.elements_count > 50000 {
        recommendations.push("📊 Large dataset detected - consider implementing streaming generation".to_string());
    }

    if recommendations.is_empty() {
        recommendations.push("✅ Performance looks good!".to_string());
    }

    recommendations
}