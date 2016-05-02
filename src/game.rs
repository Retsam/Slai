enum TileContent {
    Empty,
    // Trees
    CoastTree,
    Tree,
    // Buildings
    Hut,
    Castle,
    // People
    Peasant,
    Knight,
    Lord,
    King,
}

enum Team {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

pub struct GameTile {
    content: TileContent,
    team: Team,
}

pub mod map {
    use std::collections::HashMap;
    use hex::HexCoords;
    use super::GameTile;

    pub trait GameMap {
        fn get(&self, &HexCoords) -> Option<&GameTile>;
    }

    pub fn new() -> HashGameMap {
        HashGameMap { tile_hash_map: HashMap::new() }
    }

    pub struct HashGameMap {
        tile_hash_map: HashMap<HexCoords, GameTile>,
    }

    impl GameMap for HashGameMap {
        fn get(&self, coords: &HexCoords) -> Option<&GameTile> {
            self.tile_hash_map.get(&coords)
        }
    }

}
