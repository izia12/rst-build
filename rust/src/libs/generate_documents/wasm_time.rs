//! WASM-compatible timing utilities

use std::time::Duration;
use wasm_bindgen::prelude::*;

/// WASM-compatible instant for timing measurements
#[derive(Debug, Clone, Copy)]
pub struct WasmInstant {
    timestamp: f64,
}

impl WasmInstant {
    /// Creates a new instant representing the current time
    pub fn now() -> Self {
        let timestamp = web_sys::window()
            .and_then(|window| window.performance())
            .map(|performance| performance.now())
            .unwrap_or(0.0);
        
        Self { timestamp }
    }
    
    /// Returns the amount of time elapsed since this instant
    pub fn elapsed(&self) -> Duration {
        let now = Self::now().timestamp;
        let elapsed_ms = now - self.timestamp;
        Duration::from_millis(elapsed_ms.max(0.0) as u64)
    }
}

/// Helper function to get current timestamp in milliseconds
pub fn now_ms() -> f64 {
    web_sys::window()
        .and_then(|window| window.performance())
        .map(|performance| performance.now())
        .unwrap_or(0.0)
}

/// Helper function to calculate duration between two timestamps
pub fn duration_from_ms(start_ms: f64, end_ms: f64) -> Duration {
    let elapsed_ms = (end_ms - start_ms).max(0.0);
    Duration::from_millis(elapsed_ms as u64)
}