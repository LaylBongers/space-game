use {
    alga::linear::{EuclideanSpace},
    nalgebra::{Point2},
    slog::{Logger},

    object_class::{ObjectClasses},
    pathfinding,
    state::ship::{TaskId, TaskQueue, Tiles},
    Error,
};

const UNIT_SPEED: f32 = 1.5;

#[derive(Deserialize, Serialize)]
pub struct Unit {
    position: Point2<f32>,

    action_stack: Vec<Action>,
}

impl Unit {
    pub fn new(position: Point2<f32>) -> Self {
        Unit {
            position,

            action_stack: vec!(Action::FindTask),
        }
    }

    pub fn position(&self) -> Point2<f32> {
        self.position
    }

    pub fn update(
        &mut self, log: &Logger,
        object_classes: &ObjectClasses, tiles: &mut Tiles, task_queue: &mut TaskQueue,
        delta: f32,
    ) -> Result<(), Error> {
        let result = {
            let action = self.action_stack.iter_mut().last().unwrap();
            action.update(
                log, object_classes, tiles, task_queue, &mut self.position, delta,
            )?
        };

        match result {
            ActionResult::Continue => {},
            ActionResult::Done => {
                self.action_stack.pop();
            },
            ActionResult::Push(action) => {
                self.action_stack.push(action);
            },
        }

        Ok(())
    }
}

#[derive(Deserialize, Serialize)]
enum Action {
    FindTask,
    Work { task_id: TaskId, },
    FollowPath { path: Vec<Point2<i32>>, },
}

impl Action {
    fn update(
        &mut self,
        log: &Logger,
        object_classes: &ObjectClasses, tiles: &mut Tiles, task_queue: &mut TaskQueue,
        unit_position: &mut Point2<f32>, delta: f32,
    ) -> Result<ActionResult, Error> {
        let result = match *self {
            Action::FindTask => {
                if let Some(task_id) = task_queue.assign(log, *unit_position) {
                    ActionResult::Push(Action::Work { task_id })
                } else {
                    ActionResult::Continue
                }
            },
            Action::Work { task_id } => {
                let task = task_queue.get_mut(task_id).unwrap();

                let task_center = Point2::new(
                    task.position().x as f32 + 0.5,
                    task.position().y as f32 + 0.5
                );

                // Check if we're at the destination
                if (task_center.x - unit_position.x).abs() < 1.1 &&
                   (task_center.y - unit_position.y).abs() < 1.1 {
                    // We're there, apply work
                    task.apply_work(delta);

                    // If the work's done, we can add an object to the tile
                    if task.is_done() {
                        tiles.get_mut(task.position()).unwrap()
                            .object = Some(object_classes.create_object(task.object_class())?);
                        tiles.mark_changed();

                        ActionResult::Done
                    } else {
                        ActionResult::Continue
                    }
                } else {
                    // We're not there, find a path to our destination
                    if let Some(path) = pathfinding::find_path(
                        Point2::new(unit_position.x as i32, unit_position.y as i32),
                        task.position(), false, 1.0 / UNIT_SPEED,
                        tiles, object_classes,
                    ) {
                        ActionResult::Push(Action::FollowPath { path })
                    } else {
                        // We couldn't find a path, mark the task as unreachable
                        task.set_unreachable(true);
                        task.set_assigned(false);

                        info!(log, "Unassigned task {}, it's unreachable", task_id.0);

                        ActionResult::Continue
                    }
                }
            },
            Action::FollowPath { ref mut path, } => {
                let target = *path.iter().last().unwrap();
                let target = Point2::new(target.x as f32 + 0.5, target.y as f32 + 0.5);

                // Calculate how far away we are and how far we can travel
                let distance = unit_position.distance(&target);
                let distance_possible = UNIT_SPEED * delta;

                // If we're within our travel distance, just arrive
                if distance < distance_possible {
                    // TODO: If we encounter something that blocks us from moving, cancel the path
                    path.pop();
                    *unit_position = target;
                } else {
                    // If not, travel closer
                    let difference = target - *unit_position;
                    let move_amount = difference / distance * distance_possible;
                    *unit_position += move_amount;
                }

                // If the path's at the end, we can stop following
                if path.is_empty() {
                    ActionResult::Done
                } else {
                    ActionResult::Continue
                }
            },
        };

        Ok(result)
    }
}

enum ActionResult {
    Continue,
    Done,
    Push(Action)
}
