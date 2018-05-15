extern crate ggez;
extern crate nalgebra;
#[macro_use] extern crate slog;
extern crate lagato;
extern crate lagato_ggez;
extern crate blockengine;

use {
    ggez::{
        event::{EventHandler, MouseButton, MouseState},
        timer, mouse,
        Context, GameResult,
    },
    nalgebra::{Point3, Vector3, UnitQuaternion},
    slog::{Logger},

    lagato::{camera::{PitchYawCamera}},
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
    camera: PitchYawCamera,
    player_position: Point3<f32>,
}

impl MainState {
    fn new(ctx: &mut Context, log: Logger) -> GameResult<MainState> {
        info!(log, "Loading game");

        let renderer = Renderer::new(ctx);
        let camera = PitchYawCamera::new(0.0, 0.0);

        mouse::set_relative_mode(ctx, true);

        Ok(MainState {
            log,
            renderer,
            camera,
            player_position: Point3::new(8.0, 1.0, 8.0),
        })
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;
        const _DELTA: f32 = 1.0 / DESIRED_FPS as f32;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            //self.camera.yaw += DELTA;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let render_camera = RenderCamera::new(
            self.player_position + Vector3::new(0.0, 1.5, 0.0),
            UnitQuaternion::from_euler_angles(self.camera.pitch, self.camera.yaw, 0.0),
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
        _state: MouseState, _x: i32, _y: i32, xrel: i32, yrel: i32
    ) {
        let sensitivity = 0.0025;

        self.camera.yaw += xrel as f32 * -sensitivity;
        self.camera.pitch += yrel as f32 * -sensitivity;

        let limit = ::std::f32::consts::PI * 0.4;
        self.camera.pitch = self.camera.pitch.max(-limit).min(limit);
    }

    fn quit_event(&mut self, _ctx: &mut Context) -> bool {
        info!(self.log, "quit_event() callback called, quitting");
        false
    }
}
