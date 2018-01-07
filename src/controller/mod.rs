use nalgebra::{Point2, Vector2};

use model::{InputState, Camera, Ship};

pub fn handle_mouse_move(
    mouse_position: Point2<i32>, mouse_move: Vector2<i32>,
    input_state: &mut InputState, camera: &mut Camera, ship: &Ship,
) {
    // If the move button is held down, we need to move the camera
    if input_state.move_down {
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
        input_state.hovered_tile = Some(tile_position);
    } else {
        input_state.hovered_tile = None;
    }
}
