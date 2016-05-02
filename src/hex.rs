pub struct HexCoordsAxial {
    pub col: i32,
    pub row: i32,
}

pub struct HexCoordsCube {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct HexCoords {
    col: i32,
    row: i32,
}

impl HexCoords {
    pub fn from_axial(coords: HexCoordsAxial) -> Self {
        HexCoords {
            row: coords.row,
            col: coords.col,
        }
    }
    pub fn as_axial(&self) -> HexCoordsAxial {
        HexCoordsAxial {
            row: self.row,
            col: self.col,
        }
    }
    pub fn from_cube(coords: HexCoordsCube) -> Self {
        HexCoords {
            row: coords.z,
            col: coords.x,
        }
    }

    pub fn as_cube(&self) -> HexCoordsCube {
        let x = self.col;
        let z = self.row;
        HexCoordsCube {
            x: x,
            y: -x - z,
            z: z,
        }
    }
}
