use {
    alga::linear::{EuclideanSpace},
    nalgebra::{Point2},
    metrohash::{MetroHashMap},
    slog::{Logger},

    object_class::{ObjectClassId},
    Error,
};

#[derive(Deserialize, Serialize)]
pub struct TaskQueue {
    // Faster non-crypto hasher for small & medium key sizes
    tasks: MetroHashMap<TaskId, Task>,
    next_task_id: u32,
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

    pub fn get(&self, id: TaskId) -> Option<&Task> {
        self.tasks.get(&id)
    }

    pub fn get_mut(&mut self, id: TaskId) -> Option<&mut Task> {
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

    pub fn queue(&mut self, task: Task) -> Result<(), Error> {
        let id = TaskId(self.next_task_id);
        self.next_task_id += 1;
        self.tasks.insert(id, task);

        Ok(())
    }

    pub fn dequeue(&mut self, id: TaskId) -> Result<(), Error> {
        self.tasks.remove(&id)
            .ok_or(Error::InvalidTaskId(id))?;

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

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq, Deserialize, Serialize)]
pub struct TaskId(pub u32);

#[derive(Deserialize, Serialize)]
pub struct Task {
    pub position: Point2<i32>,
    pub object_class: ObjectClassId,
    pub assigned: bool,
    pub unreachable: bool,

    work_done: f32,
    work_target: f32,
}

impl Task {
    pub fn new(position: Point2<i32>, object_class: ObjectClassId, work_target: f32) -> Self {
        Task {
            position,
            object_class,
            assigned: false,
            unreachable: false,

            work_done: 0.0,
            work_target,
        }
    }

    pub fn apply_work(&mut self, amount: f32) {
        self.work_done += amount;
    }

    pub fn is_done(&self) -> bool {
        self.work_done > self.work_target
    }
}
