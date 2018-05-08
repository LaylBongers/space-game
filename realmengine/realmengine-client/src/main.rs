extern crate ggez;
#[macro_use] extern crate slog;
extern crate sloggers;

use {
    std::{path},

    ggez::{
        Context, GameResult, GameError,
        conf::{Conf, WindowMode, WindowSetup},
        event::{self, EventHandler, MouseButton, MouseState},
        timer,
    },
    slog::{Logger},
    sloggers::{Build, terminal::{TerminalLoggerBuilder}, types::{Severity}},
};

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

    // Just add the local resources directory
    let path = path::PathBuf::from("./resources");
    ctx.filesystem.mount(&path, true);

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
}

impl MainState {
    fn new(ctx: &mut Context, log: Logger) -> GameResult<MainState> {
        info!(log, "Loading game");

        Ok(MainState {
            log,
        })
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;
        const DELTA: f32 = 1.0 / DESIRED_FPS as f32;

        while timer::check_update_time(ctx, DESIRED_FPS) {
        }

        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self, _ctx: &mut Context,
        button: MouseButton, x: i32, y: i32
    ) {
    }

    fn mouse_button_up_event(
        &mut self, _ctx: &mut Context,
        button: MouseButton, x: i32, y: i32
    ) {
    }

    fn mouse_motion_event(
        &mut self, _ctx: &mut Context,
        _state: MouseState, x: i32, y: i32, xrel: i32, yrel: i32
    ) {
    }

    fn quit_event(&mut self, _ctx: &mut Context) -> bool {
        info!(self.log, "quit_event() callback called, quitting");
        false
    }
}
