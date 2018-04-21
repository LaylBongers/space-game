extern crate ggez;
extern crate nalgebra;
#[macro_use] extern crate slog;
extern crate sloggers;
extern crate serde;
extern crate rmp_serde;
extern crate rivr;
extern crate spacegame_game;

mod input;
mod rendering;

use std::env;
use std::path;

use ggez::{
    Context, GameResult, GameError,
    conf::{Conf, WindowMode, WindowSetup},
    event::{self, EventHandler, MouseButton, MouseState},
    graphics::{Rect},
    timer,
};
use nalgebra::{Vector2, Point2};
use slog::{Logger};
use sloggers::{Build, terminal::{TerminalLoggerBuilder}, types::{Severity}};

use spacegame_game::{
    ObjectClasses, GenericObjectClass,
    state::{BuildInputState, Camera, ship::{Ship}},
};
use input::{InputHandler};
use rendering::{Renderer};

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
    let ctx = &mut Context::load_from_conf("spacegame", "carbidegames", c).unwrap();

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
        match e {
            GameError::UnknownError(text) => error!(log, "Fatal:\n{}", text),
            e => error!(log, "Fatal: {}", e)
        }
    } else {
        info!(log, "Game exited cleanly");
    }
}

struct MainState {
    log: Logger,
    renderer: Renderer,
    input_handler: InputHandler,

    // Game Data
    object_classes: ObjectClasses,

    // Game State
    build_input_state: BuildInputState,
    camera: Camera,
    ship: Ship,
}

impl MainState {
    fn new(ctx: &mut Context, log: Logger) -> GameResult<MainState> {
        info!(log, "Loading game");

        // Initialize game subsystems
        let renderer = Renderer::new(ctx)?;
        let input_handler = InputHandler::new(ctx)?;

        // Set up all the objects we can place in ships
        let mut object_classes = ObjectClasses::new();
        object_classes.register(GenericObjectClass {
            uvs: Rect::new(0.0, 0.0, 0.5, 0.5), walkable: false,
        });
        object_classes.register(GenericObjectClass {
            uvs: Rect::new(0.5, 0.0, 0.5, 0.5), walkable: true,
        });

        // Set up the game world camera
        let screen_size = Vector2::new(1280, 720);
        let mut camera = Camera::new(64, screen_size);
        camera.set_position(Point2::new(50.0, 50.0));

        // Create the starter ship
        let ship = Ship::starter(&log);

        Ok(MainState {
            log,
            renderer,
            input_handler,

            build_input_state: BuildInputState::new(),
            camera,
            object_classes,
            ship,
        })
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;
        const DELTA: f32 = 1.0 / DESIRED_FPS as f32;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            self.input_handler.update()?;
            self.ship.update(&self.log, DELTA, &self.object_classes);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.renderer.render_frame(
            ctx, &self.build_input_state, &self.object_classes, &mut self.camera, &self.ship
        )
    }

    fn mouse_button_down_event(
        &mut self, _ctx: &mut Context,
        button: MouseButton, _x: i32, _y: i32
    ) {
        self.input_handler.handle_button_down(button, &mut self.build_input_state);
    }

    fn mouse_button_up_event(
        &mut self, _ctx: &mut Context,
        button: MouseButton, _x: i32, _y: i32
    ) {
        self.input_handler.handle_button_up(button, &mut self.build_input_state, &mut self.ship);
    }

    fn mouse_motion_event(
        &mut self, _ctx: &mut Context,
        _state: MouseState, x: i32, y: i32, xrel: i32, yrel: i32
    ) {
        self.input_handler.handle_motion(
            x, y, xrel, yrel, &mut self.build_input_state, &mut self.camera, &mut self.ship
        );
    }

    fn quit_event(&mut self, _ctx: &mut Context) -> bool {
        info!(self.log, "quit_event() callback called, quitting");
        false
    }
}
