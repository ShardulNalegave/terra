
// ===== Imports =====
use std::process::exit;
use crate::{
  state::State,
};
// ===================

pub fn render_ui(_state: &mut State) {
  egui_macroquad::ui(|egui_ctx| {
    egui::Window::new("Window Controls").show(egui_ctx, |ui| {
      if ui.button("Close Application").clicked() {
        exit(0);
      }
    });
  });

  egui_macroquad::draw();
}