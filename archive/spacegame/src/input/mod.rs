mod build_input;
mod camera_input;

use {
    ggez::{
        Context, GameResult,
        event::{MouseButton},
    },
    nalgebra::{Point2, Vector2},

    rivr::input::{PcInputHandler},

    spacegame_game::{
        state::{GameState},
    },
    input::{
        build_input::{BuildInputHandler},
        camera_input::{CameraInputHandler},
    },
    ui::{UiSystem},
};

pub struct InputHandler {
    build_input: BuildInputHandler,
    camera_input: CameraInputHandler,
    ui_input: PcInputHandler,
}

impl InputHandler {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let build_input = BuildInputHandler::new(ctx)?;
        let camera_input = CameraInputHandler::new();

        Ok(InputHandler {
            build_input,
            camera_input,
            ui_input: PcInputHandler::new(),
        })
    }

    pub fn update(&mut self) -> GameResult<()> {
        self.build_input.update()?;

        Ok(())
    }

    pub fn handle_button_down(
        &mut self,
        button: MouseButton, x: i32, y: i32,
        ui_system: &mut UiSystem, game_state: &mut GameState,
    ) {
        let position_f = Point2::new(x as f32, y as f32);

        self.ui_input.handle_drag_started(
            position_f, &mut ui_system.ui, &ui_system.frame
        ).unwrap();
        self.build_input.handle_mouse_down(button, &mut game_state.build_state);
        self.camera_input.handle_mouse_down(button);
    }

    pub fn handle_button_up(
        &mut self,
        button: MouseButton, x: i32, y: i32,
        ui_system: &mut UiSystem, game_state: &mut GameState,
    ) {
        let position_f = Point2::new(x as f32, y as f32);

        self.ui_input.handle_drag_ended(
            position_f, &mut ui_system.ui, &ui_system.frame
        ).unwrap();
        self.build_input.handle_mouse_up(
            button, &mut game_state.build_state, &mut game_state.ship
        ).unwrap();
        self.camera_input.handle_mouse_up(button);
    }

    pub fn handle_motion(
        &mut self,
        x: i32, y: i32, xrel: i32, yrel: i32,
        ui_system: &mut UiSystem, game_state: &mut GameState,
    ) {
        let position = Point2::new(x, y);
        let position_f = Point2::new(x as f32, y as f32);
        let rel_position = Vector2::new(xrel, yrel);

        self.ui_input.handle_cursor_moved(
            position_f, &mut ui_system.ui, &ui_system.frame
        ).unwrap();
        self.build_input.handle_mouse_move(
            position, &self.ui_input,
            &mut game_state.build_state, &mut game_state.camera, &mut game_state.ship
        );
        self.camera_input.handle_mouse_move(rel_position, &mut game_state.camera);
    }
}
