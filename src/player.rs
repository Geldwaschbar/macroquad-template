use macroquad::prelude::*;
use macroquad::{
    camera::Camera2D,
    input::is_key_down,
    math::Vec2,
    shapes::draw_circle,
    window::{screen_height, screen_width},
};

use crate::{spritesheet::Spritesheet, tilemap::Tilemap};

const PLAYER_SPEED: f32 = 1.15;
const PLAYER_ZOOM: f32 = 0.02;

#[derive(Debug)]
pub struct Player {
    camera: Camera2D,
    pub position: Vec2,
}

impl Player {
    pub fn new() -> Player {
        Player {
            camera: Camera2D {
                zoom: Vec2::new(PLAYER_ZOOM, PLAYER_ZOOM * screen_width() / screen_height()),
                ..Default::default()
            },
            position: Vec2::ZERO,
        }
    }

    pub fn move_to(&mut self, next: Vec2) {
        self.position = next;
        self.camera.zoom = Vec2::new(PLAYER_ZOOM, PLAYER_ZOOM * screen_width() / screen_height());
        self.camera.target = next.floor();
    }

    pub fn movement(&mut self, spritesheet: &Spritesheet, tilemap: &Tilemap) {
        let mut diff = Vec2::ZERO;
        if is_key_down(KeyCode::W) {
            diff.y -= PLAYER_SPEED;
        }
        if is_key_down(KeyCode::A) {
            diff.x -= PLAYER_SPEED;
        }
        if is_key_down(KeyCode::S) {
            diff.y += PLAYER_SPEED;
        }
        if is_key_down(KeyCode::D) {
            diff.x += PLAYER_SPEED;
        }
        let next = self.position + diff;
        if let Some(tile) = tilemap.get_rel_tile(spritesheet, next.x, next.y) {
            if tile.is_free() {
                self.move_to(next);
            }
        }
    }

    pub fn draw(&self) {
        draw_circle(self.camera.target.x, self.camera.target.y, 4.0, RED);
    }

    pub fn get_camera(&self) -> &Camera2D {
        &self.camera
    }

    pub fn get_viewport(&self, spritesheet: &Spritesheet) -> Rect {
        let x = (&self.position.x / spritesheet.sprite_width()).floor();
        let y = (&self.position.y / spritesheet.sprite_height()).floor();
        let w = (screen_width() / spritesheet.sprite_width()).ceil();
        let h = (screen_height() / spritesheet.sprite_height()).ceil();
        Rect::new((x - w / 2.0).floor(), (y - h / 2.0).floor(), w, h)
    }
}
