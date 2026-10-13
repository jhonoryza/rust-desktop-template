//! template-core — engine layer.
//!
//! Rules (mirroring storytold's craft repos):
//! - No UI dependencies ever (egui/eframe/winit live only in `ui-egui`).
//! - Everything user-visible is a `CommandSpec` dispatched by id, so CLI,
//!   control channel and tests all drive the exact same code paths as the GUI.
//! - Never panic outside tests: `Result<T, E>` everywhere.

use serde::{Deserialize, Serialize};

pub mod command;

/// Top-level engine error.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unknown command: {0}")]
    UnknownCommand(String),
    #[error("bad parameters for {command}: {message}")]
    BadParams { command: String, message: String },
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Example domain state. Replace with your real model.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Document {
    pub text: String,
}

impl Document {
    pub fn insert_text(&mut self, at: usize, text: &str) {
        let at = at.min(self.text.len());
        // Panic-free insertion: operate on char boundaries only.
        let mut idx = at;
        while !self.text.is_char_boundary(idx) {
            idx -= 1;
        }
        self.text.insert_str(idx, text);
    }
}
