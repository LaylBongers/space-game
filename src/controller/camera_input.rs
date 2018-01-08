use ggez::event::{MouseButton};
use nalgebra::{Vector2};

use model::{Camera};

pub struct CameraInputController {
    move_down: bool,
}

impl CameraInputController {
    pub fn new() -> Self {
        CameraInputController {
            move_down: false,
        }
    }

    pub fn handle_mouse_down(&mut self, button: MouseButton) {
        if button != MouseButton::Middle {
            return
        }

        self.move_down = true;
    }

    pub fn handle_mouse_up(&mut self, button: MouseButton) {
        if button != MouseButton::Left {
            return
        }

        self.move_down = false;
    }

    pub fn handle_mouse_move(&mut self, mouse_move: Vector2<i32>, camera: &mut Camera) {
        // If the move button is held down, we need to move the camera
        if self.move_down {
            // Use the relative position the mouse is moved, then scale it to how much that is
            // in in-game world coordinates
            let pixels_per_tile = camera.pixels_per_tile();
            let new_position = camera.position()
                + Vector2::new(
                    -mouse_move.x as f32 / pixels_per_tile as f32,
                    mouse_move.y as f32 / pixels_per_tile as f32
                );
            camera.set_position(new_position);
        }
    }
}
