use nalgebra::{Vector2, Point2};

#[derive(Deserialize, Serialize)]
pub struct Tiles {
    tiles: Vec<Tile>,
    size: Vector2<i32>,
    changed: bool,
}

impl Tiles {
    pub fn empty(size: Vector2<i32>) -> Self {
        let amount = (size.x * size.y) as usize;
        let mut tiles = Vec::with_capacity(amount);
        for _ in 0..amount { tiles.push(Tile::empty()) }

        Tiles {
            tiles,
            size,
            changed: false,
        }
    }

    pub fn size(&self) -> Vector2<i32> {
        self.size
    }

    pub fn tile(&self, position: Point2<i32>) -> Result<&Tile, TilesError> {
        if self.is_in_bounds(position) {
            Ok(&self.tiles[self.index(position)])
        } else {
            Err(TilesError::OutOfBounds { position })
        }
    }

    pub fn tile_mut(&mut self, position: Point2<i32>) -> Result<&mut Tile, TilesError> {
        if self.is_in_bounds(position) {
            let index = self.index(position);
            Ok(&mut self.tiles[index])
        } else {
            Err(TilesError::OutOfBounds { position })
        }
    }

    pub fn is_in_bounds(&self, position: Point2<i32>) -> bool {
        position.x >= 0 && position.y >= 0
            && position.x < self.size.x && position.y < self.size.y
    }

    pub fn mark_changed(&mut self) {
        self.changed = true
    }

    pub fn check_changed(&mut self) -> bool {
        let val = self.changed;
        self.changed = false;
        val
    }

    fn index(&self, position: Point2<i32>) -> usize {
        (position.x + (position.y * self.size.x)) as usize
    }
}

#[derive(Debug, PartialEq)]
pub enum TilesError {
    OutOfBounds { position: Point2<i32> },
}

#[derive(Deserialize, Serialize)]
pub struct Tile {
    pub floor: bool,
    pub object: Option<ShipObject>,
}

impl Tile {
    pub fn empty() -> Self {
        Tile {
            floor: false,
            object: None,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct ShipObject {
    pub class: ShipObjectClassId,
}

impl ShipObject {
    pub fn new(class: ShipObjectClassId) -> Self {
        ShipObject {
            class,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct ShipObjectClassId(pub i32);
