extern crate ggez;
extern crate nalgebra;
#[macro_use] extern crate slog;
extern crate serde;
extern crate rmp_serde;
extern crate rivr;
extern crate rivr_ggez;
extern crate lagato_ggez;
extern crate spacegame_game;

mod input;
mod rendering;
mod ui;

use {
    ggez::{
        event::{EventHandler, MouseButton, MouseState},
        graphics::{Rect},
        timer,
        Context, GameResult,
    },
    slog::{Logger},

    spacegame_game::{
        object_class::{ObjectClasses, ObjectClass, DoorObjectBehavior},
        state::{GameState},
    },
    input::{InputHandler},
    rendering::{Renderer},
    ui::{UiSystem},
};

pub fn main() -> GameResult<()> {
    lagato_ggez::run_game(
        "spacegame", "carbidegames", "Space Game",
        |ctx, log| MainState::new(ctx, log),
    )
}

struct MainState {
    log: Logger,
    renderer: Renderer,
    input_handler: InputHandler,
    ui_system: UiSystem,

    object_classes: ObjectClasses,
    game_state: GameState,
}

impl MainState {
    fn new(ctx: &mut Context, log: Logger) -> GameResult<MainState> {
        info!(log, "Loading game");

        // Set up all the objects we can place in ships
        let mut object_classes = ObjectClasses::new();
        object_classes.register(ObjectClass {
            friendly_name: "Wall".into(),
            uvs: Rect::new(0.0, 0.0, 0.5, 0.5),
            behavior: None,
        });
        object_classes.register(ObjectClass {
            friendly_name: "Door".into(),
            uvs: Rect::new(0.5, 0.0, 0.5, 0.5),
            behavior: Some(Box::new(DoorObjectBehavior)),
        });

        // Initialize game subsystems
        let renderer = Renderer::new(ctx)?;
        let input_handler = InputHandler::new(ctx)?;
        let ui_system = UiSystem::new(ctx, &object_classes)?;

        let game_state = GameState::new(&log);

        Ok(MainState {
            log,
            renderer,
            input_handler,
            ui_system,

            object_classes,
            game_state,
        })
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;
        const DELTA: f32 = 1.0 / DESIRED_FPS as f32;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            self.ui_system.update(&self.log, ctx, &mut self.game_state)?;
            self.input_handler.update()?;
            self.game_state.update(&self.log, &self.object_classes, DELTA).unwrap();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.renderer.render_frame(
            ctx, &mut self.ui_system, &self.object_classes, &mut self.game_state
        )
    }

    fn mouse_button_down_event(
        &mut self, _ctx: &mut Context,
        button: MouseButton, x: i32, y: i32
    ) {
        self.input_handler.handle_button_down(
            button, x, y, &mut self.ui_system, &mut self.game_state
        );
    }

    fn mouse_button_up_event(
        &mut self, _ctx: &mut Context,
        button: MouseButton, x: i32, y: i32
    ) {
        self.input_handler.handle_button_up(
            button, x, y, &mut self.ui_system, &mut self.game_state
        );
    }

    fn mouse_motion_event(
        &mut self, _ctx: &mut Context,
        _state: MouseState, x: i32, y: i32, xrel: i32, yrel: i32
    ) {
        self.input_handler.handle_motion(
            x, y, xrel, yrel, &mut self.ui_system, &mut self.game_state
        );
    }

    fn quit_event(&mut self, _ctx: &mut Context) -> bool {
        info!(self.log, "quit_event() callback called, quitting");
        false
    }
}
