
// ===== Imports =====
use crate::{
  state::State,
  renderer::{
    heatmap::AltitudeHeatMapRenderer,
  },
};
use crate::renderer::Renderer;
// ===================

pub struct Graphics {
  altitude_heatmap_renderer: AltitudeHeatMapRenderer,
}

impl Graphics {
  pub fn new() -> Self {
    Self {
      altitude_heatmap_renderer: AltitudeHeatMapRenderer::new(),
    }
  }
  
  pub fn render(&self, state: &State) {
    match state.renderer_type {
      Renderer::AltitudeHeatMap => self.altitude_heatmap_renderer.render(state),
    }
  }
}