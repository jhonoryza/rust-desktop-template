//! Headless UI verification with egui_kittest, mirroring FilmCraft's
//! scripted UI tests: drive the app by label/automation id and snapshot
//! the result for golden-image diffs.

use egui_kittest::Harness;
use egui_kittest::kittest::Queryable;
use template_core::Document;
use template_core::command::Registry;
use template_ui_egui::{AppState, TemplateApp};

#[test]
fn insert_button_fills_document() {
    let state = AppState {
        doc: Document::default(),
        registry: Registry::new(),
    };
    let mut app = TemplateApp::new(state);

    let mut harness = Harness::new_ui(|ui| {
        eframe::App::ui(&mut app, ui, &mut eframe::Frame::_new_kittest());
    });

    harness.get_by_label("Insert Hello").click();
    harness.run();
    drop(harness);

    let text = app.state.doc.text.clone();
    assert_eq!(text, "Hello ");

    #[cfg(all(feature = "wgpu", feature = "snapshot"))]
    harness.snapshot("insert_hello");
}
