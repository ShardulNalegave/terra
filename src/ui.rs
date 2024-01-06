
// ===== Imports =====
use bevy::{prelude::*, app::AppExit};
use bevy_egui::{egui::{self, Color32, DragValue, ComboBox, Button}, EguiContexts};
use crate::{state::RendererType, context::AppContext, terrain::TerrainData};
// ===================

pub fn show_ui(
  mut app_ctx: ResMut<AppContext>,
  mut terrain_data: ResMut<TerrainData>,
  mut contexts: EguiContexts,
  mut exit: EventWriter<AppExit>,
) {
  egui::Window::new("Terrain").show(contexts.ctx_mut(), |ui| {
    ui.horizontal(|ui| {
      ui.colored_label(Color32::LIGHT_BLUE, "Terrain Renderer: ");
      ComboBox::from_label("")
        .selected_text(format!("{:?}", app_ctx.renderer_type))
        .show_ui(ui, |ui| {
          ui.selectable_value(&mut app_ctx.renderer_type, RendererType::Heatmap, "HeatMap");
          ui.selectable_value(&mut app_ctx.renderer_type, RendererType::Terrain2D, "Terrain 2D");
          ui.selectable_value(&mut app_ctx.renderer_type, RendererType::Terrian3D, "Terrain 3D");
        });
    });

    ui.separator();

    ui.horizontal(|ui| {
      ui.colored_label(Color32::LIGHT_BLUE, "Noise Seed: ");
      ui.add(DragValue::new(&mut app_ctx.ui_terrain_data.noise_seed).speed(1));
    });

    ui.horizontal(|ui| {
      ui.colored_label(Color32::LIGHT_BLUE, "Mesh Size: ");
      ui.add(DragValue::new(&mut app_ctx.ui_terrain_data.mesh_size.0).speed(1).prefix("x: "));
      ui.add(DragValue::new(&mut app_ctx.ui_terrain_data.mesh_size.1).speed(1).prefix("y: "));
    });

    let should_allow_generate = {
      let is_noise_seed_same = terrain_data.get_noise_seed() == app_ctx.ui_terrain_data.noise_seed;
      let is_mesh_size_same = terrain_data.get_mesh_size() == app_ctx.ui_terrain_data.mesh_size;
      !(is_noise_seed_same && is_mesh_size_same)
    };

    if ui.add_enabled(should_allow_generate, Button::new("Generate")).clicked() {
      terrain_data.update(app_ctx.ui_terrain_data.noise_seed, app_ctx.ui_terrain_data.mesh_size);
    }
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