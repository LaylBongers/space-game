extern crate ggez;
extern crate nalgebra;

mod camera;
mod ship;

use std::env;
use std::path;

use ggez::{Context, GameResult};
use ggez::conf::{Conf, WindowMode, WindowSetup};
use ggez::event;
use ggez::graphics;
use nalgebra::{Vector2, Point2};

use camera::{Camera};
use ship::{Ship};

struct MainState {
    ship: Ship,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let mut ship = Ship::empty(Vector2::new(100, 100));

        // Just create a pattern so we know rendering works
        let mut place = true;
        for tile in ship.tiles_mut() {
            if place {
                tile.floor = true;
                place = false
            } else {
                place = true
            }
        }

        Ok(MainState {
            ship,
        })
    }
}


impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        // Create the camera and give it the relevant values for the current frame
        let mut camera = Camera::new(64);
        camera.set_position(Point2::new(0.0, 0.0));
        let size = graphics::get_size(ctx);
        camera.set_screen_size(Vector2::new(size.0 as i32, size.1 as i32));

        // Find the tiles we are drawing

        // Draw the ship's tiles
        graphics::set_color(ctx, (150, 150, 150).into())?;
        let size = self.ship.size();
        for y in 0..size.y {
            for x in 0..size.x {
                let tile = self.ship.tile(Point2::new(x, y));

                if !tile.floor {
                    continue
                }

                let pos = camera.world_to_screen(Point2::new(x as f32, y as f32));
                let rect = graphics::Rect::new(pos.x, pos.y, 64.0, -64.0);
                graphics::rectangle(ctx, graphics::DrawMode::Fill, rect)?;
            }
        }

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
