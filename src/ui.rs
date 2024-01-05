
// ===== Imports =====
use bevy::{prelude::*, app::AppExit};
use bevy_egui::{egui::{self, Color32}, EguiContexts};
use crate::{state::RendererType, context::AppContext};
// ===================

pub fn show_ui(
  mut app_ctx: ResMut<AppContext>,
  mut contexts: EguiContexts,
  mut exit: EventWriter<AppExit>,
) {
  egui::Window::new("Terrain").show(contexts.ctx_mut(), |ui| {
    ui.horizontal(|ui| {
      ui.colored_label(Color32::LIGHT_BLUE, "Terrain Renderer: ");
      egui::ComboBox::from_label("")
        .selected_text(format!("{:?}", app_ctx.renderer_type))
        .show_ui(ui, |ui| {
          ui.selectable_value(&mut app_ctx.renderer_type, RendererType::Heatmap, "HeatMap");
          ui.selectable_value(&mut app_ctx.renderer_type, RendererType::Terrain2D, "Terrain 2D");
          ui.selectable_value(&mut app_ctx.renderer_type, RendererType::Terrian3D, "Terrain 3D");
        });
    });
  });

  egui::TopBottomPanel::top("Top-Bar").show(contexts.ctx_mut(), |ui| {
    egui::menu::bar(ui, |ui| {
      ui.label("Terra - Procedural Terrain Generator");
      ui.separator();
      ui.menu_button("File", |ui| {
        if ui.button("Open").clicked() {
          // …
        }
      });
      ui.separator();
      if ui.button("Quit").clicked() {
        exit.send(AppExit);
      }
    });
 });
}