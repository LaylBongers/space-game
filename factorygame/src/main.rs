extern crate ggez;
extern crate nalgebra;
#[macro_use] extern crate slog;
extern crate lagato;
extern crate lagato_ggez;
extern crate blockengine;

use {
    std::f32::consts::{PI},

    ggez::{
        event::{EventHandler, MouseButton, MouseState},
        timer,
        Context, GameResult,
    },
    nalgebra::{Vector3, Point3, UnitQuaternion},
    slog::{Logger},

    lagato::{camera::{OrbitingCamera}, grid::{Voxels}},
    blockengine::rendering::{Renderer},
};

pub fn main() -> GameResult<()> {
    lagato_ggez::run_game(
        "factorygame", "carbidegames", "Factory Game",
        |ctx, log| MainState::new(ctx, log),
    )
}

struct MainState {
    log: Logger,
    renderer: Renderer,

    world: Voxels<bool>,
    camera: OrbitingCamera,
}

impl MainState {
    fn new(ctx: &mut Context, log: Logger) -> GameResult<MainState> {
        info!(log, "Loading game");

        // Create and generate world
        let size = Vector3::new(128, 32, 128);
        let mut world = Voxels::empty(size);
        for x in 0..size.x {
            for z in 0..size.z {
                *world.get_mut(Point3::new(x, 0, z)).unwrap() = true;
            }
        }

        let renderer = Renderer::new(ctx, &world);

        let camera = OrbitingCamera::new(Point3::new(0.0, 1.0, 0.0), PI * -0.25, PI * 1.25, 10.0);

        Ok(MainState {
            log,
            renderer,

            world,
            camera,
        })
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;
        const _DELTA: f32 = 1.0 / DESIRED_FPS as f32;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            //self.rotation += DELTA;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let render_camera = self.camera.to_render_camera();

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
