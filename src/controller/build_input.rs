use ggez::event::{MouseButton};
use nalgebra::{Point2};

use controller::ui::{UiInputController};
use model::{Camera, Ship};

pub struct BuildInputController {
    last_tile_position: Option<Point2<i32>>,
    build_state: BuildState,
    build_choice: BuildChoice,
}

impl BuildInputController {
    pub fn new() -> Self {
        BuildInputController {
            last_tile_position: None,
            build_state: BuildState::Hovering { position: None },
            build_choice: BuildChoice::Floor,
        }
    }

    pub fn build_state(&self) -> &BuildState {
        &self.build_state
    }

    pub fn set_build_choice(&mut self, build_choice: BuildChoice) {
        self.build_choice = build_choice;
    }

    pub fn handle_mouse_down(&mut self, button: MouseButton) {
        if button != MouseButton::Left {
            return
        }

        // If we were currently hovering, switch over to dragging
        if let BuildState::Hovering { position: Some(hovered_tile) } = self.build_state {
            self.build_state = BuildState::Dragging {
                start: hovered_tile,
                end: hovered_tile,
            }
        }
    }

    pub fn handle_mouse_up(&mut self, button: MouseButton, ship: &mut Ship) {
        if button != MouseButton::Left {
            return
        }

        // If we were currently dragging, switch back to hovering
        if let BuildState::Dragging { start, end } = self.build_state {
            // This also means we finished a build, so let's apply it
            let (start, end) = build_area(start, end);
            for y in start.y..end.y {
                for x in start.x..end.x {
                    let tile = Point2::new(x, y);
                    match self.build_choice {
                        BuildChoice::Floor | BuildChoice::Wall =>
                            ship.tile_mut(tile).unwrap().floor = true,
                        BuildChoice::Bulldoze =>
                            ship.tile_mut(tile).unwrap().floor = false,
                    }
                }
            }

            // Actually switch back to hovering now
            self.build_state = BuildState::Hovering { position: self.last_tile_position }
        }
    }

    pub fn handle_mouse_move(
        &mut self,
        mouse_position: Point2<i32>,
        camera: &mut Camera, ship: &Ship, ui_input: &UiInputController,
    ) {
        // Get the position of the cursor in-world
        let world_position = camera.screen_to_world(mouse_position);
        let tile_position = Point2::new(
            world_position.x.floor() as i32,
            world_position.y.floor() as i32,
        );

        // Make sure we're not over UI, and the tile we're hovering over is valid
        if !ui_input.mouse_over_ui() && ship.is_in_bounds(tile_position) {
            self.last_tile_position = Some(tile_position);

            match self.build_state {
                BuildState::Hovering { ref mut position } => *position = Some(tile_position),
                BuildState::Dragging { start: _, ref mut end } => *end = tile_position,
             }
        } else {
            self.last_tile_position = None;

            // If this is an invalid tile, the dragging won't be interested but the hover should be
            // set to None so it won't show up previewed
            if let &mut BuildState::Hovering { ref mut position } = &mut self.build_state {
                *position = None;
            }
        }
    }
}

pub enum BuildState {
    Hovering { position: Option<Point2<i32>> },
    Dragging { start: Point2<i32>, end: Point2<i32> },
}

pub enum BuildChoice {
    Floor,
    Wall,
    Bulldoze,
}

pub fn build_area(start: Point2<i32>, end: Point2<i32>) -> (Point2<i32>, Point2<i32>) {
    let min_x = start.x.min(end.x);
    let min_y = start.y.min(end.y);
    let max_x = start.x.max(end.x);
    let max_y = start.y.max(end.y);
    (Point2::new(min_x, min_y), Point2::new(max_x + 1, max_y + 1))
}
