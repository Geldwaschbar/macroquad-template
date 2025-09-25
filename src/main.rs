mod spritesheet;
mod tilemap;

use macroquad::prelude::*;

use crate::spritesheet::Spritesheet;
use crate::tilemap::*;

#[macroquad::main("Texture")]
async fn main() {
    let spritesheet = Spritesheet::new("assets/spritesheet.png").await;
    let tilemap = Tilemap::from(pico8::Loader {}, include_bytes!("../assets/spritemap.txt"));
    loop {
        clear_background(BLACK);
        tilemap.draw_area(
            &spritesheet,
            Rect::new(0.0, 0.0, 31.0, 31.0),
            Vec2::ZERO,
            5.0,
        );

        next_frame().await
    }
}
