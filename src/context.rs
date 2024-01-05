
// ===== Imports =====
use bevy::prelude::*;
use crate::state::RendererType;
// ===================

#[derive(Resource)]
pub struct AppContext {
  pub renderer_type: RendererType,
}

impl Default for AppContext {
  fn default() -> Self {
    Self { renderer_type: RendererType::Heatmap }
  }
}