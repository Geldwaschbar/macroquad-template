use macroquad::{prelude::*, texture};

pub struct Spritesheet {
    texture: Texture2D,
    tile_dim: Vec2,
}

impl Spritesheet {
    pub async fn new(path: &'static str) -> Spritesheet {
        let texture = load_texture(path)
            .await
            .expect("expected spritesheet exists");
        texture.set_filter(FilterMode::Nearest);
        Spritesheet {
            texture,
            tile_dim: Vec2::splat(8.0),
        }
    }

    pub fn draw_self(&self, scale: f32) {
        draw_texture_ex(
            &self.texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(
                    self.texture.width() * scale,
                    self.texture.height() * scale,
                )),
                ..DrawTextureParams::default()
            },
        );
    }

    pub fn draw_sprite(&self, sprite: Vec2, pos: Vec2, scale: f32) {
        assert!(scale > 0.0);
        draw_texture_ex(
            &self.texture,
            pos.x,
            pos.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(self.tile_dim * scale),
                source: Some(Rect::new(
                    sprite.x * self.tile_dim.x,
                    sprite.x * self.tile_dim.y,
                    self.tile_dim.x,
                    self.tile_dim.y,
                )),
                ..DrawTextureParams::default()
            },
        );
    }

    pub fn width(&self) -> usize {
        (self.texture.width() / self.tile_dim.x) as usize
    }

    pub fn height(&self) -> usize {
        (self.texture.height() / self.tile_dim.y) as usize
    }
}
