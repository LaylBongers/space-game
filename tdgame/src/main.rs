extern crate ggez;
extern crate nalgebra;
#[macro_use] extern crate slog;
extern crate lagato;
extern crate lagato_ggez;
extern crate blockengine;
extern crate blockengine_rendering;

use {
    std::f32::consts::{PI},

    ggez::{
        event::{EventHandler, MouseButton, MouseState, Keycode, Mod},
        timer,
        Context, GameResult,
    },
    nalgebra::{Vector3, Point3},
    slog::{Logger},

    lagato::{camera::{OrbitingCamera}, grid::{Voxels, Range}, DirectionalInput, rotate_vector},
    blockengine_rendering::{Renderer, Texture, Mesh, Object, triangulate_voxels},
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
    input: DirectionalInput,

    world: Voxels<bool>,
    objects: Vec<Object>,
    camera: OrbitingCamera,
}

impl MainState {
    fn new(ctx: &mut Context, log: Logger) -> GameResult<MainState> {
        info!(log, "Loading game");

        let block_texture = Texture::load(ctx, "/greystone.png")?;
        let renderer = Renderer::new(ctx, &block_texture);
        let input = DirectionalInput::new();

        // Create and generate world
        let mut objects = Vec::new();
        let world_size = Vector3::new(32, 32, 32);
        let mut world = Voxels::empty(world_size);
        for local_position in Range::new_dim2(0, 0, world_size.x-1, world_size.z-1).iter() {
            let voxel_position = Point3::new(local_position.x, 0, local_position.y);
            *world.get_mut(voxel_position).unwrap() = true;
        }

        let mesh = Mesh::new(ctx, &triangulate_voxels(&world));
        objects.push(Object {
            position: Point3::new(0.0, 0.0, 0.0),
            mesh,
        });

        let camera = OrbitingCamera::new(Point3::new(0.0, 1.0, 0.0), PI * -0.25, PI * 1.25, 25.0);

        Ok(MainState {
            log,
            renderer,
            input,

            world,
            objects,
            camera,
        })
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;
        const DELTA: f32 = 1.0 / DESIRED_FPS as f32;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            let mut input = self.input.to_vector();
            input = rotate_vector(input, -self.camera.yaw);

            if input.x != 0.0 || input.y != 0.0 {
                input = input.normalize();
            }

            const SPEED: f32 = 10.0;
            self.camera.focus += Vector3::new(input.x, 0.0, input.y) * DELTA * SPEED;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let render_camera = self.camera.to_render_camera();

        self.renderer.draw(ctx, &render_camera, &self.objects)?;

        Ok(())
    }

    fn key_down_event(
        &mut self, ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool
    ) {
        match keycode {
            Keycode::S => self.input.backward = true,
            Keycode::W => self.input.forward = true,
            Keycode::A => self.input.left = true,
            Keycode::D => self.input.right = true,
            Keycode::Escape => ctx.quit().unwrap(),
            _ => {}
        }
    }

    fn key_up_event(
        &mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool
    ) {
        match keycode {
            Keycode::S => self.input.backward = false,
            Keycode::W => self.input.forward = false,
            Keycode::A => self.input.left = false,
            Keycode::D => self.input.right = false,
            _ => {}
        }
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
