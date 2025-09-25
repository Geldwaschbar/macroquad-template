use crate::spritesheet::Spritesheet;
use macroquad::prelude::*;

pub trait TilemapLoader {
    fn parse(&self, data: &[u8]) -> Tilemap;
}

#[derive(Debug)]
pub struct Tilemap {
    rows: Vec<Vec<Vec2>>,
}

impl Tilemap {
    pub fn from<T: TilemapLoader>(loader: T, data: &[u8]) -> Tilemap {
        loader.parse(data)
    }

    pub fn draw_area(&self, spritesheet: &Spritesheet, rect: Rect, pos: Vec2, scale: f32) {
        for x in (rect.x as usize)..(rect.w as usize) {
            for y in (rect.y as usize)..(rect.h as usize) {
                if let Some(sprite) = self.get_sprite(x, y) {
                    spritesheet.draw_sprite(
                        sprite,
                        pos + vec2(
                            (x as f32) * spritesheet.sprite_width() * scale,
                            (y as f32) * spritesheet.sprite_height() * scale,
                        ),
                        scale,
                    );
                }
            }
        }
    }

    pub fn get_sprite(&self, x: usize, y: usize) -> Option<&Vec2> {
        if let Some(row) = self.rows.get(y) {
            if let Some(v) = row.get(x) {
                return if v.x == 0.0 && v.y == 0.0 {
                    None
                } else {
                    Some(&v)
                };
            }
        }
        None
    }
}

pub mod pico8 {
    use crate::tilemap::{Tilemap, TilemapLoader};
    use macroquad::prelude::*;

    #[derive(Debug, Default)]
    pub struct Loader {}

    fn byte_to_float(byte: u8) -> f32 {
        assert!(byte.is_ascii_hexdigit());
        if byte >= b'0' && byte <= b'9' {
            return (byte - b'0') as f32;
        }
        if byte >= b'a' && byte <= b'f' {
            return (byte - b'a' + 10) as f32;
        }
        if byte >= b'A' && byte <= b'F' {
            return (byte - b'A' + 10) as f32;
        }
        panic!("could not convert byte to number");
    }

    impl TilemapLoader for Loader {
        fn parse(&self, data: &[u8]) -> Tilemap {
            let mut cols = Vec::new();
            let mut rows = Vec::new();
            let mut iter = data.iter();

            loop {
                if let Some(next) = iter.next() {
                    if *next == b'\n' {
                        rows.push(cols);
                        cols = Vec::new();
                    } else {
                        cols.push(vec2(
                            byte_to_float(*iter.next().expect("expected pair")),
                            byte_to_float(*next),
                        ));
                    }
                } else {
                    break;
                }
            }

            dbg!(rows.get(0));

            Tilemap { rows }
        }
    }
}
