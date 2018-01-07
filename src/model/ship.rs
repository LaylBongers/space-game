use nalgebra::{Vector2, Point2};

pub struct Ship {
    tiles: Vec<Tile>,
    size: Vector2<i32>,
}

impl Ship {
    pub fn empty(size: Vector2<i32>) -> Self {
        Ship {
            tiles: vec![Tile::empty(); (size.x * size.y) as usize],
            size,
        }
    }

    pub fn size(&self) -> Vector2<i32> {
        self.size
    }

    pub fn tile(&self, position: Point2<i32>) -> Result<&Tile, MapError> {
        if self.is_in_bounds(position) {
            Ok(&self.tiles[self.index(position)])
        } else {
            Err(MapError::OutOfBounds { position })
        }
    }

    pub fn tile_mut(&mut self, position: Point2<i32>) -> Result<&mut Tile, MapError> {
        if self.is_in_bounds(position) {
            let index = self.index(position);
            Ok(&mut self.tiles[index])
        } else {
            Err(MapError::OutOfBounds { position })
        }
    }

    pub fn is_in_bounds(&self, position: Point2<i32>) -> bool {
        position.x >= 0 && position.y >= 0
            && position.x < self.size.x && position.y < self.size.y
    }

    fn index(&self, position: Point2<i32>) -> usize {
        (position.x + (position.y * self.size.x)) as usize
    }
}

#[derive(Debug, PartialEq)]
pub enum MapError {
    OutOfBounds { position: Point2<i32> },
}

#[derive(Clone)]
pub struct Tile {
    pub floor: bool,
}

impl Tile {
    pub fn empty() -> Self {
        Tile {
            floor: false,
        }
    }
}
