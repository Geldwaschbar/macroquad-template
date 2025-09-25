use macroquad::prelude::*;

pub struct Spritesheet {
    texture: Texture2D,
    sprite_dim: Vec2,
}

impl Spritesheet {
    pub async fn new(path: &'static str) -> Spritesheet {
        let texture = load_texture(path)
            .await
            .expect("expected spritesheet exists");
        texture.set_filter(FilterMode::Nearest);
        Spritesheet {
            texture,
            sprite_dim: Vec2::splat(8.0),
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
                    self.sprite_width() * scale,
                    self.sprite_height() * scale,
                )),
                ..DrawTextureParams::default()
            },
        );
    }

    pub fn draw_sprite(&self, sprite: &Vec2, pos: Vec2, scale: f32) {
        assert!(scale > 0.0);
        draw_texture_ex(
            &self.texture,
            pos.x,
            pos.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(self.sprite_dim * scale),
                source: Some(Rect::new(
                    sprite.x * self.sprite_width(),
                    sprite.y * self.sprite_height(),
                    self.sprite_width(),
                    self.sprite_height(),
                )),
                ..DrawTextureParams::default()
            },
        );
    }

    pub fn sprite_width(&self) -> f32 {
        self.sprite_dim.x
    }

    pub fn sprite_height(&self) -> f32 {
        self.sprite_dim.y
    }
}
