use nalgebra::{Vector2, Point2};

pub struct Camera {
    position: Point2<f32>,
    pixels_per_tile: i32,
    half_screen_size: Vector2<i32>,
}

impl Camera {
    pub fn new(pixels_per_tile: i32) -> Self {
        Camera {
            position: Point2::new(0.0, 0.0),
            pixels_per_tile,
            half_screen_size: Vector2::new(0, 0)
        }
    }

    pub fn set_position(&mut self, position: Point2<f32>) {
        self.position = position;
    }

    pub fn set_screen_size(&mut self, screen_size: Vector2<i32>) {
        self.half_screen_size = screen_size/2;
    }

    pub fn world_to_screen(&self, world: Point2<f32>) -> Point2<f32> {
        // Figure out the screen position itself
        let mut screen = (world - self.position.coords) * self.pixels_per_tile as f32;

        // Flip the Y, we start at the top on the screen
        screen.y = -screen.y;

        // Apply screen centering on the camera position
        screen.x += self.half_screen_size.x as f32;
        screen.y += self.half_screen_size.y as f32;

        screen
    }

    /*pub fn screen_to_world(&self, screen: Point2<f32>) -> Point2<f32> {
    }

    /// Calculates the start and end bounds of the camera in world space
    pub fn camera_bounds(&self) -> (Point2<f32>, Point2<f32>) {
    }*/
}
