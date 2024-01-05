
// ===== Imports =====
use bevy::{prelude::*, app::AppExit};
use crate::state::{AppState, RendererType};
// ===================

pub fn toggle_keyreleased(
  app_state: Res<State<AppState>>,
  mut next_app_state: ResMut<NextState<AppState>>,
  input: Res<Input<KeyCode>>,
  mut exit: EventWriter<AppExit>,
) {
  match app_state.get() {
    AppState::UI => if input.just_released(KeyCode::U) {
      next_app_state.set(AppState::TerrainView(RendererType::Heatmap));
    },
    AppState::TerrainView(_) => if input.just_released(KeyCode::Q) {
      exit.send(AppExit);
    } else if input.just_released(KeyCode::U) {
      next_app_state.set(AppState::UI);
    },
  }
}