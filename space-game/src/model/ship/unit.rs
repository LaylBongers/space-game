use alga::linear::{EuclideanSpace};
use nalgebra::{Point2};
use slog::{Logger};
use pathfinding;

use model::ship::{TaskId, TaskQueue, ShipObject, Tiles, Tile, TilesError};

#[derive(Deserialize, Serialize)]
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

        let mut got_task = false;

        // Try to find a task to do if we don't have one yet, or the old one was removed
        if let Some((task_id, task)) = self.assigned_task
            .and_then(|task_id| task_queue.task_mut(task_id).map(|task| (task_id, task)))
        {
            got_task = true;

            // If we're still path following, don't do anything
            if self.path.is_some() {
                return
            }

            let task_center = Point2::new(
                task.position().x as f32 + 0.5,
                task.position().y as f32 + 0.5
            );

            // Check if we're at the destination
            if (task_center.x - self.position.x).abs() < 1.1 &&
               (task_center.y - self.position.y).abs() < 1.1 {
                // We're there, apply work
                task.apply_work(delta);

                // If the work's done, we can add an object to the tile
                if task.is_done() {
                    tiles.tile_mut(task.position()).unwrap().object = Some(ShipObject::new());
                    tiles.mark_changed();
                }
            } else {
                // We're not there, find a path to our destination
                if !self.path_to(task.position(), tiles, false) {
                    // We couldn't find a path, mark the task as unreachable
                    task.set_unreachable(true);
                    task.set_assigned(false);
                    self.assigned_task = None;

                    info!(log, "Unassigned task {}, it's unreachable", task_id.0);
                }
            }
        }

        if !got_task {
            // We don't have a task, or the task we had is gone, wait while finding a new one
            self.assigned_task = task_queue.assign_task(log, self.position);
            self.path = None;
        }
    }

    /// Finds a path to the goal, returns false if no path could be found.
    fn path_to(&mut self, goal: Point2<i32>, tiles: &Tiles, goal_inclusive: bool) -> bool {
        // Calculate some advance values relevant to pathfinding
        let costs = Costs {
            straight: 100,
            diagonal: (f32::sqrt(2.0) * 100.0) as i32,
        };
        let start = Point2::new(self.position.x as i32, self.position.y as i32);

        // Now do the actual pathfinding
        // Keep in mind our path following wants the path in reverse
        let result = pathfinding::astar(
            &goal,
            |node| neighbors(*node, start, goal, goal_inclusive, tiles, &costs),
            |node| heuristic(*node, start, &costs),
            |node| *node == start,
        );

        if let Some((mut path, _cost)) = result {
            if !goal_inclusive {
                path.remove(0);
            }
            self.path = Some(path);

            // Everything's set for path following, return that we found a path
            true
        } else {
            // We didn't find a path, return that this is unreachable
            false
        }
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

struct Costs {
    // Can only use Ord cost, f32 isn't Ord
    straight: i32,
    diagonal: i32,
}

fn neighbors(
    node: Point2<i32>, start: Point2<i32>, goal: Point2<i32>,
    goal_inclusive: bool, tiles: &Tiles, costs: &Costs
) -> Vec<(Point2<i32>, i32)> {
    let mut neighbors = Vec::new();

    for y in node.y-1..node.y+2 {
        for x in node.x-1..node.x+2 {
            let neighbor = Point2::new(x, y);

            // If this is the same one, skip it
            if node == neighbor {
                continue
            }

            // Retrieve the tile data itself
            let tile_res =  tiles.tile(neighbor);

            // Make sure we can walk over this tile
            // We always allow the start, we want to move off where we are even if it's blocked
            // Same for the goal if it's not inclusive, because then we only have to reach it one
            // tile away, if it's walkable is irrelevant
            if !is_walkable(tile_res) &&
                !(neighbor == start) &&
                !(!goal_inclusive && node == goal)
            {
                continue
            }

            // Cost differ for straight and diagonal movement
            let cost = if x == node.x || y == node.y {
                costs.straight
            } else {
                // If it's a diagonal we also need to check we're not moving through a hard corner
                // Except, if it's the start and the end's not inclusive, we can ignore that
                // because we're only trying to reach it one tile away, not move to it
                if !(!goal_inclusive && node == goal) {
                    if !is_walkable(tiles.tile(Point2::new(x, node.y))) ||
                       !is_walkable(tiles.tile(Point2::new(node.x, y))) {
                        continue
                    }
                }

                costs.diagonal
            };

            neighbors.push((neighbor, cost));
        }
    }

    neighbors
}

fn is_walkable(tile_res: Result<&Tile, TilesError>) -> bool {
    if let Ok(tile) = tile_res {
        tile.floor && tile.object.is_none()
    } else {
        false
    }
}

fn heuristic(node: Point2<i32>, start: Point2<i32>, costs: &Costs) -> i32 {
    let dx = (node.x - start.x).abs();
    let dy = (node.y - start.y).abs();
    costs.straight*(dx + dy) + (costs.diagonal - 2*costs.straight) * dx.min(dy)
}
