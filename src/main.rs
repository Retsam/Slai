#![allow(dead_code)]

use std::collections::HashMap;

//Grid stuff

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct HexCoordsAxial {
    col: i32,
    row: i32
}

//Map stuff

struct GameMap {
    _tile_hash_map: HashMap<HexCoordsAxial, GameTile>
}

impl GameMap {
    fn new() -> GameMap {
        GameMap {
            _tile_hash_map:  HashMap::new()
        }
    }
}

enum TileContent {
    Empty,
    //Trees
    CoastTree,
    Tree,
    //Buildings
    Hut,
    Castle,
    //People
    Peasant,
    Knight,
    Lord,
    King
}

enum Team {
    One,
    Two,
    Three,
    Four,
    Five,
    Six
}

struct GameTile {
    content: TileContent,
    team: Team
}

fn main() {
    GameMap::new();
    println!("Hello World");
}
