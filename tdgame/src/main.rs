extern crate cgmath;
extern crate ggez;
#[macro_use] extern crate slog;
extern crate lagato;
extern crate lagato_ggez;
extern crate blockengine;
extern crate blockengine_rendering;

mod input;

use {
    cgmath::{Vector3, Point3, Rad, Angle},
    ggez::{
        event::{EventHandler, MouseButton, MouseState, Keycode, Mod},
        timer, graphics,
        Context, GameResult,
    },
    slog::{Logger},

    lagato::{
        camera::{OrbitingCamera, RenderCamera},
        grid::{Voxels, Range},
    },
    blockengine_rendering::{Renderer, Texture, Mesh, Object, triangulate_voxels},

    input::{InputHandler},
};

pub fn main() -> GameResult<()> {
    lagato_ggez::run_game(
        "factorygame", "carbidegames", "Tower Defense Game",
        |ctx, log| MainState::new(ctx, log),
    )
}

struct MainState {
    log: Logger,
    renderer: Renderer,
    input: InputHandler,

    world: Voxels<bool>,
    objects: Vec<Object>,

    camera: OrbitingCamera,
    last_camera: RenderCamera,
}

impl MainState {
    fn new(ctx: &mut Context, log: Logger) -> GameResult<MainState> {
        info!(log, "Loading game");

        let mut objects = Vec::new();

        let block_texture = Texture::load(ctx, "/greystone.png")?;
        let renderer = Renderer::new(ctx);
        let input = InputHandler::new(ctx, &mut objects)?;

        // Create and generate world
        let world_size = Vector3::new(32, 32, 32);
        let mut world = Voxels::empty(world_size);
        for local_position in Range::new_dim2(0, 0, world_size.x-1, world_size.z-1).iter() {
            let voxel_position = Point3::new(local_position.x, 0, local_position.y);
            *world.get_mut(voxel_position).unwrap() = true;
        }

        let mesh = Mesh::new(ctx, &triangulate_voxels(&world));
        objects.push(Object {
            position: Point3::new(0.0, 0.0, 0.0),
            visible: true,

            mesh,
            texture: block_texture.clone(),
        });

        let camera = OrbitingCamera::new(
            Point3::new(16.0, 1.0, 16.0), Rad::full_turn() * -0.15, Rad::full_turn() * 0.625, 10.0
        );
        let last_camera = camera.to_render_camera(graphics::get_size(ctx).into());

        Ok(MainState {
            log,
            renderer,
            input,

            world,
            objects,

            camera,
            last_camera,
        })
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;
        const DELTA: f32 = 1.0 / DESIRED_FPS as f32;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            self.input.update(
                &self.world, &mut self.objects, &mut self.camera, &self.last_camera, DELTA,
            );
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let render_camera = self.camera.to_render_camera(graphics::get_size(ctx).into());

        self.renderer.draw(ctx, &render_camera, &self.objects)?;

        self.last_camera = render_camera;
        Ok(())
    }

    fn key_down_event(
        &mut self, ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool
    ) {
        self.input.key_down_event(keycode);
        match keycode {
            Keycode::Escape => ctx.quit().unwrap(),
            _ => {}
        }
    }

    fn key_up_event(
        &mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool
    ) {
        self.input.key_up_event(keycode);
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
        _state: MouseState, x: i32, y: i32, _xrel: i32, _yrel: i32
    ) {
        self.input.mouse_motion_event(x, y);
    }

    fn quit_event(&mut self, _ctx: &mut Context) -> bool {
        info!(self.log, "quit_event() callback called, quitting");
        false
    }
}
