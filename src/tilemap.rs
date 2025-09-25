use crate::spritesheet::Spritesheet;
use macroquad::prelude::*;

pub trait TilemapLoader {
    fn parse(&self, data: &[u8]) -> Tilemap;
}

#[derive(Debug, Default)]
pub struct Tile {
    sprite: Vec2,
    is_solid: bool,
}

impl Tile {
    pub fn new(sprite: Vec2, is_solid: bool) -> Tile {
        Tile { sprite, is_solid }
    }
}

#[derive(Debug)]
pub struct Tilemap {
    /// Map is the matrix of all tile id's.
    /// A tile id is the index of the `tiles` vec.
    map: Vec<Vec<Option<usize>>>,
    /// Tiles is the vec that contains both the sprites and flags.
    tiles: Vec<Tile>,
}

impl Tilemap {
    pub fn from<T: TilemapLoader>(loader: T, data: &[u8]) -> Tilemap {
        loader.parse(data)
    }

    pub fn draw_area(&self, spritesheet: &Spritesheet, rect: Rect, pos: Vec2, scale: f32) {
        for x in (rect.x as usize)..(rect.w as usize) {
            for y in (rect.y as usize)..(rect.h as usize) {
                if let Some(tile) = self.get_tile(x, y) {
                    spritesheet.draw_sprite(
                        &tile.sprite,
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

    pub fn get_tile(&self, x: usize, y: usize) -> Option<&Tile> {
        if let Some(row) = self.map.get(y) {
            if let Some(tile) = row.get(x) {
                if let Some(id) = tile {
                    return self.tiles.get(*id);
                }
            }
        }
        // Outside of map
        None
    }
}

pub mod pico8 {
    use crate::tilemap::{Tile, Tilemap, TilemapLoader};
    use macroquad::prelude::*;
    use std::collections::HashMap;

    #[derive(Debug, Default)]
    pub struct Loader {}

    fn byte_to_hex(byte: u8) -> i8 {
        assert!(byte.is_ascii_hexdigit());
        if byte >= b'0' && byte <= b'9' {
            return (byte - b'0') as i8;
        }
        if byte >= b'a' && byte <= b'f' {
            return (byte - b'a' + 10) as i8;
        }
        if byte >= b'A' && byte <= b'F' {
            return (byte - b'A' + 10) as i8;
        }
        panic!("could not convert byte to number");
    }

    impl TilemapLoader for Loader {
        fn parse(&self, data: &[u8]) -> Tilemap {
            let mut pos2id = HashMap::new();
            let mut map = Vec::new();
            let mut tiles = Vec::new();
            let mut col = Vec::new();
            let mut iter = data.iter();

            loop {
                if let Some(next) = iter.next() {
                    if *next == b'\n' {
                        map.push(col);
                        col = Vec::new();
                    } else {
                        let (x, y) = (
                            byte_to_hex(*iter.next().expect("expected pair")),
                            byte_to_hex(*next),
                        );
                        if x == 0 && y == 0 {
                            col.push(None)
                        } else {
                            let tile_id = {
                                if let Some(id) = pos2id.get(&(x, y)) {
                                    *id
                                } else {
                                    tiles.push(Tile::new(Vec2::new(x as f32, y as f32), false));
                                    let tile_id = tiles.len() - 1;
                                    pos2id.insert((x, y), tile_id);
                                    tile_id
                                }
                            };
                            col.push(Some(tile_id));
                        }
                    }
                } else {
                    break;
                }
            }

            Tilemap { map, tiles }
        }
    }
}
