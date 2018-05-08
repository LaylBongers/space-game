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
}

impl<Tile> Tiles<Tile> {
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

    pub fn iter_pos(&self) -> IterPos {
        IterPos {
            size: self.size(),
            next_position: Point2::new(0, 0),
        }
    }

    pub fn bounds(&self, start: Point2<f32>, end: Point2<f32>) -> Bounds {
        let start_x = (start.x.floor() as i32).max(0);
        let start_y = (start.y.floor() as i32).max(0);
        let end_x = (end.x.ceil() as i32).min(self.size.x);
        let end_y = (end.y.ceil() as i32).min(self.size.y);

        Bounds {
            start: Point2::new(start_x, start_y),
            end: Point2::new(end_x, end_y),
        }
    }

    fn index(&self, position: Point2<i32>) -> usize {
        (position.x + (position.y * self.size.x)) as usize
    }
}

#[derive(Debug, PartialEq)]
pub enum Error {
    OutOfBounds { position: Point2<i32> },
}

pub struct IterPos {
    size: Vector2<i32>,
    next_position: Point2<i32>,
}

impl Iterator for IterPos {
    type Item = Point2<i32>;

    fn next(&mut self) -> Option<Point2<i32>> {
        let position = self.next_position;

        if position.y >= self.size.y {
            None
        } else {
            // Increment for next
            self.next_position.x += 1;
            if self.next_position.x >= self.size.x {
                self.next_position.x = 0;
                self.next_position.y += 1;
            }

            Some(position)
        }
    }
}

#[derive(Clone, Copy)]
pub struct Bounds {
    pub start: Point2<i32>,
    pub end: Point2<i32>,
}

impl Bounds {
    pub fn iter(self) -> IterBounds {
        IterBounds {
            bounds: self,
            next_position: self.start,
        }
    }
}

pub struct IterBounds {
    bounds: Bounds,
    next_position: Point2<i32>,
}

impl Iterator for IterBounds {
    type Item = Point2<i32>;

    fn next(&mut self) -> Option<Point2<i32>> {
        let position = self.next_position;

        if position.y >= self.bounds.end.y {
            None
        } else {
            // Increment for next
            self.next_position.x += 1;
            if self.next_position.x >= self.bounds.end.x {
                self.next_position.x = self.bounds.start.x;
                self.next_position.y += 1;
            }

            Some(position)
        }
    }
}
