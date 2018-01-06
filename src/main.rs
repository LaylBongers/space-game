extern crate ggez;
extern crate nalgebra;

mod model;
mod view;
mod camera;

use std::env;
use std::path;

use ggez::{graphics, timer, Context, GameResult};
use ggez::conf::{Conf, WindowMode, WindowSetup};
use ggez::event::{self, EventHandler};
use nalgebra::{Vector2, Point2};

use model::{Ship};
use camera::{Camera};

struct MainState {
    ship: Ship,
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

        Ok(MainState {
            ship,
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
        let mut camera = Camera::new(64);
        camera.set_position(Point2::new(50.0, 50.0));
        let size = graphics::get_size(ctx);
        camera.set_screen_size(Vector2::new(size.0 as i32, size.1 as i32));
        graphics::set_projection(ctx, camera.projection());
        graphics::apply_transformations(ctx)?;

        // Draw the ship
        view::draw_ship(ctx, &self.ship, &camera)?;

        graphics::present(ctx);
        Ok(())
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
