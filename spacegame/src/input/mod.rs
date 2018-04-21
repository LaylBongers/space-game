mod build_input;
mod camera_input;

use ggez::{
    Context, GameResult,
    event::{MouseButton},
};
use nalgebra::{Point2, Vector2};

use spacegame_game::{
    state::{BuildInputState, Camera, ship::{Ship}},
};
use self::{
    build_input::{BuildInputHandler},
    camera_input::{CameraInputHandler},
};

pub struct InputHandler {
    build_input: BuildInputHandler,
    camera_input: CameraInputHandler,
}

impl InputHandler {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let build_input = BuildInputHandler::new(ctx)?;
        let camera_input = CameraInputHandler::new();

        Ok(InputHandler {
            build_input,
            camera_input,
        })
    }

    pub fn update(&mut self) -> GameResult<()> {
        self.build_input.update()?;

        Ok(())
    }

    pub fn handle_button_down(
        &mut self, button: MouseButton, state: &mut BuildInputState
    ) {
        self.build_input.handle_mouse_down(button, state);
        self.camera_input.handle_mouse_down(button);
    }

    pub fn handle_button_up(
        &mut self, button: MouseButton, state: &mut BuildInputState, ship: &mut Ship
    ) {
        self.build_input.handle_mouse_up(button, state, ship).unwrap();
        self.camera_input.handle_mouse_up(button);
    }

    pub fn handle_motion(
        &mut self,
        x: i32, y: i32, xrel: i32, yrel: i32,
        state: &mut BuildInputState,
        camera: &mut Camera, ship: &mut Ship
    ) {
        let position = Point2::new(x, y);
        let rel_position = Vector2::new(xrel, yrel);

        self.build_input.handle_mouse_move(position, state, camera, ship);
        self.camera_input.handle_mouse_move(rel_position, camera);
    }
}
