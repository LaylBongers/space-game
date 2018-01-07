use ggez::event::{MouseButton};
use nalgebra::{Point2, Vector2};

use model::{Camera, Ship};

pub struct ShipInputController {
    build_down: bool,
    move_down: bool,
    pub hovered_tile: Option<Point2<i32>>,
}

impl ShipInputController {
    pub fn new() -> Self {
        ShipInputController {
            build_down: false,
            move_down: false,
            hovered_tile: None,
        }
    }

    pub fn update(&self, ship: &mut Ship) {
        if self.build_down {
            if let Some(hovered_tile) = self.hovered_tile {
                ship.tile_mut(hovered_tile).unwrap().floor = true;
            }
        }
    }

    pub fn handle_mouse_down(&mut self, button: MouseButton) {
        match button {
            MouseButton::Left => self.build_down = true,
            MouseButton::Middle => self.move_down = true,
            _ => {}
        }
    }

    pub fn handle_mouse_up(&mut self, button: MouseButton) {
        match button {
            MouseButton::Left => self.build_down = false,
            MouseButton::Middle => self.move_down = false,
            _ => {}
        }
    }

    pub fn handle_mouse_move(
        &mut self,
        mouse_position: Point2<i32>, mouse_move: Vector2<i32>,
        camera: &mut Camera, ship: &Ship,
    ) {
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

        // Get the position of the cursor in-world
        let world_position = camera.screen_to_world(mouse_position);
        let tile_position = Point2::new(
            world_position.x.floor() as i32,
            world_position.y.floor() as i32,
        );

        // Make sure it's a valid tile
        if ship.is_in_bounds(tile_position) {
            self.hovered_tile = Some(tile_position);
        } else {
            self.hovered_tile = None;
        }
    }
}
