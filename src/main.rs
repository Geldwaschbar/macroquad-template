mod player;
mod spritesheet;
mod tilemap;

use macroquad::prelude::*;

use crate::player::Player;
use crate::spritesheet::Spritesheet;
use crate::tilemap::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Macroquad Template".to_owned(),
        high_dpi: true,
        sample_count: 1,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let spritesheet = Spritesheet::new("assets/spritesheet.png").await;
    let tilemap = Tilemap::from(&mut pico8::Loader::new(include_bytes!(
        "../assets/spritemap.txt"
    )));
    let mut player = Player::new();
    player.move_to(Vec2::new(64.0, 64.0));
    loop {
        #[cfg(not(target_arch = "wasm32"))]
        if is_key_down(KeyCode::Q) | is_key_down(KeyCode::Escape) {
            break;
        }

        clear_background(BLACK);
        player.movement(&spritesheet, &tilemap);
        set_camera(player.get_camera());
        tilemap.draw_area(
            &spritesheet,
            player.get_viewport(&spritesheet),
            Vec2::ZERO,
            1.0,
        );
        player.draw();
        set_default_camera();

        next_frame().await
    }
}
