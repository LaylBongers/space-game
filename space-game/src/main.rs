extern crate ggez;
extern crate alga;
extern crate nalgebra;
#[macro_use]
extern crate slog;
extern crate sloggers;
extern crate metrohash;
extern crate pathfinding;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate rmp_serde;
extern crate markedly;
extern crate markedly_ggez;

mod controller;
pub mod model;
mod view;

use std::env;
use std::path;
use std::collections::{HashMap};

use ggez::{Context, GameResult};
use ggez::timer;
use ggez::conf::{Conf, WindowMode, WindowSetup};
use ggez::event::{self, EventHandler, MouseButton, MouseState};
use ggez::graphics::{self, Font, Text};
use nalgebra::{Vector2, Point2};
use slog::{Logger};
use sloggers::{Build};
use sloggers::terminal::{TerminalLoggerBuilder};
use sloggers::types::{Severity};

use markedly::class::{ComponentClasses};
use markedly::input::{UiInput};
use markedly::template::{ComponentTemplate};
use markedly::{Ui};
use markedly_ggez::{GgezRenderer};

use controller::{BuildInputController, CameraInputController, SaveInputController};
use controller::ui::{UiInputController};
use model::{Camera};
use model::ship::{Ship};
use model::ui::{UiOld};

pub fn main() {
    // Set up logging
    let mut builder = TerminalLoggerBuilder::new();
    builder.level(Severity::Debug);
    let log = builder.build().unwrap();

    // Set up the ggez context
    let mut c = Conf::new();
    c.window_mode = WindowMode {
        width: 1280,
        height: 720,
        .. Default::default()
    };
    c.window_setup = WindowSetup {
        title: "Space Game".into(),
        .. Default::default()
    };
    let ctx = &mut Context::load_from_conf("space-game", "carbidegames", c).unwrap();

    // We add the CARGO_MANIFEST_DIR/resources do the filesystems paths so we we look in the cargo
    // project for files.
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        ctx.filesystem.mount(&path, true);
    }

    // Initialize and run the game
    let result = MainState::new(ctx, log.clone())
        .and_then(|mut s| event::run(ctx, &mut s));

    // Check if it ran successfully
    if let Err(e) = result {
        error!(log, "Error encountered: {}", e);
    } else {
        info!(log, "Game exited cleanly");
    }
}

struct MainState {
    log: Logger,
    ui: Ui<GgezRenderer>,
    ui_renderer: GgezRenderer,
    ui_input: UiInput,

    // Models
    camera: Camera,
    ship: Ship,
    ui_old: UiOld,

    // Controllers
    build_input: BuildInputController,
    camera_input: CameraInputController,
    save_input: SaveInputController,
    ui_input_old: UiInputController,

    // View Data
    font: Font,
}

impl MainState {
    fn new(ctx: &mut Context, log: Logger) -> GameResult<MainState> {
        info!(log, "Loading game");

        // Set up the game world camera
        let screen_size = Vector2::new(1280, 720);
        let mut camera = Camera::new(64, screen_size);
        camera.set_position(Point2::new(50.0, 50.0));

        let mut ui_old = UiOld::new();
        let font = Font::new(ctx, "/DejaVuSansMono.ttf", 8)?;

        // Set up everything needed for the UI
        let templates = load_templates(&log, ctx)?;

        let mut classes = ComponentClasses::new();
        classes.register("container", markedly::class::ContainerClass::new);
        classes.register("button", markedly::class::ButtonClass::new);

        let ui_renderer = GgezRenderer::new(font.clone());
        let ui_input = UiInput::new();

        // Set up the UI itself
        let ui = Ui::new(
            &templates["root"],
            Vector2::new(screen_size.x as f32, screen_size.y as f32),
            &classes,
        )?;

        // Create the starter ship
        let ship = Ship::starter(&log);

        let build_input = BuildInputController::new(ctx, &mut ui_old, &font)?;
        let camera_input = CameraInputController::new();
        let save_input = SaveInputController::new(ctx, &mut ui_old, &font)?;
        let ui_input_old = UiInputController::new();

        Ok(MainState {
            log,
            ui,
            ui_renderer,
            ui_input,

            camera,
            ship,
            ui_old,

            build_input,
            camera_input,
            save_input,
            ui_input_old,

            font,
        })
    }
}

