use ggez::{Context, GameResult};
use ggez::audio::{Source};
use ggez::event::{MouseButton};
use ggez::graphics::{Text, Font};
use nalgebra::{Point2, Vector2};

use controller::ui::{UiInputController};
use model::{Camera};
use model::ship::{Ship};
use model::ui::{Button, ButtonId, Ui};

pub struct BuildInputController {
    last_tile_position: Option<Point2<i32>>,
    build_state: BuildState,
    build_choice: BuildChoice,

    build_floor_button: ButtonId,
    build_wall_button: ButtonId,
    destroy_object_button: ButtonId,
    destroy_all_button: ButtonId,

    build_sound_queued: bool,
    place_sound: Source,
}

impl BuildInputController {
    pub fn new(ctx: &mut Context, ui: &mut Ui, font: &Font) -> GameResult<Self> {
        let mut pos = 6;
        let build_floor_button = ui.add(Button::new(
            Point2::new(pos, 6),
            Vector2::new(72, 24),
            Text::new(ctx, "Floor", font)?,
        ));
        pos += 72 + 6;

        let build_wall_button = ui.add(Button::new(
            Point2::new(pos, 6),
            Vector2::new(72, 24),
            Text::new(ctx, "Wall", font)?,
        ));
        pos += 72 + 6;

        let destroy_object_button = ui.add(Button::new(
            Point2::new(pos, 6),
            Vector2::new(84, 24),
            Text::new(ctx, "Destroy Object", font)?,
        ));
        pos += 84 + 6;

        let destroy_all_button = ui.add(Button::new(
            Point2::new(pos, 6),
            Vector2::new(72, 24),
            Text::new(ctx, "Destroy All", font)?,
        ));

        let mut place_sound = Source::new(ctx, "/object_placed.ogg")?;
        place_sound.set_volume(0.2);

        Ok(BuildInputController {
            last_tile_position: None,
            build_state: BuildState::Hovering { position: None },
            build_choice: BuildChoice::Floor,

            build_floor_button,
            build_wall_button,
            destroy_object_button,
            destroy_all_button,

            build_sound_queued: false,
            place_sound,
        })
    }

    pub fn build_state(&self) -> &BuildState {
        &self.build_state
    }

    pub fn update(&mut self, ui: &mut Ui) -> GameResult<()> {
        if ui.get_mut(self.build_floor_button).check_pressed() {
            self.build_choice = BuildChoice::Floor;
        }
        if ui.get_mut(self.build_wall_button).check_pressed() {
            self.build_choice = BuildChoice::Wall;
        }
        if ui.get_mut(self.destroy_object_button).check_pressed() {
            self.build_choice = BuildChoice::DestroyObject;
        }
        if ui.get_mut(self.destroy_all_button).check_pressed() {
            self.build_choice = BuildChoice::DestroyAll;
        }

        if self.build_sound_queued {
            self.place_sound.play()?;
            self.build_sound_queued = false;
        }

        Ok(())
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
                    let tile_pos = Point2::new(x, y);
                    match self.build_choice {
                        BuildChoice::Floor => {
                            let tile = ship.tile_mut(tile_pos).unwrap();
                            tile.floor = true;
                        },
                        BuildChoice::Wall => {
                            let can_build = {
                                let tile = ship.tile_mut(tile_pos).unwrap();
                                tile.floor == true || !tile.build_job.is_some()
                            };

                            // You can only build objects over floors
                            if can_build {
                                ship.job_queue_mut().queue_job(tile_pos).unwrap();
                                //tile.object = Some(ShipObject::new());
                            }
                        },
                        BuildChoice::DestroyObject => {
                            let job = {
                                let tile = ship.tile_mut(tile_pos).unwrap();
                                tile.object = None;
                                tile.build_job
                            };

                            // TODO
                            //if let Some(job) = job {
                            //    ship.dequeue_job(job).unwrap();
                            //}
                        },
                        BuildChoice::DestroyAll => {
                            let job = {
                                let tile = ship.tile_mut(tile_pos).unwrap();
                                tile.floor = false;
                                tile.object = None;
                                tile.build_job
                            };

                            // TODO
                            //if let Some(job) = job {
                            //    ship.dequeue_job(job).unwrap();
                            //}
                        },
                    }
                }
            }

            // Actually switch back to hovering now
            self.build_state = BuildState::Hovering { position: self.last_tile_position };

            // And finally, make sure the build sound is played
            self.build_sound_queued = true;
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
    DestroyObject,
    DestroyAll,
}

pub fn build_area(start: Point2<i32>, end: Point2<i32>) -> (Point2<i32>, Point2<i32>) {
    let min_x = start.x.min(end.x);
    let min_y = start.y.min(end.y);
    let max_x = start.x.max(end.x);
    let max_y = start.y.max(end.y);
    (Point2::new(min_x, min_y), Point2::new(max_x + 1, max_y + 1))
}
