use crate::prelude::*;

const WORLD_INDEX_SIZE: usize = 55;

fn coords_to_index(x: u16, y: u16) -> usize {
    ((y * WORLD_INDEX_SIZE) + x) as usize
}

pub enum TileType {
    Grass1,
    Grass2,
    Grass3,
    Grass4,
}

struct Tile {
    index: usize,
}

impl Tile {
    fn from_tile_type(tile_type: TileType) -> Tile {
        match tile_type {
            TileType::Grass1 => Tile { index: coords_to_index(28, 12) },
            TileType::Grass2 => Tile { index: coords_to_index(29, 12) },
            TileType::Grass3 => Tile { index: coords_to_index(30, 12) },
            TileType::Grass4 => Tile { index: coords_to_index(31, 12) }
        }
    }
}

struct Map {

}