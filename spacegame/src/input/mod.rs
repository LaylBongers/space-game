mod build_input;
mod camera_input;

pub use self::build_input::{BuildInputController, BuildState, BuildChoice, build_area};
pub use self::camera_input::{CameraInputController};

pub struct InputHandler {
}
