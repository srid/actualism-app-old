//! Application state

use dioxus_signals::{Signal, Writable};

#[derive(Clone, Copy)]
pub struct AppState {
    pub name: Signal<String>,
}

impl AppState {
    pub fn new() -> Self {
        let name = std::env::var("USER").unwrap_or("world".to_string());
        Self {
            name: Signal::new(name),
        }
    }
}
