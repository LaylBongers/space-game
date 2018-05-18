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
    nalgebra::{Vector2, Vector3, Point3},
    slog::{Logger},

    lagato::{camera::{OrbitingCamera}, grid::{Voxels}},
    blockengine::{Chunk, rendering::{Renderer, VoxelsMesh}},
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

    chunks: Vec<Chunk>,
    camera: OrbitingCamera,
}

impl MainState {
    fn new(ctx: &mut Context, log: Logger) -> GameResult<MainState> {
        info!(log, "Loading game");

        let renderer = Renderer::new(ctx);

        // Create and generate world
        let chunk_size = Vector3::new(16, 16, 16);

        let mut chunks = Vec::new();
        // TODO: Restructure bounds to any kind of cell range
        for chunk_x in -3..4 {
            for chunk_z in -3..4 {
                let mut chunk_voxels = Voxels::empty(chunk_size);
                for x in 0..chunk_size.x {
                    for z in 0..chunk_size.z {
                        *chunk_voxels.get_mut(Point3::new(x, 0, z)).unwrap() = true;
                    }
                }

                let mesh = VoxelsMesh::triangulate(ctx, &chunk_voxels);
                chunks.push(Chunk {
                    position: Vector2::new(chunk_x, chunk_z),
                    voxels: chunk_voxels,
                    mesh,
                });
            }
        }

        let camera = OrbitingCamera::new(Point3::new(0.0, 1.0, 0.0), PI * -0.25, PI * 1.25, 10.0);

        Ok(MainState {
            log,
            renderer,

            chunks,
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

        self.renderer.draw(ctx, &render_camera, &self.chunks)?;

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
