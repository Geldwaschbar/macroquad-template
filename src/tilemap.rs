use crate::spritesheet::Spritesheet;
use macroquad::prelude::*;

pub trait TilemapLoader {
    fn parse(&mut self) -> Tilemap;
}

#[derive(Debug, Default)]
pub struct Tile {
    sprite: Vec2,
    flag: u8,
}

impl Tile {
    pub fn new(sprite: Vec2, flag: u8) -> Tile {
        Tile { sprite, flag }
    }

    pub fn is_free(&self) -> bool {
        (&self.flag & 1) > 0
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
    pub fn from<T: TilemapLoader>(loader: &mut T) -> Tilemap {
        loader.parse()
    }

    pub fn draw_area(&self, spritesheet: &Spritesheet, rect: Rect, pos: Vec2) {
        for x in (rect.x as usize)..((rect.x + rect.w) as usize) {
            for y in (rect.y as usize)..((rect.y + rect.h) as usize) {
                if let Some(tile) = self.get_abs_tile(x, y) {
                    spritesheet.draw_sprite(
                        &tile.sprite,
                        pos + vec2(
                            (x as f32) * spritesheet.sprite_width(),
                            (y as f32) * spritesheet.sprite_height(),
                        ),
                    );
                }
            }
        }
    }

    /// Returns the tile based on the absolut index position of the map.
    pub fn get_abs_tile(&self, x: usize, y: usize) -> Option<&Tile> {
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

    pub fn get_rel_tile(&self, spritesheet: &Spritesheet, x: f32, y: f32) -> Option<&Tile> {
        self.get_abs_tile(
            (x / spritesheet.sprite_width()) as usize,
            (y / spritesheet.sprite_height()) as usize,
        )
    }
}

pub mod pico8 {
    use crate::tilemap::{Tile, Tilemap, TilemapLoader};
    use macroquad::prelude::*;
    use std::slice::Iter;

    #[derive(Debug)]
    pub struct Loader<'a> {
        iter: Iter<'a, u8>,
        line: usize,
        col: usize,
    }

    impl<'a> Loader<'a> {
        pub fn new(data: &'a [u8]) -> Loader<'a> {
            Loader {
                iter: data.iter(),
                line: 1,
                col: 0,
            }
        }

        fn read_byte(&mut self) -> u8 {
            (Self::byte_to_hex(self.advance()) << 4) | Self::byte_to_hex(self.advance())
        }

        fn skip_line(&mut self) {
            while self.advance() != b'\n' {}
            self.next_line();
        }

        fn next_line(&mut self) {
            self.line += 1;
            self.col = 0;
        }

        fn advance(&mut self) -> u8 {
            self.col += 1;
            *self.iter.next().expect("expect token")
        }

        fn consume_eol(&mut self) {
            let got = self.advance();
            assert_eq!(
                got, b'\n',
                "got char '{}' but expected '\\n' at {}:{}",
                got as char, self.line, self.col
            );
            self.next_line();
        }

        fn byte_to_hex(byte: u8) -> u8 {
            assert!(byte.is_ascii_hexdigit(), "{}", byte as char);
            if byte >= b'0' && byte <= b'9' {
                return byte - b'0';
            }
            if byte >= b'a' && byte <= b'f' {
                return byte - b'a' + 10;
            }
            if byte >= b'A' && byte <= b'F' {
                return byte - b'A' + 10;
            }
            panic!("could not convert byte to number");
        }
    }

    impl TilemapLoader for Loader<'_> {
        fn parse(&mut self) -> Tilemap {
            let mut map = Vec::new();
            let mut tiles = Vec::with_capacity(256);

            // Skip __gff__ label
            self.skip_line();
            for i in 0..256 {
                let flag = self.read_byte();
                tiles.push(Tile::new(Vec2::new((i % 16) as f32, (i / 16) as f32), flag))
            }
            self.consume_eol();

            // Skip __map__ label
            self.skip_line();
            for _ in 0..31 {
                let mut col = Vec::with_capacity(128);
                for _ in 0..128 {
                    let id = self.read_byte();
                    if id == 0 {
                        col.push(None)
                    } else {
                        col.push(Some(id as usize));
                    }
                }
                map.push(col);
                self.consume_eol();
            }

            Tilemap { map, tiles }
        }
    }
}
