use nalgebra::{Vector2};
use slog::{Logger};

use model::ship::{Tiles, Unit, JobQueue};

pub struct Ship {
    units: Vec<Unit>,

    pub tiles: Tiles,
    pub job_queue: JobQueue,
}

impl Ship {
    pub fn empty(size: Vector2<i32>) -> Self {
        Ship {
            units: Vec::new(),

            tiles: Tiles::empty(size),
            job_queue: JobQueue::new(),
        }
    }

    pub fn units(&self) -> &Vec<Unit> {
        &self.units
    }

    pub fn add_unit(&mut self, unit: Unit) {
        self.units.push(unit);
    }

    pub fn update(&mut self, log: &Logger, delta: f32) {
        for unit in &mut self.units {
            unit.update(log, delta, &mut self.tiles, &mut self.job_queue);
        }

        self.job_queue.update(log);
    }
}
