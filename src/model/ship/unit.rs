use alga::linear::{EuclideanSpace};
use nalgebra::{Point2};
use slog::{Logger};

use model::ship::{JobId, JobQueue};

pub struct Unit {
    position: Point2<f32>,
    move_target: Option<Point2<i32>>,
    assigned_job: Option<JobId>,
}

impl Unit {
    pub fn new(position: Point2<f32>) -> Self {
        Unit {
            position,
            move_target: None,
            assigned_job: None,
        }
    }

    pub fn position(&self) -> Point2<f32> {
        self.position
    }

    pub fn update(&mut self, log: &Logger, delta: f32, job_queue: &mut JobQueue) {
        self.update_job(log, job_queue);
        self.update_move_to_target(delta);
    }

    fn update_job(&mut self, log: &Logger, job_queue: &mut JobQueue) {
        // Try to find a job to do if we don't have one yet, or the old one was removed
        if let Some(job) = self.assigned_job.and_then(|j| job_queue.job(j)) {
            // We have a job, go to it
            self.move_target = Some(job.position());

            return
        }

        // We don't have a job, stand in place while finding a new one
        self.assigned_job = job_queue.assign_job(log);
        self.move_target = None;
    }

    fn update_move_to_target(&mut self, delta: f32) {
        if let Some(target) = self.move_target {
            let speed = 1.0;
            let target = Point2::new(target.x as f32 + 0.5, target.y as f32 + 0.5);

            // Calculate how far away we are and how far we can travel
            let distance = self.position.distance(&target);
            let distance_possible = speed * delta;

            // If we're within our travel distance, just arrive
            if distance < distance_possible {
                self.move_target = None;
                self.position = target;
            } else {
                // If not, travel closer
                let difference = target - self.position;
                let move_amount = difference / distance * distance_possible;
                self.position += move_amount;
            }
        }
    }
}
