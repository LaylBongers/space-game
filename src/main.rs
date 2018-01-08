extern crate ggez;
extern crate alga;
extern crate nalgebra;

mod controller;
mod model;
mod view;

use std::env;
use std::path;

use ggez::{timer, Context, GameResult};
use ggez::conf::{Conf, WindowMode, WindowSetup};
use ggez::event::{self, EventHandler, MouseButton, MouseState};
use ggez::graphics::{self, Font, Text};
use nalgebra::{Vector2, Point2};

use controller::{BuildInputController, CameraInputController, BuildChoice};
use controller::ui::{UiInputController};
use model::{Ship, Camera};
use model::ui::{Button};

struct MainState {
    camera: Camera,
    ship: Ship,

    build_input: BuildInputController,
    camera_input: CameraInputController,
    ui_input: UiInputController,

    build_floor_button: Button,
    build_wall_button: Button,
    destroy_button: Button,

    font: Font,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        // Set up the game world camera
        let mut camera = Camera::new(64, Vector2::new(1280, 720));
        camera.set_position(Point2::new(50.0, 50.0));

        // Create the starter ship
        let mut ship = Ship::empty(Vector2::new(100, 100));
        for y in 47..53 {
            for x in 48..52 {
                ship.tile_mut(Point2::new(x, y)).unwrap().floor = true;
            }
        }

        // Set up the UI
        let build_floor_button = Button::new(
            Point2::new(6, 6),
            Vector2::new(36, 36),
        );
        let build_wall_button = Button::new(
            Point2::new(48, 6),
            Vector2::new(36, 36),
        );
        let destroy_button = Button::new(
            Point2::new(90, 6),
            Vector2::new(36, 36),
        );

        let font = Font::new(ctx, "/DejaVuSansMono.ttf", 8)?;

        Ok(MainState {
            camera,
            ship,

            build_input: BuildInputController::new(),
            camera_input: CameraInputController::new(),
            ui_input: UiInputController::new(),

            build_floor_button,
            build_wall_button,
            destroy_button,

            font,
        })
    }
}


impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            if self.build_floor_button.pressed {
                self.build_input.set_build_choice(BuildChoice::Floor);
                self.build_floor_button.pressed = false;
            }
            if self.build_wall_button.pressed {
                self.build_input.set_build_choice(BuildChoice::Wall);
                self.build_wall_button.pressed = false;
            }
            if self.destroy_button.pressed {
                self.build_input.set_build_choice(BuildChoice::Bulldoze);
                self.destroy_button.pressed = false;
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::set_background_color(ctx, (5, 5, 10).into());
        graphics::clear(ctx);

        // Switch the projection to world rendering
        let size = graphics::get_size(ctx);
        self.camera.set_screen_size(Vector2::new(size.0 as i32, size.1 as i32));
        let pixels_projection = graphics::get_projection(ctx);
        graphics::set_projection(ctx, self.camera.projection());
        graphics::apply_transformations(ctx)?;

        // Draw everything in the world
        view::draw_ship(ctx, &self.ship, &self.camera)?;
        view::draw_build_indicator(ctx, &self.build_input)?;

        // Swith the projection back to pixels rendering for UI
        graphics::set_projection(ctx, pixels_projection);
        graphics::apply_transformations(ctx)?;

        // Draw the UI
        view::draw_button(ctx, &self.build_floor_button)?;
        view::draw_button(ctx, &self.build_wall_button)?;
        view::draw_button(ctx, &self.destroy_button)?;

        // Draw an FPS counter
        let fps = timer::get_fps(ctx);
        let text = Text::new(ctx, &format!("FPS: {:.2}", fps), &self.font)?;
        graphics::draw(ctx, &text, Point2::new(0.0, 710.0), 0.0)?;

        graphics::present(ctx);
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self, _ctx: &mut Context,
        button: MouseButton, _x: i32, _y: i32
    ) {
        self.build_input.handle_mouse_down(button);
        self.camera_input.handle_mouse_down(button);
    }

    fn mouse_button_up_event(
        &mut self, _ctx: &mut Context,
        button: MouseButton, x: i32, y: i32
    ) {
        self.ui_input.handle_mouse_up(button, Point2::new(x, y), &mut [
            &mut self.build_floor_button,
            &mut self.build_wall_button,
            &mut self.destroy_button,
        ]);
        self.build_input.handle_mouse_up(button, &mut self.ship);
        self.camera_input.handle_mouse_up(button);
    }

    fn mouse_motion_event(
        &mut self, _ctx: &mut Context,
        _state: MouseState, x: i32, y: i32, xrel: i32, yrel: i32
    ) {
        let position = Point2::new(x, y);
        let rel_position = Vector2::new(xrel, yrel);

        self.ui_input.handle_mouse_move(position, &[
            &self.build_floor_button,
            &self.build_wall_button,
            &self.destroy_button,
        ]);
        self.build_input.handle_mouse_move(
            position, &mut self.camera, &self.ship, &self.ui_input
        );
        self.camera_input.handle_mouse_move(rel_position, &mut self.camera);
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
