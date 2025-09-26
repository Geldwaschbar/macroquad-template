use macroquad::prelude::*;

pub struct Spritesheet {
    texture: Texture2D,
    sprite_dim: Vec2,
    scale: f32,
}

impl Spritesheet {
    pub async fn new(path: &'static str, scale: f32) -> Spritesheet {
        assert!(scale > 0.0);

        let texture = load_texture(path)
            .await
            .expect("expected spritesheet exists");
        texture.set_filter(FilterMode::Nearest);
        Spritesheet {
            texture,
            sprite_dim: Vec2::splat(8.0),
            scale,
        }
    }

    pub fn draw_self(&self) {
        draw_texture_ex(
            &self.texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(self.sprite_width(), self.sprite_height())),
                ..DrawTextureParams::default()
            },
        );
    }

    pub fn draw_sprite(&self, sprite: &Vec2, pos: Vec2) {
        if pos.floor() != pos {
            dbg!(&pos);
        }
        draw_texture_ex(
            &self.texture,
            pos.x - 0.1,
            pos.y - 0.1,
            WHITE,
            DrawTextureParams {
                dest_size: Some(self.sprite_dim * self.scale + Vec2::splat(0.2)),
                source: Some(Rect::new(
                    sprite.x * self.sprite_dim.x,
                    sprite.y * self.sprite_dim.y,
                    self.sprite_dim.x,
                    self.sprite_dim.y,
                )),
                ..DrawTextureParams::default()
            },
        );
    }

    pub fn sprite_width(&self) -> f32 {
        self.sprite_dim.x * self.scale
    }

    pub fn sprite_height(&self) -> f32 {
        self.sprite_dim.y * self.scale
    }
}
