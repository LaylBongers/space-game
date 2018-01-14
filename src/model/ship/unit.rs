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
        self.update_job(log, delta, job_queue);
        self.update_move_to_target(delta);
    }

    fn update_job(&mut self, log: &Logger, delta: f32, job_queue: &mut JobQueue) {
        // A lot of the functionality in here is sequential steps to complete a job checked every
        // frame. For performance it may be beneficial to restructure it into something else that
        // lets a unit sequantially go through the actions needed (find job -> move to -> work).

        // Try to find a job to do if we don't have one yet, or the old one was removed
        if let Some(job) = self.assigned_job.and_then(|j| job_queue.job_mut(j)) {
            let job_center = Point2::new(
                job.position().x as f32 + 0.5,
                job.position().y as f32 + 0.5
            );

            // Check if we're at the destination
            if self.position.distance_squared(&job_center) < 0.5 * 0.5 {
                // We're there, apply work
                job.apply_work(delta);
            } else {
                // We're not there yet, go to the job
                self.move_target = Some(job.position());
            }

            return
        }

        // We don't have a job, stand in place while finding a new one
        self.assigned_job = job_queue.assign_job(log);
        self.move_target = None;
    }

    fn update_move_to_target(&mut self, delta: f32) {
        if let Some(target) = self.move_target {
            let speed = 1.5;
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
