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

pub struct GameTile {
    content: TileContent,
    team: Team
}

pub mod map {
    use std::collections::HashMap;
    use hex::HexCoordsAxial;
    use super::GameTile;

    pub trait GameMap {
        fn get(&self, &HexCoordsAxial) -> Option<&GameTile>;
    }

    pub fn new() -> HashGameMap {
        HashGameMap {
            tile_hash_map: HashMap::new()
        }
    }

    pub struct HashGameMap {
        tile_hash_map: HashMap<HexCoordsAxial, GameTile>
    }

    impl GameMap for HashGameMap {
        fn get(&self, coords: &HexCoordsAxial) -> Option<&GameTile> {
            self.tile_hash_map.get(&coords)
        }
    }

}
