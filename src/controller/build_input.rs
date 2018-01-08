use ggez::{Context, GameResult};
use ggez::event::{MouseButton};
use ggez::graphics::{Text, Font};
use nalgebra::{Point2, Vector2};

use controller::ui::{UiInputController};
use model::{Camera, Ship};
use model::ui::{Button, ButtonId, Ui};

pub struct BuildInputController {
    last_tile_position: Option<Point2<i32>>,
    build_state: BuildState,
    build_choice: BuildChoice,

    build_floor_button: ButtonId,
    build_wall_button: ButtonId,
    destroy_button: ButtonId,
}

impl BuildInputController {
    pub fn new(ctx: &mut Context, ui: &mut Ui, font: &Font) -> GameResult<Self> {
        let build_floor_button = ui.add(Button::new(
            Point2::new(6, 6),
            Vector2::new(72, 24),
            Text::new(ctx, "Floor", font)?,
        ));
        let build_wall_button = ui.add(Button::new(
            Point2::new(84, 6),
            Vector2::new(72, 24),
            Text::new(ctx, "Wall", font)?,
        ));
        let destroy_button = ui.add(Button::new(
            Point2::new(162, 6),
            Vector2::new(72, 24),
            Text::new(ctx, "Destroy", font)?,
        ));

        Ok(BuildInputController {
            last_tile_position: None,
            build_state: BuildState::Hovering { position: None },
            build_choice: BuildChoice::Floor,

            build_floor_button,
            build_wall_button,
            destroy_button,
        })
    }

    pub fn build_state(&self) -> &BuildState {
        &self.build_state
    }

    pub fn update(&mut self, ui: &mut Ui) {
        if ui.get(self.build_floor_button).pressed {
            self.build_choice = BuildChoice::Floor;
            ui.get_mut(self.build_floor_button).pressed = false;
        }
        if ui.get(self.build_wall_button).pressed {
            self.build_choice = BuildChoice::Wall;
            ui.get_mut(self.build_wall_button).pressed = false;
        }
        if ui.get(self.destroy_button).pressed {
            self.build_choice = BuildChoice::Bulldoze;
            ui.get_mut(self.destroy_button).pressed = false;
        }
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
