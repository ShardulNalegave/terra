
// ===== Imports =====
use macroquad::prelude::*;
use crate::{
  state::State,
};
// ===================

pub struct AltitudeHeatMapRenderer {
  //
}

impl AltitudeHeatMapRenderer {
  pub fn new() -> Self {
    Self {}
  }
  
  pub fn render(&self, _state: &State) {
    clear_background(LIGHTGRAY);

    draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
    draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
    draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

    draw_text("HELLO", 20.0, 20.0, 30.0, DARKGRAY);
  }
}