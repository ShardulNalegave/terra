
// ===== Imports =====
use bevy_egui::{egui, EguiContexts};
// ===================

pub fn show_ui(mut contexts: EguiContexts) {
  egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
    ui.label("world");
  });
}