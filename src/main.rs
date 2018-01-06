extern crate ggez;
extern crate alga;
extern crate nalgebra;

mod model;
mod view;

use std::env;
use std::path;

use ggez::{graphics, timer, Context, GameResult};
use ggez::conf::{Conf, WindowMode, WindowSetup};
use ggez::event::{self, EventHandler, MouseButton, MouseState};
use nalgebra::{Vector2, Point2};

use model::{Ship, Camera, InputState};
use model::ui::{Button};

struct MainState {
    camera: Camera,
    input_state: InputState,
    ship: Ship,

    build_wall_button: Button,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        // Set up the game world camera
        let mut camera = Camera::new(64, Vector2::new(1280, 720));
        camera.set_position(Point2::new(50.0, 50.0));

        // Just create a pattern so we know rendering works
        let mut ship = Ship::empty(Vector2::new(100, 100));
        let size = ship.size();
        for y in 0..size.y {
            for x in 0..size.x {
                let v = x + y;
                if (v % 2) == 0 {
                    ship.tile_mut(Point2::new(x, y)).floor = true;
                }
            }
        }

        // Set up the UI
        let build_wall_button = Button {
            position: Point2::new(12.0, 12.0),
            size: Vector2::new(12.0, 12.0),
        };

        Ok(MainState {
            camera,
            input_state: InputState::new(),
            ship,

            build_wall_button,
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

        // Draw the tile selection indicator
        graphics::set_color(ctx, (255, 255, 255, 100).into())?;
        graphics::rectangle(
            ctx, graphics::DrawMode::Fill,
            graphics::Rect::new(
                self.input_state.hovered_tile.x as f32, self.input_state.hovered_tile.y as f32,
                1.0, 1.0
            ),
        )?;

        graphics::present(ctx);
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self, _ctx: &mut Context,
        button: MouseButton, _x: i32, _y: i32
    ) {
        if button == MouseButton::Middle {
            self.input_state.mouse_down = true;
        }
    }

    fn mouse_button_up_event(
        &mut self, _ctx: &mut Context,
        button: MouseButton, _x: i32, _y: i32
    ) {
        if button == MouseButton::Middle {
            self.input_state.mouse_down = false;
        }
    }

    fn mouse_motion_event(
        &mut self, _ctx: &mut Context,
        _state: MouseState, x: i32, y: i32, xrel: i32, yrel: i32
    ) {
        // If the move button is held down, we need to move the camera
        if self.input_state.mouse_down {
            // Use the relative position the mouse is moved, then scale it to how much that is
            // in in-game world coordinates
            let pixels_per_tile = self.camera.pixels_per_tile();
            let new_position = self.camera.position()
                + Vector2::new(
                    -xrel as f32 / pixels_per_tile as f32,
                    yrel as f32 / pixels_per_tile as f32
                );
            self.camera.set_position(new_position);
        }

        // Find the position of the cursor in-world
        let world_position = self.camera.screen_to_world(Point2::new(x, y));
        self.input_state.hovered_tile = Point2::new(
            world_position.x.floor() as i32,
            world_position.y.floor() as i32,
        );
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

    // We add the CARGO_MANIFEST_DIR/resources do the filesystems paths so we we look in the cargo
    // project for files.
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
