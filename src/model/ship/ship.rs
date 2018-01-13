use nalgebra::{Vector2, Point2};

use model::ship::{Unit, JobQueue, JobId};

pub struct Ship {
    tiles: Vec<Tile>,
    size: Vector2<i32>,

    units: Vec<Unit>,
    job_queue: JobQueue,
}

impl Ship {
    pub fn empty(size: Vector2<i32>) -> Self {
        let amount = (size.x * size.y) as usize;
        let mut tiles = Vec::with_capacity(amount);
        for _ in 0..amount { tiles.push(Tile::empty()) }

        Ship {
            tiles,
            size,

            units: Vec::new(),
            job_queue: JobQueue::new(),
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

    pub fn units(&self) -> &Vec<Unit> {
        &self.units
    }

    pub fn add_unit(&mut self, unit: Unit) {
        self.units.push(unit);
    }

    pub fn job_queue(&self) -> &JobQueue {
        &self.job_queue
    }

    pub fn job_queue_mut(&mut self) -> &mut JobQueue {
        &mut self.job_queue
    }

    pub fn update(&mut self, delta: f32) {
        for unit in &mut self.units {
            unit.update(delta, &mut self.job_queue);
        }
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

    /// Marks if there is a build job on this tile, do not edit this directly, this is managed by
    /// Ship's job queueing.
    pub build_job: Option<JobId>,
}

impl Tile {
    pub fn empty() -> Self {
        Tile {
            floor: false,
            object: None,
            build_job: None,
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
