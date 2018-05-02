use {
    nalgebra::{Vector2, Point2},
    Event,
};

#[derive(Deserialize, Serialize)]
pub struct Tiles<Tile> {
    tiles: Vec<Tile>,
    size: Vector2<i32>,

    pub changed: Event,
}

impl<Tile: Default> Tiles<Tile> {
    pub fn empty(size: Vector2<i32>) -> Self {
        let amount = (size.x * size.y) as usize;
        let mut tiles = Vec::with_capacity(amount);
        for _ in 0..amount { tiles.push(Tile::default()) }

        Tiles {
            tiles,
            size,

            changed: Event::new(),
        }
    }

    pub fn size(&self) -> Vector2<i32> {
        self.size
    }

    pub fn get(&self, position: Point2<i32>) -> Result<&Tile, Error> {
        if self.is_in_bounds(position) {
            Ok(&self.tiles[self.index(position)])
        } else {
            Err(Error::OutOfBounds { position })
        }
    }

    pub fn get_mut(&mut self, position: Point2<i32>) -> Result<&mut Tile, Error> {
        if self.is_in_bounds(position) {
            let index = self.index(position);
            Ok(&mut self.tiles[index])
        } else {
            Err(Error::OutOfBounds { position })
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
pub enum Error {
    OutOfBounds { position: Point2<i32> },
}
