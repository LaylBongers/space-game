use std::collections::{HashMap};
use nalgebra::{Vector2, Point2};

pub struct Ship {
    tiles: Vec<Tile>,
    size: Vector2<i32>,

    jobs: HashMap<i32, Point2<i32>>,
    next_job_id: i32,
}

impl Ship {
    pub fn empty(size: Vector2<i32>) -> Self {
        let amount = (size.x * size.y) as usize;
        let mut tiles = Vec::with_capacity(amount);
        for _ in 0..amount { tiles.push(Tile::empty()) }

        Ship {
            tiles,
            size,

            jobs: HashMap::new(),
            next_job_id: 0,
        }
    }

    pub fn size(&self) -> Vector2<i32> {
        self.size
    }

    pub fn tile(&self, position: Point2<i32>) -> Result<&Tile, ShipError> {
        if self.is_in_bounds(position) {
            Ok(&self.tiles[self.index(position)])
        } else {
            Err(ShipError::OutOfBounds { position })
        }
    }

    pub fn tile_mut(&mut self, position: Point2<i32>) -> Result<&mut Tile, ShipError> {
        if self.is_in_bounds(position) {
            let index = self.index(position);
            Ok(&mut self.tiles[index])
        } else {
            Err(ShipError::OutOfBounds { position })
        }
    }

    pub fn is_in_bounds(&self, position: Point2<i32>) -> bool {
        position.x >= 0 && position.y >= 0
            && position.x < self.size.x && position.y < self.size.y
    }

    pub fn queue_job(&mut self, position: Point2<i32>) -> Result<(), ShipError> {
        self.tile_mut(position)?.jobs += 1;
        self.jobs.insert(self.next_job_id, position);
        self.next_job_id += 1;

        Ok(())
    }

    fn index(&self, position: Point2<i32>) -> usize {
        (position.x + (position.y * self.size.x)) as usize
    }
}

#[derive(Debug, PartialEq)]
pub enum ShipError {
    OutOfBounds { position: Point2<i32> },
}

pub struct Tile {
    pub floor: bool,
    pub object: Option<ShipObject>,

    /// Counts how many jobs there are on this tile, do not edit this directly, this is managed by
    /// Ship's job queueing.
    pub jobs: i32,
}

impl Tile {
    pub fn empty() -> Self {
        Tile {
            floor: false,
            object: None,
            jobs: 0,
        }
    }
}

pub struct ShipObject {
}

impl ShipObject {
    pub fn new() -> Self {
        ShipObject {
        }
    }
}

pub struct Job {
    work_done: f32,
    work_target: f32,
}

impl Job {
    pub fn new(work_target: f32) -> Self {
        Job {
            work_done: 0.0,
            work_target,
        }
    }
}
