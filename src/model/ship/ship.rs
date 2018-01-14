use nalgebra::{Vector2};
use slog::{Logger};

use model::ship::{Tiles, Unit, JobQueue};

pub struct Ship {
    tiles: Tiles,
    units: Vec<Unit>,
    job_queue: JobQueue,
}

impl Ship {
    pub fn empty(size: Vector2<i32>) -> Self {
        Ship {
            tiles: Tiles::empty(size),
            units: Vec::new(),
            job_queue: JobQueue::new(),
        }
    }

    pub fn tiles(&self) -> &Tiles {
        &self.tiles
    }

    pub fn tiles_mut(&mut self) -> &mut Tiles {
        &mut self.tiles
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

    pub fn update(&mut self, log: &Logger, delta: f32) {
        for unit in &mut self.units {
            unit.update(log, delta, &mut self.tiles, &mut self.job_queue);
        }

        self.job_queue.update(log);
    }
}
