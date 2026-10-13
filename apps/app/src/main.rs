//! Desktop entry point.

fn main() -> eframe::Result {
    env_logger::init();
    template_ui_egui::run(template_ui_egui::AppState::default())
}
