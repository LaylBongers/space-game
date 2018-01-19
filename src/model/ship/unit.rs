use alga::linear::{EuclideanSpace};
use nalgebra::{Point2};
use slog::{Logger};
use pathfinding;

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
            if (task_center.x - self.position.x).abs() < 1.1 &&
               (task_center.y - self.position.y).abs() < 1.1 {
                // We're there, apply work
                task.apply_work(delta);

                // If the work's done, we can add an object to the tile
                if task.is_done() {
                    tiles.tile_mut(task.position()).unwrap().object = Some(ShipObject::new());
                }
            } else {
                // We're not there, find a path to our destination

                // Calculate some advance values relevant to pathfinding
                let straight = 100; // Can only use Ord cost, f32 isn't Ord
                let diagonal = (f32::sqrt(2.0) * straight as f32) as i32;
                let mut start = Point2::new(self.position.x as i32, self.position.y as i32);
                let mut goal = task.position();

                // Our path following wants the path in reverse
                ::std::mem::swap(&mut start, &mut goal);

                // Now do the actual pathfinding
                let result = pathfinding::astar(
                    &start,
                    |node| {
                        let mut neighbors = Vec::new();

                        for y in node.y-1..node.y+2 {
                            for x in node.x-1..node.x+2 {
                                let neighbor = Point2::new(x, y);

                                // If this is the same one, skip it
                                if *node == neighbor {
                                    continue
                                }

                                // Retrieve the tile data itself, if we can't, bail
                                let tile = if let Ok(tile) = tiles.tile(neighbor) {
                                    tile
                                } else {
                                    continue
                                };

                                // Make sure we can walk over this tile, but we always allow the
                                // goal because that's where we're moving form and even if it's
                                // blocked it might move out
                                // Check the performance if we don't include the start there and
                                // instead reverse the path after the fact
                                if (!tile.floor || tile.object.is_some()) && neighbor != goal {
                                    continue
                                }

                                // Diagonal costs are somewhat bigger
                                let cost = if x == node.x || y == node.y {
                                    straight
                                } else {
                                    diagonal
                                };

                                neighbors.push((neighbor, cost));
                            }
                        }

                        neighbors
                    },
                    |node| {
                        let dx = (node.x - goal.x).abs();
                        let dy = (node.y - goal.y).abs();
                        straight*(dx + dy) + (diagonal - 2*straight) * dx.min(dy)
                    },
                    |node| *node == goal,
                );

                if let Some((mut path, _cost)) = result {
                    // We found a path, remove the first entry, we want to stop next to the goal
                    // not on it
                    path.remove(0);
                    self.path = Some(path)
                } else {
                    // We didn't find a path, perhaps mark this task unreachable?
                }
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