fn load_templates(
    log: &Logger, ctx: &mut Context
) -> GameResult<HashMap<String, ComponentTemplate>> {
    let mut templates = HashMap::new();
    for path in ctx.filesystem.read_dir("/markedly")? {
        // We only want files, not directories, and files that are .mark
        if !ctx.filesystem.is_file(&path) ||
            !path.extension().map(|v| v == "mark").unwrap_or(false)
        {
            continue
        }

        // Get an identifier for this template
        let identifier = path.file_stem().unwrap().to_string_lossy().into_owned();

        // Now actually load in the file
        info!(log, "Loading ui template \"{}\"", identifier);
        let file = ctx.filesystem.open(path)?;
        let result = ComponentTemplate::from_reader(file);
        match result {
            Ok(template) => {
                templates.insert(identifier, template);
            },
            Err(error) => {
                error!(log, "Error while loading template:\n{}\n", error);
                return Err(String::from("Template loading failed"))?
            },
        }
    }

    Ok(templates)
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;
        const DELTA: f32 = 1.0 / DESIRED_FPS as f32;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            self.build_input.update(&mut self.ui_old)?;
            self.save_input.update(&self.log, ctx, &mut self.ui_old, &mut self.ship)?;
            self.ship.update(&self.log, DELTA);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::set_background_color(ctx, (10, 10, 15).into());
        graphics::clear(ctx);

        // Switch the projection to world rendering
        let size = graphics::get_size(ctx);
        self.camera.set_screen_size(Vector2::new(size.0 as i32, size.1 as i32));
        let pixels_projection = graphics::get_projection(ctx);
        graphics::set_projection(ctx, self.camera.projection());
        graphics::apply_transformations(ctx)?;

        // Draw everything in the world
        view::draw_ship(ctx, &self.ship, &self.camera)?;
        view::draw_build_indicator(ctx, &self.build_input)?;

        // Swith the projection back to pixels rendering for UI
        graphics::set_projection(ctx, pixels_projection);
        graphics::apply_transformations(ctx)?;

        // Draw the UI
        markedly::render::render(&self.ui_renderer, ctx, &self.ui)?;
        view::draw_ui(ctx, &self.ui_old)?;

        // Draw an FPS counter
        let fps = timer::get_fps(ctx);
        let text = Text::new(ctx, &format!("FPS: {:.2}", fps), &self.font)?;
        graphics::set_color(ctx, (255, 255, 255, 200).into())?;
        graphics::draw(ctx, &text, Point2::new(0.0, 710.0), 0.0)?;

        graphics::present(ctx);
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self, _ctx: &mut Context,
        button: MouseButton, x: i32, y: i32
    ) {
        self.ui_input.start_drag(Point2::new(x as f32, y as f32), &mut self.ui);

        self.build_input.handle_mouse_down(button);
        self.camera_input.handle_mouse_down(button);
    }

    fn mouse_button_up_event(
        &mut self, _ctx: &mut Context,
        button: MouseButton, x: i32, y: i32
    ) {
        self.ui_input.end_drag(Point2::new(x as f32, y as f32), &mut self.ui);

        self.ui_input_old.handle_mouse_up(button, Point2::new(x, y), &mut self.ui_old);
        self.build_input.handle_mouse_up(button, &mut self.ship, &mut self.ui_old);
        self.camera_input.handle_mouse_up(button);
    }

    fn mouse_motion_event(
        &mut self, _ctx: &mut Context,
        _state: MouseState, x: i32, y: i32, xrel: i32, yrel: i32
    ) {
        let position = Point2::new(x, y);
        let rel_position = Vector2::new(xrel, yrel);

        self.ui_input_old.handle_mouse_move(position, &self.ui_old);
        self.build_input.handle_mouse_move(
            position, &mut self.camera, &self.ship, &self.ui_input_old
        );
        self.camera_input.handle_mouse_move(rel_position, &mut self.camera);
    }

    fn quit_event(&mut self, _ctx: &mut Context) -> bool {
        info!(self.log, "quit_event() callback called, quitting");
        false
    }
}
