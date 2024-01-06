
// ===== Imports =====
use bevy::prelude::*;
use crate::{state::RendererType, terrain};
// ===================

#[derive(Resource)]
pub struct AppContext {
  pub renderer_type: RendererType,
  pub ui_terrain_data: UITerrainData,
}

impl Default for AppContext {
  fn default() -> Self {
    Self {
      renderer_type: RendererType::Heatmap,
      ui_terrain_data: Default::default(),
    }
  }
}

pub struct UITerrainData {
  pub mesh_size: (u32, u32),
  pub noise_seed: u32,
}

impl Default for UITerrainData {
  fn default() -> Self {
    Self {
      mesh_size: terrain::DEFAULT_MESH_SIZE,
      noise_seed: terrain::DEFAULT_NOISE_SEED,
    }
  }
}