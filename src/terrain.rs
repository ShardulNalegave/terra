
// ===== Imports =====
use bevy::prelude::*;
use noise::{Perlin, NoiseFn};
// ===================

pub const VERT_DIST: u32 = 10;
pub const DEFAULT_MESH_SIZE: (u32, u32) = (50, 50);
pub const DEFAULT_NOISE_SEED: u32 = 100;

#[derive(Resource)]
pub struct TerrainData {
  mesh_size: (u32, u32),
  noise_seed: u32,
  vertices: Vec<Vec<Vec3>>,
}

impl Default for TerrainData {
  fn default() -> Self {
    let vertices = generate_vertices(DEFAULT_MESH_SIZE, DEFAULT_NOISE_SEED);
    Self {
      mesh_size: DEFAULT_MESH_SIZE,
      noise_seed: DEFAULT_NOISE_SEED,
      vertices,
    }
  }
}

impl TerrainData {
  pub fn get_mesh_size(&self) -> (u32, u32) {
    return self.mesh_size;
  }

  pub fn get_noise_seed(&self) -> u32 {
    return self.noise_seed;
  }

  pub fn update(&mut self, noise_seed: u32, mesh_size: (u32, u32)) {
    self.mesh_size = mesh_size;
    self.noise_seed = noise_seed;
    self.vertices = generate_vertices(self.mesh_size, self.noise_seed);
  }

}

fn generate_vertices(
  mesh_size: (u32, u32),
  noise_seed: u32,
) -> Vec<Vec<Vec3>> {
  let generator = Perlin::new(noise_seed);
  let mut vertices = vec![];

  for i in 0..mesh_size.0 {
    let mut row = vec![];
    for j in 0..mesh_size.1 {
      let x = (VERT_DIST * i) as f64;
      let y = (VERT_DIST * j) as f64;
      let z = generator.get([x, y]);
      let vert = Vec3::new(x as f32, y as f32, z as f32);
      row.push(vert);
    }
    vertices.push(row);
  }

  return vertices;
}