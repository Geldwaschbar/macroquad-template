mod spritesheet;

use macroquad::prelude::*;

use crate::spritesheet::Spritesheet;

#[macroquad::main("Texture")]
async fn main() {
    let mut spritesheet = Spritesheet::new("assets/spritesheet.png").await;
    loop {
        clear_background(WHITE);
        spritesheet.draw_self(4.0);

        next_frame().await
    }
}
