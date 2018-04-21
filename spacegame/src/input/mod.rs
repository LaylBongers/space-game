mod build_input;
mod camera_input;

use {
    ggez::{
        Context, GameResult,
        event::{MouseButton},
    },
    nalgebra::{Point2, Vector2},

    spacegame_game::{
        state::{GameState},
    },
    input::{
        build_input::{BuildInputHandler},
        camera_input::{CameraInputHandler},
    }
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
        &mut self, button: MouseButton, game_state: &mut GameState
    ) {
        self.build_input.handle_mouse_down(button, &mut game_state.build_input_state);
        self.camera_input.handle_mouse_down(button);
    }

    pub fn handle_button_up(
        &mut self, button: MouseButton, game_state: &mut GameState
    ) {
        self.build_input.handle_mouse_up(
            button, &mut game_state.build_input_state, &mut game_state.ship
        ).unwrap();
        self.camera_input.handle_mouse_up(button);
    }

    pub fn handle_motion(
        &mut self,
        x: i32, y: i32, xrel: i32, yrel: i32,
        game_state: &mut GameState
    ) {
        let position = Point2::new(x, y);
        let rel_position = Vector2::new(xrel, yrel);

        self.build_input.handle_mouse_move(
            position,
            &mut game_state.build_input_state, &mut game_state.camera, &mut game_state.ship
        );
        self.camera_input.handle_mouse_move(rel_position, &mut game_state.camera);
    }
}
