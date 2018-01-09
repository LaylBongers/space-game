use std::collections::{HashMap};
use nalgebra::{Vector2, Point2};

pub struct Ship {
    tiles: Vec<Tile>,
    size: Vector2<i32>,

    jobs: HashMap<i32, Job>,
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
        let id = JobId(self.next_job_id);
        let job = Job::new(position, 1.0);
        self.jobs.insert(self.next_job_id, job);

        // Also add the job to the tile
        self.tile_mut(position)?.build_job = Some(id);

        self.next_job_id += 1;
        Ok(())
    }

    pub fn dequeue_job(&mut self, id: JobId) -> Result<(), ShipError> {
        let job = self.jobs.remove(&id.0)
            .ok_or(ShipError::InvalidJobId { id })?;

        // Also remove the job from the tile
        self.tile_mut(job.position).unwrap().build_job = None;

        Ok(())
    }

    fn index(&self, position: Point2<i32>) -> usize {
        (position.x + (position.y * self.size.x)) as usize
    }
}

#[derive(Debug, PartialEq)]
pub enum ShipError {
    OutOfBounds { position: Point2<i32> },
    InvalidJobId { id: JobId }
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

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct JobId(i32);

pub struct Job {
    position: Point2<i32>,
    work_done: f32,
    work_target: f32,
}

impl Job {
    pub fn new(position: Point2<i32>, work_target: f32) -> Self {
        Job {
            position,
            work_done: 0.0,
            work_target,
        }
    }
}
