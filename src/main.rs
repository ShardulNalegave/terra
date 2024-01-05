
mod ui;
mod state;
mod context;
mod inputs;
mod renderer;
mod terrain;

// ===== Imports =====
use bevy::{prelude::*, window::WindowMode};
use bevy_egui::EguiPlugin;
use context::AppContext;
use state::AppState;
use terrain::TerrainData;
// ===================

fn main() {
  let defaults = DefaultPlugins
    .set(WindowPlugin {
      primary_window: Some(Window {
        title: "Terra - Procedural Terrain Generator".to_owned(),
        mode: WindowMode::BorderlessFullscreen,
        ..Default::default()
      }),
      ..Default::default()
    });

  App::new()
    .add_plugins(defaults)
    .add_plugins(EguiPlugin)
    .init_resource::<AppContext>()
    .init_resource::<TerrainData>()
    .add_state::<AppState>()
    .add_systems(Update, ui::show_ui.run_if(in_state(AppState::UI)))
    .add_systems(Update, inputs::key_released)
    .run();
}