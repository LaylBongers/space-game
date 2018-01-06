extern crate ggez;
extern crate nalgebra;

mod model;
mod view;

use std::env;
use std::path;

use ggez::{graphics, timer, Context, GameResult};
use ggez::conf::{Conf, WindowMode, WindowSetup};
use ggez::event::{self, EventHandler, MouseButton, MouseState};
use nalgebra::{Vector2, Point2};

use model::{Ship, Camera};

struct MainState {
    ship: Ship,
    camera: Camera,
    mouse_down: bool,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let mut ship = Ship::empty(Vector2::new(100, 100));

        // Just create a pattern so we know rendering works
        let size = ship.size();
        for y in 0..size.y {
            for x in 0..size.x {
                let v = x + y;
                if (v % 2) == 0 {
                    ship.tile_mut(Point2::new(x, y)).floor = true;
                }
            }
        }

        let mut camera = Camera::new(64);
        camera.set_position(Point2::new(50.0, 50.0));

        Ok(MainState {
            ship,
            camera,
            mouse_down: false,
        })
    }
}


impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS) {
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::set_background_color(ctx, (5, 5, 10).into());
        graphics::clear(ctx);

        // Create the camera and give it the relevant values for the current frame
        let size = graphics::get_size(ctx);
        self.camera.set_screen_size(Vector2::new(size.0 as i32, size.1 as i32));
        graphics::set_projection(ctx, self.camera.projection());
        graphics::apply_transformations(ctx)?;

        // Draw the ship
        view::draw_ship(ctx, &self.ship, &self.camera)?;

        graphics::present(ctx);
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self, _ctx: &mut Context,
        button: MouseButton, _x: i32, _y: i32
    ) {
        if button == MouseButton::Middle {
            self.mouse_down = true;
        }
    }

    fn mouse_button_up_event(
        &mut self, _ctx: &mut Context,
        button: MouseButton, _x: i32, _y: i32
    ) {
        if button == MouseButton::Middle {
            self.mouse_down = false;
        }
    }

    fn mouse_motion_event(
        &mut self, _ctx: &mut Context,
        _state: MouseState, _x: i32, _y: i32, xrel: i32, yrel: i32
    ) {
        if self.mouse_down {
            let pixels_per_tile = self.camera.pixels_per_tile();
            let new_position = self.camera.position()
                + Vector2::new(
                    -xrel as f32 / pixels_per_tile as f32,
                    yrel as f32 / pixels_per_tile as f32
                );
            self.camera.set_position(new_position);
        }
    }
}

pub fn main() {
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

    // We add the CARGO_MANIFEST_DIR/resources do the filesystems paths so
    // we we look in the cargo project for files.
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        ctx.filesystem.mount(&path, true);
    }

    let state = &mut MainState::new(ctx).unwrap();
    if let Err(e) = event::run(ctx, state) {
        println!("Error encountered: {}", e);
    } else {
        println!("Game exited cleanly.");
    }
}
