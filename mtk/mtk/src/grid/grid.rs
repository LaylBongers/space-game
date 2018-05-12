use {
    nalgebra::{Point2},

    grid::{Dim, Dim2},
    Event,
};

#[derive(Deserialize, Serialize)]
pub struct Grid<Tile, D: Dim> {
    tiles: Vec<Tile>,
    size: D::Vector,

    pub changed: Event,
}

impl<Tile: Default, D: Dim> Grid<Tile, D> {
    pub fn empty(size: D::Vector) -> Self {
        let amount = D::area(size);
        let mut tiles = Vec::with_capacity(amount);
        for _ in 0..amount { tiles.push(Tile::default()) }

        Grid {
            tiles,
            size,

            changed: Event::new(),
        }
    }
}

impl<Tile, D: Dim> Grid<Tile, D> {
    pub fn size(&self) -> D::Vector {
        self.size
    }

    pub fn get(&self, position: D::Point) -> Result<&Tile, Error> {
        if D::is_in_bounds(position, self.size) {
            Ok(&self.tiles[D::index(position, self.size)])
        } else {
            Err(Error::OutOfBounds)
        }
    }

    pub fn get_mut(&mut self, position: D::Point) -> Result<&mut Tile, Error> {
        if D::is_in_bounds(position, self.size) {
            let index = D::index(position, self.size);
            Ok(&mut self.tiles[index])
        } else {
            Err(Error::OutOfBounds)
        }
    }

    pub fn is_in_bounds(&self, position: D::Point) -> bool {
        D::is_in_bounds(position, self.size)
    }

    pub fn iter_pos(&self) -> IterPos<D> {
        IterPos {
            size: self.size,
            next_position: Some(D::start()),
        }
    }
}

impl<Tile> Grid<Tile, Dim2> {
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
}

#[derive(Debug, PartialEq)]
pub enum Error {
    OutOfBounds,
}

pub struct IterPos<D: Dim> {
    size: D::Vector,
    next_position: Option<D::Point>,
}

impl<D: Dim> Iterator for IterPos<D> {
    type Item = D::Point;

    fn next(&mut self) -> Option<D::Point> {
        let position = self.next_position;

        if let Some(position) = position {
            self.next_position = D::next(position, self.size);
        }

        position
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
