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

    pub fn tile(&self, position: Point2<i32>) -> &Tile {
        &self.tiles[self.index(position)]
    }

    pub fn tile_mut(&mut self, position: Point2<i32>) -> &mut Tile {
        let index = self.index(position);
        &mut self.tiles[index]
    }

    fn index(&self, position: Point2<i32>) -> usize {
        (position.x + (position.y * self.size.x)) as usize
    }
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
