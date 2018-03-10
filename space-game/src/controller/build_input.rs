use ggez::{Context, GameResult};
use ggez::audio::{Source};
use ggez::event::{MouseButton};
use nalgebra::{Point2};

use markedly::template::{Template};
use markedly::input::{UiInput};
use markedly::scripting::{ScriptTable};
use markedly::{Ui, Context as UiContext, Tree};

use model::{Camera, ObjectClassId};
use model::ship::{Ship, Task};

pub struct BuildInputController {
    last_tile_position: Option<Point2<i32>>,
    build_state: BuildState,
    build_choice: BuildChoice,

    ui: BuildInputUiController,
    build_sound_queued: bool,
    place_sound: Source,
}

impl BuildInputController {
    pub fn new(
        ctx: &mut Context, ui: &mut Ui, ui_context: &UiContext,
    ) -> GameResult<Self> {
        let ui = BuildInputUiController::new(ctx, ui, ui_context)?;

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

    pub fn update(
        &mut self, ui: &mut Ui, ui_context: &UiContext,
    ) -> GameResult<()> {
        self.ui.update(&mut self.build_choice, ui, ui_context)?;

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
        &mut self, button: MouseButton, ship: &mut Ship, ui: &mut Ui, ui_context: &UiContext
    ) -> GameResult<()> {
        if self.build_choice == BuildChoice::None {
            return Ok(())
        }

        match button {
            MouseButton::Left => self.handle_build_up(ship),
            MouseButton::Right => self.handle_cancel_up(ui, ui_context)?,
            _ => {},
        }

        Ok(())
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
                            let has_task = ship.task_queue.task_at(tile_pos).is_some();

                            if has_tile && !has_object && !has_task {
                                let task = Task::new(tile_pos, id, 1.0);
                                ship.task_queue.queue_task(task).unwrap();
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

                            if let Some(task_id) = ship.task_queue.task_at(tile_pos) {
                                ship.task_queue.dequeue_task(task_id).unwrap();
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

    fn handle_cancel_up(&mut self, ui: &mut Ui, ui_context: &UiContext) -> GameResult<()> {
        self.build_state = BuildState::Hovering { position: self.last_tile_position };
        self.build_choice = BuildChoice::None;
        self.ui.clear_active_button(ui, ui_context)?;

        Ok(())
    }

    pub fn handle_mouse_move(
        &mut self,
        mouse_position: Point2<i32>,
        camera: &mut Camera, ship: &Ship, ui_input: &UiInput,
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
    Object(ObjectClassId),
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
    model: ScriptTable,
    ui: Tree,

    popup_template: Template,
    popup_ui: Option<Tree>,
}

impl BuildInputUiController {
    pub fn new(
        ctx: &mut Context, ui: &mut Ui, ui_context: &UiContext,
    ) -> GameResult<Self> {
        let template = Template::from_reader(ctx.filesystem.open("/markedly/build-menu.mark")?)?;
        let popup_template =
            Template::from_reader(ctx.filesystem.open("/markedly/build-menu-popup.mark")?)?;

        let ui = ui.insert_template(
            &template, None, "top-menu", ui_context,
        ).map_err(|e| format!("{:#?}", e))?;

        Ok(BuildInputUiController {
            model: ScriptTable::new(),
            ui,

            popup_template,
            popup_ui: None,
        })
    }

    fn update(
        &mut self, build_choice: &mut BuildChoice, ui: &mut Ui, ui_context: &UiContext,
    ) -> GameResult<()> {
        let old_choice = build_choice.clone();

        // Check the menu buttons
        while let Some(event) = self.ui.event_sink().next() {
            match event.as_str() {
                "build" => self.toggle_popup(ui, ui_context)?,
                "destroy" => *build_choice = BuildChoice::Destroy,
                "destroy-all" => *build_choice = BuildChoice::DestroyAll,
                _ => {}
            }
        }

        // Check the popup buttons
        if let Some(ref popup_ui) = self.popup_ui {
            while let Some(event) = popup_ui.event_sink().next() {
                match event.as_str() {
                    "build-floor" => *build_choice = BuildChoice::Floor,
                    "build-wall" => *build_choice = BuildChoice::Object(ObjectClassId(0)),
                    "build-door" => *build_choice = BuildChoice::Object(ObjectClassId(1)),
                    _ => {}
                }
            }
        }

        // Update the active indicator so the user knows which option is enabled
        if old_choice != *build_choice {
            self.clear_active_button(ui, ui_context)?;

            match *build_choice {
                BuildChoice::Floor | BuildChoice::Object(_) =>
                    self.model.set("build_active", true),
                BuildChoice::Destroy =>
                    self.model.set("destroy_active", true),
                BuildChoice::DestroyAll =>
                    self.model.set("destroy_all_active", true),
                _ => {},
            }

            ui.update_model(&self.ui, &self.model, ui_context)
                .map_err(|e| format!("{:#?}", e))?;
        }

        Ok(())
    }

    fn clear_active_button(&mut self, ui: &mut Ui, ui_context: &UiContext) -> GameResult<()> {
        self.model.set("build_active", false);
        self.model.set("destroy_active", false);
        self.model.set("destroy_all_active", false);

        ui.update_model(&self.ui, &self.model, ui_context)
            .map_err(|e| format!("{:#?}", e))?;

        Ok(())
    }

    fn toggle_popup(&mut self, ui: &mut Ui, ui_context: &UiContext) -> GameResult<()> {
        if self.popup_ui.is_some() {
            self.close_popup()
        } else {
            self.open_popup(ui, ui_context)
        }
    }

    fn close_popup(&mut self) -> GameResult<()> {
        if let Some(ref popup_events) = self.popup_ui {
        }

        Ok(())
    }

    fn open_popup(&mut self, ui: &mut Ui, ui_context: &UiContext) -> GameResult<()> {
        if self.popup_ui.is_none() {
            self.popup_ui = Some(ui.insert_template(
                &self.popup_template, None, "popup-container", ui_context,
            ).map_err(|e| format!("{:#?}", e))?);
        }

        Ok(())
    }
}
