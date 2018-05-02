use {
    alga::linear::{EuclideanSpace},
    nalgebra::{Point2},
    metrohash::{MetroHashMap},
    slog::{Logger},
};

#[derive(Deserialize, Serialize)]
pub struct TaskQueue<P> {
    // Faster non-crypto hasher for small & medium key sizes
    tasks: MetroHashMap<TaskId, Task<P>>,
    next_task_id: u32,
}

impl<P: TaskPayload> TaskQueue<P> {
    pub fn new() -> Self {
        TaskQueue {
            tasks: MetroHashMap::default(),
            next_task_id: 0,
        }
    }

    pub fn tasks(&self) -> &MetroHashMap<TaskId, Task<P>> {
        &self.tasks
    }

    pub fn get(&self, id: TaskId) -> Option<&Task<P>> {
        self.tasks.get(&id)
    }

    pub fn get_mut(&mut self, id: TaskId) -> Option<&mut Task<P>> {
        self.tasks.get_mut(&id)
    }

    pub fn get_at(&self, position: Point2<i32>) -> Option<TaskId> {
        for (key, task) in &self.tasks {
            if task.position == position {
                return Some(*key)
            }
        }

        None
    }

    pub fn queue(&mut self, task: Task<P>) -> Result<(), TaskQueueError> {
        let id = TaskId(self.next_task_id);
        self.next_task_id += 1;
        self.tasks.insert(id, task);

        Ok(())
    }

    pub fn dequeue(&mut self, id: TaskId) -> Result<(), TaskQueueError> {
        self.tasks.remove(&id)
            .ok_or(TaskQueueError::InvalidTaskId { id })?;

        Ok(())
    }

    pub fn assign(&mut self, log: &Logger, closest_to: Point2<f32>) -> Option<TaskId> {
        let mut found_distance_squared = ::std::f32::INFINITY;
        let mut found_task = None;

        // Find the closest valid task
        for (key, task) in &mut self.tasks {
            // We don't want a task that's already assigned, or one we can't reach
            if task.assigned || task.unreachable {
                continue
            }

            // Check if this task is closer than what we found
            let task_center = Point2::new(
                task.position.x as f32 + 0.5,
                task.position.y as f32 + 0.5
            );
            let distance_squared = closest_to.distance_squared(&task_center);
            if distance_squared < found_distance_squared {
                found_distance_squared = distance_squared;
                found_task = Some(*key)
            }
        }

        // If we found a task, assign it
        if let Some(task_id) = found_task {
            self.get_mut(task_id).unwrap().assigned = true;
            info!(log, "Assigned task {}", task_id.0);
        }

        found_task
    }

    pub fn clear_unreachable(&mut self) {
        for (_, task) in &mut self.tasks {
            task.unreachable = false;
        }
    }

    pub fn update(&mut self, log: &Logger) {
        let mut done = Vec::new();

        for (key, task) in &self.tasks {
            if task.payload.is_done() {
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
pub struct TaskId(pub u32);

#[derive(Deserialize, Serialize)]
pub struct Task<P> {
    pub position: Point2<i32>,
    pub assigned: bool,
    pub unreachable: bool,

    pub payload: P,
}

impl<P: TaskPayload> Task<P> {
    pub fn new(position: Point2<i32>, payload: P) -> Self {
        Task {
            position,
            assigned: false,
            unreachable: false,

            payload,
        }
    }
}

pub trait TaskPayload {
    fn is_done(&self) -> bool;
}
