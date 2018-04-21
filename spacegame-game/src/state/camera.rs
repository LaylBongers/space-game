use alga::linear::{Transformation};
use nalgebra::{Vector2, Point2, Point3, Matrix4};

#[derive(Deserialize, Serialize)]
pub struct Camera {
    position: Point2<f32>,
    pixels_per_tile: i32,
    half_screen_size: Vector2<i32>,
}

impl Camera {
    pub fn new(pixels_per_tile: i32, screen_size: Vector2<i32>) -> Self {
        Camera {
            position: Point2::new(0.0, 0.0),
            pixels_per_tile,
            half_screen_size: screen_size/2,
        }
    }

    pub fn position(&mut self) -> Point2<f32> {
        self.position
    }

    pub fn set_position(&mut self, position: Point2<f32>) {
        self.position = position;
    }

    pub fn pixels_per_tile(&self) -> i32 {
        self.pixels_per_tile
    }

    pub fn set_screen_size(&mut self, screen_size: Vector2<i32>) {
        self.half_screen_size = screen_size/2;
    }

    pub fn projection(&self) -> Matrix4<f32> {
        let half_world_size = self.half_world_size();
        Matrix4::new_orthographic(
            self.position.x - half_world_size.x,
            self.position.x + half_world_size.x,
            self.position.y - half_world_size.y,
            self.position.y + half_world_size.y,
            -1.0, 1.0,
        )
    }

    /// Calculates the start and end bounds of the camera in world space
    pub fn world_bounds(&self) -> (Point2<f32>, Point2<f32>) {
        let half_world_size = self.half_world_size();
        (self.position - half_world_size, self.position + half_world_size)
    }

    /// Converts a screen position to a world position
    pub fn screen_to_world(&self, pixels: Point2<i32>) -> Point2<f32> {
        // Start by calculating our screen position in clip space, so we can just use the matrix
        // to convert to world coordinates
        let clip = Point3::new(
            (pixels.x as f32 / self.half_screen_size.x as f32) - 1.0,
            -((pixels.y as f32 / self.half_screen_size.y as f32) - 1.0),
            0.0,
        );

        // Do the actual conversion
        let world = self.projection().try_inverse().unwrap().transform_point(&clip);

        Point2::new(world.x, world.y)
    }

    fn half_world_size(&self) -> Vector2<f32> {
        Vector2::new(
            self.half_screen_size.x as f32 / self.pixels_per_tile as f32,
            self.half_screen_size.y as f32 / self.pixels_per_tile as f32,
        )
    }
}
