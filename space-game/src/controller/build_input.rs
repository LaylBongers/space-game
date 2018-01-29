use ggez::{Context, GameResult};
use ggez::audio::{Source};
use ggez::event::{MouseButton};
use nalgebra::{Point2};

use markedly::template::{ComponentTemplate};
use markedly::input::{UiInput};
use markedly::class::{ComponentClasses};
use markedly::{Ui, ComponentEvents};

use controller::ui::{UiInputController};
use model::{Camera};
use model::ship::{Ship};

pub struct BuildInputController {
    last_tile_position: Option<Point2<i32>>,
    build_state: BuildState,
    build_choice: BuildChoice,

    ui: BuildInputUiController,
    build_sound_queued: bool,
    place_sound: Source,
}

impl BuildInputController {
    pub fn new(ctx: &mut Context, ui: &mut Ui, classes: &ComponentClasses) -> GameResult<Self> {
        let ui = BuildInputUiController::new(ctx, ui, classes)?;

        let mut place_sound = Source::new(ctx, "/object_placed.ogg")?;
        place_sound.set_volume(0.2);

        Ok(BuildInputController {
            last_tile_position: None,
            build_state: BuildState::Hovering { position: None },
            build_choice: BuildChoice::None,

            ui,
            build_sound_queued: false,
            place_sound,
        })
    }

    pub fn build_state(&self) -> &BuildState {
        &self.build_state
    }

    pub fn build_choice(&self) -> &BuildChoice {
        &self.build_choice
    }

    pub fn update(&mut self, ui: &mut Ui) -> GameResult<()> {
        self.ui.update(ui, &mut self.build_choice);

        if self.build_sound_queued {
            self.place_sound.play()?;
            self.build_sound_queued = false;
        }

        Ok(())
    }

    pub fn handle_mouse_down(&mut self, button: MouseButton) {
        if button != MouseButton::Left || self.build_choice == BuildChoice::None {
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

    pub fn handle_mouse_up(
        &mut self, button: MouseButton, ship: &mut Ship, ui: &mut Ui
    ) {
        if self.build_choice == BuildChoice::None {
            return
        }

        match button {
            MouseButton::Left => self.handle_build_up(ship),
            MouseButton::Right => self.handle_cancel_up(ui),
            _ => {},
        }
    }

    fn handle_build_up(&mut self, ship: &mut Ship) {
        // If we were currently dragging, switch back to hovering
        if let BuildState::Dragging { start, end } = self.build_state {
            let mut world_changed = false;

            // This also means we finished a build, so let's apply it
            let (start, end) = build_area(start, end);
            for y in start.y..end.y {
                for x in start.x..end.x {
                    let tile_pos = Point2::new(x, y);
                    match self.build_choice {
                        BuildChoice::None => unreachable!(),
                        BuildChoice::Floor => {
                            let tile = ship.tiles.tile_mut(tile_pos).unwrap();

                            if !tile.floor {
                                tile.floor = true;
                                world_changed = true;
                                self.build_sound_queued = true;
                            }
                        },
                        BuildChoice::Wall => {
                            let tile = ship.tiles.tile_mut(tile_pos).unwrap();
                            let has_tile = tile.floor;
                            let has_object = tile.object.is_some();
                            let has_task = ship.task_queue.task_at(tile_pos).is_some();

                            if has_tile && !has_object && !has_task {
                                ship.task_queue.queue_task(tile_pos).unwrap();
                                self.build_sound_queued = true;
                            }
                        },
                        BuildChoice::Destroy => {
                            let tile = ship.tiles.tile_mut(tile_pos).unwrap();

                            if tile.object.is_some() {
                                world_changed = true;
                                self.build_sound_queued = true;
                            }

                            tile.object = None;

                            if let Some(task_id) = ship.task_queue.task_at(tile_pos) {
                                ship.task_queue.dequeue_task(task_id).unwrap();
                                self.build_sound_queued = true;
                            }
                        },
                        BuildChoice::DestroyAll => {
                            let tile = ship.tiles.tile_mut(tile_pos).unwrap();

                            if tile.floor || tile.object.is_some() {
                                world_changed = true;
                                self.build_sound_queued = true;
                            }

                            tile.floor = false;
                            tile.object = None;

                            if let Some(task_id) = ship.task_queue.task_at(tile_pos) {
                                ship.task_queue.dequeue_task(task_id).unwrap();
                                self.build_sound_queued = true;
                            }
                        },
                    }
                }
            }

            // Actually switch back to hovering now
            self.build_state = BuildState::Hovering { position: self.last_tile_position };

            if world_changed {
                ship.tiles.mark_changed();
            }
        }
    }

    fn handle_cancel_up(&mut self, ui: &mut Ui) {
        self.build_state = BuildState::Hovering { position: self.last_tile_position };
        self.build_choice = BuildChoice::None;
        self.ui.clear_active_button(ui);
    }

    pub fn handle_mouse_move(
        &mut self,
        mouse_position: Point2<i32>,
        camera: &mut Camera, ship: &Ship, ui_input: &UiInput, ui_input_old: &UiInputController,
    ) {
        // Get the position of the cursor in-world
        let world_position = camera.screen_to_world(mouse_position);
        let tile_position = Point2::new(
            world_position.x.floor() as i32,
            world_position.y.floor() as i32,
        );

        // Make sure we're not over UI, and the tile we're hovering over is valid
        if !ui_input.is_cursor_over_ui() && !ui_input_old.mouse_over_ui() &&
            ship.tiles.is_in_bounds(tile_position)
        {
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

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BuildChoice {
    None,
    Floor,
    Wall,
    Destroy,
    DestroyAll,
}

pub fn build_area(start: Point2<i32>, end: Point2<i32>) -> (Point2<i32>, Point2<i32>) {
    let min_x = start.x.min(end.x);
    let min_y = start.y.min(end.y);
    let max_x = start.x.max(end.x);
    let max_y = start.y.max(end.y);
    (Point2::new(min_x, min_y), Point2::new(max_x + 1, max_y + 1))
}

struct BuildInputUiController {
    events: ComponentEvents,
}

impl BuildInputUiController {
    pub fn new(ctx: &mut Context, ui: &mut Ui, classes: &ComponentClasses) -> GameResult<Self> {
        let template_file = ctx.filesystem.open("/markedly/build-input.mark")?;
        let template = ComponentTemplate::from_reader(template_file)?;
        let events = ui.insert_template(&template, "top-menu", &classes)?;

        Ok(BuildInputUiController {
            events,
        })
    }

    fn update(&mut self, _ui: &mut Ui, build_choice: &mut BuildChoice) {
        while let Some(event) = self.events.next() {
            match event.as_str() {
                "build-floor" => *build_choice = BuildChoice::Floor,
                "build-wall" => *build_choice = BuildChoice::Wall,
                "destroy" => *build_choice = BuildChoice::Destroy,
                "destroy-all" => *build_choice = BuildChoice::DestroyAll,
                _ => {}
            }
        }

        //ui.get_mut(button).color = (120, 255, 120);
    }

    fn clear_active_button(&mut self, _ui: &mut Ui) {
    }
}
