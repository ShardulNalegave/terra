
// ===== Imports =====
use bevy::ecs::schedule::States;
// ===================

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
  #[default]
  UI,
  TerrainView(RendererType),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum RendererType {
  Heatmap,
}