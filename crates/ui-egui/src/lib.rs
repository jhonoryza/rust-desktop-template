//! template-ui-egui — the egui front end.
//!
//! The UI is thin: it owns no state beyond the window and renders the
//! engine's `Document` through the `Registry`. Swap this crate out and the
//! engine still works (CLI, tests, control channel).

use eframe::egui;
use serde_json::json;
use template_core::Document;
use template_core::command::Registry;

/// App state handed to the UI.
pub struct AppState {
    pub doc: Document,
    pub registry: Registry,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            doc: Document::default(),
            registry: Registry::new(),
        }
    }
}

/// The eframe app.
pub struct TemplateApp {
    pub state: AppState,
    status: String,
}

impl TemplateApp {
    #[must_use]
    pub fn new(state: AppState) -> Self {
        Self {
            state,
            status: String::from("Ready"),
        }
    }

    fn run(&mut self, id: &str, params: serde_json::Value) {
        match self
            .state
            .registry
            .execute(&mut self.state.doc, id, &params)
        {
            Ok(()) => self.status = format!("{id}: ok"),
            Err(e) => self.status = format!("{id}: {e}"),
        }
    }
}

impl eframe::App for TemplateApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Toolbar panel at the top.
        egui::Panel::top("toolbar").show(ui, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Insert Hello").clicked() {
                    self.run("text.insert", json!({"text": "Hello "}));
                }
                if ui.button("Clear").clicked() {
                    self.run("text.clear", json!({}));
                }
                ui.separator();
                ui.weak(&self.status);
            });
        });

        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Template App");
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add(
                    egui::Label::new(egui::RichText::new(&self.state.doc.text).size(16.0)).wrap(),
                );
            });
        });
    }
}

/// Run the desktop app.
pub fn run(state: AppState) -> eframe::Result {
    let native = eframe::NativeOptions {
        renderer: eframe::Renderer::Wgpu,
        ..Default::default()
    };
    eframe::run_native(
        "Template App",
        native,
        Box::new(move |_cc| Ok(Box::new(TemplateApp::new(state)))),
    )
}
