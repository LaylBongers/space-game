use nalgebra::{Point2, Vector2};
use slog::{Logger};

use {ObjectClasses};
use state::ship::{Tiles, Unit, TaskQueue};

#[derive(Deserialize, Serialize)]
pub struct Ship {
    units: Vec<Unit>,

    pub tiles: Tiles,
    pub task_queue: TaskQueue,
}

impl Ship {
    pub fn empty(size: Vector2<i32>) -> Self {
        Ship {
            units: Vec::new(),

            tiles: Tiles::empty(size),
            task_queue: TaskQueue::new(),
        }
    }

    pub fn starter(log: &Logger) -> Self {
        info!(log, "Creating starter ship");
        let mut ship = Ship::empty(Vector2::new(100, 100));
        for y in 47..53 {
            for x in 48..52 {
                ship.tiles.get_mut(Point2::new(x, y)).unwrap().floor = true;
            }
        }
        ship.add_unit(Unit::new(Point2::new(50.5, 50.5)));
        ship.add_unit(Unit::new(Point2::new(49.5, 49.5)));

        ship
    }

    pub fn units(&self) -> &Vec<Unit> {
        &self.units
    }

    pub fn add_unit(&mut self, unit: Unit) {
        self.units.push(unit);
    }

    pub fn update(&mut self, log: &Logger, object_classes: &ObjectClasses, delta: f32) {
        if self.tiles.handle_changed(object_classes) {
            // Since the world has changed, we can mark all tasks as being possible again
            self.task_queue.clear_unreachable();
        }

        self.tiles.update(object_classes, delta);

        for unit in &mut self.units {
            unit.update(log, object_classes, &mut self.tiles, &mut self.task_queue, delta);
        }

        self.task_queue.update(log);
    }
}
