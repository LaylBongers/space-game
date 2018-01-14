use alga::linear::{EuclideanSpace};
use nalgebra::{Point2};
use metrohash::{MetroHashMap};
use slog::{Logger};

pub struct JobQueue {
    // Faster non-crypto hasher for small & medium key sizes
    jobs: MetroHashMap<JobId, Job>,
    next_job_id: i32,
}

impl JobQueue {
    pub fn new() -> Self {
        JobQueue {
            jobs: MetroHashMap::default(),
            next_job_id: 0,
        }
    }

    pub fn jobs(&self) -> &MetroHashMap<JobId, Job> {
        &self.jobs
    }

    pub fn job(&self, id: JobId) -> Option<&Job> {
        self.jobs.get(&id)
    }

    pub fn job_mut(&mut self, id: JobId) -> Option<&mut Job> {
        self.jobs.get_mut(&id)
    }

    pub fn job_at(&self, position: Point2<i32>) -> Option<JobId> {
        for (key, job) in &self.jobs {
            if job.position == position {
                return Some(*key)
            }
        }

        None
    }

    pub fn queue_job(&mut self, position: Point2<i32>) -> Result<(), JobQueueError> {
        let id = JobId(self.next_job_id);
        self.next_job_id += 1;

        let job = Job::new(position, 1.0);
        self.jobs.insert(id, job);

        Ok(())
    }

    pub fn dequeue_job(&mut self, id: JobId) -> Result<(), JobQueueError> {
        self.jobs.remove(&id)
            .ok_or(JobQueueError::InvalidJobId { id })?;

        Ok(())
    }

    pub fn assign_job(&mut self, log: &Logger, closest_to: Point2<f32>) -> Option<JobId> {
        let mut found_distance_squared = ::std::f32::INFINITY;
        let mut found_job = None;

        // Find the closest job that isn't assigned
        for (key, job) in &mut self.jobs {
            if job.assigned() {
                continue
            }

            // Check if this job is closer than what we found
            let job_center = Point2::new(
                job.position().x as f32 + 0.5,
                job.position().y as f32 + 0.5
            );
            let distance_squared = closest_to.distance_squared(&job_center);
            if distance_squared < found_distance_squared {
                found_distance_squared = distance_squared;
                found_job = Some(*key)
            }
        }

        // If we found a job, assign it
        if let Some(job_id) = found_job {
            self.job_mut(job_id).unwrap().set_assigned(true);
            info!(log, "Assigned job {}", job_id.0);
        }

        found_job
    }

    pub fn update(&mut self, log: &Logger) {
        let mut done = Vec::new();

        for (key, job) in &self.jobs {
            if job.is_done() {
                info!(log, "Removing job {} from queue, it's done", key.0);
                done.push(*key);
            }
        }

        for key in done {
            self.jobs.remove(&key);
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum JobQueueError {
    InvalidJobId { id: JobId },
}

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub struct JobId(i32);

pub struct Job {
    position: Point2<i32>,
    assigned: bool,
    work_done: f32,
    work_target: f32,
}

impl Job {
    pub fn new(position: Point2<i32>, work_target: f32) -> Self {
        Job {
            position,
            assigned: false,
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

    pub fn apply_work(&mut self, amount: f32) {
        self.work_done += amount;
    }

    pub fn is_done(&self) -> bool {
        self.work_done > self.work_target
    }
}
