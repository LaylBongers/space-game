extern crate ggez;
extern crate nalgebra;
#[macro_use] extern crate slog;
extern crate lagato;
extern crate lagato_ggez;
extern crate blockengine;

use {
    ggez::{
        event::{EventHandler, MouseButton, MouseState},
        timer,
        Context, GameResult,
    },
    nalgebra::{Point3, UnitQuaternion},
    slog::{Logger},

    blockengine::{rendering::{Renderer, RenderCamera}},
};

pub fn main() -> GameResult<()> {
    lagato_ggez::run_game(
        "blockgame", "carbidegames", "Block Game",
        |ctx, log| MainState::new(ctx, log),
    )
}

struct MainState {
    log: Logger,
    renderer: Renderer,
    rotation: f32,
}

impl MainState {
    fn new(ctx: &mut Context, log: Logger) -> GameResult<MainState> {
        info!(log, "Loading game");

        let renderer = Renderer::new(ctx);

        Ok(MainState {
            log,
            renderer,
            rotation: 0.0,
        })
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;
        const DELTA: f32 = 1.0 / DESIRED_FPS as f32;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            self.rotation += DELTA;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let render_camera = RenderCamera::new(
            Point3::new(0.0, 2.0, 0.0),
            UnitQuaternion::from_euler_angles(0.0, self.rotation, 0.0),
        );

        self.renderer.draw(ctx, &render_camera)?;

        Ok(())
    }

    fn mouse_button_down_event(
        &mut self, _ctx: &mut Context,
        _button: MouseButton, _x: i32, _y: i32
    ) {
    }

    fn mouse_button_up_event(
        &mut self, _ctx: &mut Context,
        _button: MouseButton, _x: i32, _y: i32
    ) {
    }

    fn mouse_motion_event(
        &mut self, _ctx: &mut Context,
        _state: MouseState, _x: i32, _y: i32, _xrel: i32, _yrel: i32
    ) {
    }

    fn quit_event(&mut self, _ctx: &mut Context) -> bool {
        info!(self.log, "quit_event() callback called, quitting");
        false
    }
}
