use rand::{
    distributions::{Distribution, Standard},
    Rng
};
use crate::prelude::*;

const WORLD_INDEX_SIZE: usize = 55;
const NUM_TILES: usize = (MAP_WIDTH * MAP_HEIGHT) as usize;

fn coords_to_index(x: usize, y: usize) -> usize {
    ((y * WORLD_INDEX_SIZE) + x)
}

pub enum TileType {
    Grass1,
    Grass2,
    Grass3,
    Grass4,
}

impl Distribution<TileType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TileType {
        match rng.gen_range(0..=2) { // rand 0.8
            0 => TileType::Grass1,
            1 => TileType::Grass2,
            2 => TileType::Grass3,
            _ => TileType::Grass4,
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct Tile {
    pub index: usize,
}

impl Tile {
    fn from_tile_type(tile_type: TileType) -> Tile {
        match tile_type {
            TileType::Grass1 => Tile { index: coords_to_index(28, 33) },
            TileType::Grass2 => Tile { index: coords_to_index(29, 33) },
            TileType::Grass3 => Tile { index: coords_to_index(30, 33) },
            TileType::Grass4 => Tile { index: coords_to_index(31, 33) }
        }
    }
}

pub struct GameMap {
    pub tiles: Vec<Tile>
}

impl GameMap {
    pub fn new() -> Self {
        let mut rand_tiles: Vec<Tile> = Vec::new();
        for i in 0..NUM_TILES {
            rand_tiles.push(Tile::from_tile_type(rand::random()));
        }
        Self{
            tiles: rand_tiles
        }
    }

    fn map_index(x: usize, y: usize) -> usize {
        ((y * MAP_HEIGHT) + x) as usize
    }

     pub fn index_to_map(index: usize) -> (usize, usize) {
        let x = index % MAP_WIDTH;
        let y = index / MAP_WIDTH;
        (x, y)
    }
}