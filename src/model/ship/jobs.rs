use std::collections::{HashMap};
use nalgebra::{Point2};

pub struct JobQueue {
    jobs: HashMap<i32, Job>,
    next_job_id: i32,
}

impl JobQueue {
    pub fn new() -> Self {
        JobQueue {
            jobs: HashMap::new(),
            next_job_id: 0,
        }
    }

    pub fn jobs(&self) -> &HashMap<i32, Job> {
        &self.jobs
    }

    pub fn job_at(&self, position: Point2<i32>) -> Option<JobId> {
        for (key, job) in &self.jobs {
            if job.position == position {
                return Some(JobId(*key))
            }
        }

        None
    }

    pub fn queue_job(&mut self, position: Point2<i32>) -> Result<(), JobQueueError> {
        let id = JobId(self.next_job_id);
        self.next_job_id += 1;

        let job = Job::new(position, 1.0);
        self.jobs.insert(id.0, job);

        Ok(())
    }

    pub fn dequeue_job(&mut self, id: JobId) -> Result<(), JobQueueError> {
        self.jobs.remove(&id.0)
            .ok_or(JobQueueError::InvalidJobId { id })?;

        Ok(())
    }

    pub fn assign_job(&mut self) -> Option<JobId> {
        None
    }
}

#[derive(Debug, PartialEq)]
pub enum JobQueueError {
    InvalidJobId { id: JobId },
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct JobId(i32);

pub struct Job {
    position: Point2<i32>,
    _work_done: f32,
    _work_target: f32,
}

impl Job {
    pub fn new(position: Point2<i32>, work_target: f32) -> Self {
        Job {
            position,
            _work_done: 0.0,
            _work_target: work_target,
        }
    }

    pub fn position(&self) -> Point2<i32> {
        self.position
    }
}
