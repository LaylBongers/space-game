use ggez::{Context, GameResult};
use ggez::audio::{Source};
use ggez::event::{MouseButton};
use nalgebra::{Point2};

use {
    rivr::input::{PcInputHandler},

    spacegame_game::{
        state::{normalize_area, BuildState, BuildDrag, BuildChoice, Camera, ship::{Ship, Task}}
    },
};

pub struct BuildInputHandler {
    last_tile_position: Option<Point2<i32>>,

    build_sound_queued: bool,
    place_sound: Source,
}

impl BuildInputHandler {
    pub fn new(
        ctx: &mut Context,
    ) -> GameResult<Self> {
        let mut place_sound = Source::new(ctx, "/object_placed.ogg")?;
        place_sound.set_volume(0.2);

        Ok(BuildInputHandler {
            last_tile_position: None,

            build_sound_queued: false,
            place_sound,
        })
    }

    pub fn update(
        &mut self
    ) -> GameResult<()> {
        if self.build_sound_queued {
            self.place_sound.play()?;
            self.build_sound_queued = false;
        }

        Ok(())
    }

    pub fn handle_mouse_down(&mut self, button: MouseButton, state: &mut BuildState) {
        if button != MouseButton::Left || state.choice == BuildChoice::None {
            return
        }

        // If we were currently hovering, switch over to dragging
        if let BuildDrag::Hovering { position: Some(hovered_tile) } = state.drag {
            state.drag = BuildDrag::Dragging {
                start: hovered_tile,
                end: hovered_tile,
            }
        }
    }

    pub fn handle_mouse_up(
        &mut self, button: MouseButton, state: &mut BuildState, ship: &mut Ship
    ) -> GameResult<()> {
        if state.choice == BuildChoice::None {
            return Ok(())
        }

        match button {
            MouseButton::Left => self.handle_build_up(state, ship),
            MouseButton::Right => self.handle_cancel_up(state)?,
            _ => {},
        }

        Ok(())
    }

    fn handle_build_up(&mut self, state: &mut BuildState, ship: &mut Ship) {
        // If we were currently dragging, switch back to hovering
        if let BuildDrag::Dragging { start, end } = state.drag {
            let mut world_changed = false;

            // This also means we finished a build, so let's apply it
            let (start, end) = normalize_area(start, end);
            for y in start.y..end.y {
                for x in start.x..end.x {
                    let tile_pos = Point2::new(x, y);
                    match state.choice {
                        BuildChoice::None => unreachable!(),
                        BuildChoice::Floor => {
                            let tile = ship.tiles.get_mut(tile_pos).unwrap();

                            if !tile.floor {
                                tile.floor = true;
                                world_changed = true;
                                self.build_sound_queued = true;
                            }
                        },
                        BuildChoice::Object(id) => {
                            let tile = ship.tiles.get_mut(tile_pos).unwrap();
                            let has_tile = tile.floor;
                            let has_object = tile.object.is_some();
                            let has_task = ship.task_queue.get_at(tile_pos).is_some();

                            if has_tile && !has_object && !has_task {
                                let task = Task::new(tile_pos, id, 1.0);
                                ship.task_queue.queue(task).unwrap();
                                self.build_sound_queued = true;
                            }
                        },
                        BuildChoice::Destroy => {
                            let tile = ship.tiles.get_mut(tile_pos).unwrap();

                            if tile.object.is_some() {
                                world_changed = true;
                                self.build_sound_queued = true;
                            }

                            tile.object = None;

                            if let Some(task_id) = ship.task_queue.get_at(tile_pos) {
                                ship.task_queue.dequeue(task_id).unwrap();
                                self.build_sound_queued = true;
                            }
                        },
                        BuildChoice::DestroyAll => {
                            let tile = ship.tiles.get_mut(tile_pos).unwrap();

                            if tile.floor || tile.object.is_some() {
                                world_changed = true;
                                self.build_sound_queued = true;
                            }

                            tile.floor = false;
                            tile.object = None;

                            if let Some(task_id) = ship.task_queue.get_at(tile_pos) {
                                ship.task_queue.dequeue(task_id).unwrap();
                                self.build_sound_queued = true;
                            }
                        },
                    }
                }
            }

            // Actually switch back to hovering now
            state.drag = BuildDrag::Hovering { position: self.last_tile_position };

            if world_changed {
                ship.tiles.mark_changed();
            }
        }
    }

    fn handle_cancel_up(&mut self, state: &mut BuildState) -> GameResult<()> {
        state.drag = BuildDrag::Hovering { position: self.last_tile_position };
        state.choice = BuildChoice::None;

        Ok(())
    }

    pub fn handle_mouse_move(
        &mut self,
        mouse_position: Point2<i32>, ui_input: &PcInputHandler,
        state: &mut BuildState, camera: &mut Camera, ship: &Ship,
    ) {
        // Get the position of the cursor in-world
        let world_position = camera.screen_to_world(mouse_position);
        let tile_position = Point2::new(
            world_position.x.floor() as i32,
            world_position.y.floor() as i32,
        );

        // Make sure we're not over UI, and the tile we're hovering over is valid
        if !ui_input.is_cursor_over_ui() &&
            ship.tiles.is_in_bounds(tile_position)
        {
            self.last_tile_position = Some(tile_position);

            match state.drag {
                BuildDrag::Hovering { ref mut position } => *position = Some(tile_position),
                BuildDrag::Dragging { start: _, ref mut end } => *end = tile_position,
            }
        } else {
            self.last_tile_position = None;

            // If this is an invalid tile, the dragging won't be interested but the hover should be
            // set to None so it won't show up previewed
            if let &mut BuildDrag::Hovering { ref mut position } = &mut state.drag {
                *position = None;
            }
        }
    }
}
