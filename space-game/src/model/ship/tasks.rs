use alga::linear::{EuclideanSpace};
use nalgebra::{Point2};
use metrohash::{MetroHashMap};
use slog::{Logger};

#[derive(Deserialize, Serialize)]
pub struct TaskQueue {
    // Faster non-crypto hasher for small & medium key sizes
    tasks: MetroHashMap<TaskId, Task>,
    next_task_id: i32,
}

impl TaskQueue {
    pub fn new() -> Self {
        TaskQueue {
            tasks: MetroHashMap::default(),
            next_task_id: 0,
        }
    }

    pub fn tasks(&self) -> &MetroHashMap<TaskId, Task> {
        &self.tasks
    }

    pub fn task(&self, id: TaskId) -> Option<&Task> {
        self.tasks.get(&id)
    }

    pub fn task_mut(&mut self, id: TaskId) -> Option<&mut Task> {
        self.tasks.get_mut(&id)
    }

    pub fn task_at(&self, position: Point2<i32>) -> Option<TaskId> {
        for (key, task) in &self.tasks {
            if task.position == position {
                return Some(*key)
            }
        }

        None
    }

    pub fn queue_task(&mut self, position: Point2<i32>) -> Result<(), TaskQueueError> {
        let id = TaskId(self.next_task_id);
        self.next_task_id += 1;

        let task = Task::new(position, 1.0);
        self.tasks.insert(id, task);

        Ok(())
    }

    pub fn dequeue_task(&mut self, id: TaskId) -> Result<(), TaskQueueError> {
        self.tasks.remove(&id)
            .ok_or(TaskQueueError::InvalidTaskId { id })?;

        Ok(())
    }

    pub fn assign_task(&mut self, log: &Logger, closest_to: Point2<f32>) -> Option<TaskId> {
        let mut found_distance_squared = ::std::f32::INFINITY;
        let mut found_task = None;

        // Find the closest valid task
        for (key, task) in &mut self.tasks {
            // We don't want a task that's already assigned, or one we can't reach
            if task.assigned() || task.unreachable() {
                continue
            }

            // Check if this task is closer than what we found
            let task_center = Point2::new(
                task.position().x as f32 + 0.5,
                task.position().y as f32 + 0.5
            );
            let distance_squared = closest_to.distance_squared(&task_center);
            if distance_squared < found_distance_squared {
                found_distance_squared = distance_squared;
                found_task = Some(*key)
            }
        }

        // If we found a task, assign it
        if let Some(task_id) = found_task {
            self.task_mut(task_id).unwrap().set_assigned(true);
            info!(log, "Assigned task {}", task_id.0);
        }

        found_task
    }

    pub fn clear_unreachable(&mut self) {
        for (_, task) in &mut self.tasks {
            task.set_unreachable(false);
        }
    }

    pub fn update(&mut self, log: &Logger) {
        let mut done = Vec::new();

        for (key, task) in &self.tasks {
            if task.is_done() {
                info!(log, "Removing task {} from queue, it's done", key.0);
                done.push(*key);
            }
        }

        for key in done {
            self.tasks.remove(&key);
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TaskQueueError {
    InvalidTaskId { id: TaskId },
}

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq, Deserialize, Serialize)]
pub struct TaskId(pub i32);

#[derive(Deserialize, Serialize)]
pub struct Task {
    position: Point2<i32>,
    assigned: bool,
    unreachable: bool,

    work_done: f32,
    work_target: f32,
}

impl Task {
    pub fn new(position: Point2<i32>, work_target: f32) -> Self {
        Task {
            position,
            assigned: false,
            unreachable: false,
            work_done: 0.0,
            work_target,
        }
    }

    pub fn position(&self) -> Point2<i32> {
        self.position
    }

    pub fn assigned(&self) -> bool {
        self.assigned
    }

    pub fn set_assigned(&mut self, value: bool) {
        self.assigned = value;
    }

    pub fn unreachable(&self) -> bool {
        self.unreachable
    }

    pub fn set_unreachable(&mut self, value: bool) {
        self.unreachable = value;
    }

    pub fn apply_work(&mut self, amount: f32) {
        self.work_done += amount;
    }

    pub fn is_done(&self) -> bool {
        self.work_done > self.work_target
    }
}
