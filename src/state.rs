
// ===== Imports =====
use crate::{
  terrain::Terrain,
  renderer::Renderer,
};
// ===================

pub struct State {
  pub renderer_type: Renderer,
  pub terrain: Option<Terrain>,
}

impl State {
  pub fn new() -> Self {
    Self {
      terrain: None,
      renderer_type: Renderer::AltitudeHeatMap,
    }
  }
}