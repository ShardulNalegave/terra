
mod ui;
mod state;
mod context;
mod inputs;

// ===== Imports =====
use bevy::{prelude::*, window::WindowMode};
use bevy_egui::EguiPlugin;
use context::AppContext;
use state::AppState;
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
    .add_state::<AppState>()
    .add_systems(Update, ui::show_ui.run_if(in_state(AppState::UI)))
    .add_systems(Update, inputs::key_released)
    .run();
}