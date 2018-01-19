use alga::linear::{EuclideanSpace};
use nalgebra::{Point2};
use slog::{Logger};

use model::ship::{TaskId, TaskQueue, ShipObject, Tiles};

pub struct Unit {
    position: Point2<f32>,
    assigned_task: Option<TaskId>,

    /// The path we're currently following.
    path: Option<Vec<Point2<i32>>>,
}

impl Unit {
    pub fn new(position: Point2<f32>) -> Self {
        Unit {
            position,
            assigned_task: None,

            path: None,
        }
    }

    pub fn position(&self) -> Point2<f32> {
        self.position
    }

    pub fn update(
        &mut self, log: &Logger, delta: f32, tiles: &mut Tiles, task_queue: &mut TaskQueue
    ) {
        self.update_task(log, delta, tiles, task_queue);
        self.update_movement(delta);
    }

    fn update_task(
        &mut self, log: &Logger, delta: f32, tiles: &mut Tiles, task_queue: &mut TaskQueue
    ) {
        // A lot of the functionality in here is sequential steps to complete a task checked every
        // frame. For performance it may be beneficial to restructure it into something else that
        // lets a unit sequantially go through the actions needed (find task -> move to -> work).

        // Try to find a task to do if we don't have one yet, or the old one was removed
        if let Some(task) = self.assigned_task.and_then(|j| task_queue.task_mut(j)) {
            let task_center = Point2::new(
                task.position().x as f32 + 0.5,
                task.position().y as f32 + 0.5
            );

            // If we're still path following, don't do anything
            if self.path.is_some() {
                return
            }

            // Check if we're at the destination
            if self.position.distance_squared(&task_center) < 1.0 * 1.0 {
                // We're there, apply work
                task.apply_work(delta);

                // If the work's done, we can add an object to the tile
                if task.is_done() {
                    tiles.tile_mut(task.position()).unwrap().object = Some(ShipObject::new());
                }
            } else {
                // We're not there yet, go to the task
                self.path = Some(vec!(task.position()));
            }

            return
        }

        // We don't have a task, stand in place while finding a new one
        self.assigned_task = task_queue.assign_task(log, self.position);
        self.path = None;
    }

    fn update_movement(&mut self, delta: f32) {
        // Update the path we're following
        let target = if let Some(ref path) = self.path {
            *path.iter().last().unwrap()
        } else {
            return
        };

        let speed = 1.5;
        let target = Point2::new(target.x as f32 + 0.5, target.y as f32 + 0.5);

        // Calculate how far away we are and how far we can travel
        let distance = self.position.distance(&target);
        let distance_possible = speed * delta;

        // If we're within our travel distance, just arrive
        if distance < distance_possible {
            self.path.as_mut().unwrap().pop();
            self.position = target;
        } else {
            // If not, travel closer
            let difference = target - self.position;
            let move_amount = difference / distance * distance_possible;
            self.position += move_amount;
        }

        // If the path's at the end, we can clear it
        if self.path.as_ref().unwrap().is_empty() {
            self.path = None
        }
    }
}
