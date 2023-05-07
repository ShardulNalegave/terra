
pub mod ui;
pub mod graphics;
pub mod state;
pub mod renderer;

// ===== Imports =====
use macroquad::prelude::*;
use crate::{
  state::State,
};
// ===================

fn window_conf() -> Conf {
  Conf {
    window_title: "Terra - Procedural Terrain Generator".to_owned(),
    fullscreen: true,
    ..Default::default()
  }
}

#[macroquad::main(window_conf)]
async fn main() {
  let mut state = State::new();

  loop {
    clear_background(LIGHTGRAY);

    graphics::render(&state);
    ui::render_ui(&mut state);

    next_frame().await
  }
}