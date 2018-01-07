use nalgebra::{Point2};

use model::ui::{Button};

pub struct UiInputController {
    mouse_over_ui: bool,
}

impl UiInputController {
    pub fn new() -> Self {
        UiInputController {
            mouse_over_ui: false,
        }
    }

    pub fn mouse_over_ui(&self) -> bool {
        self.mouse_over_ui
    }

    pub fn handle_mouse_move(&mut self, position: Point2<i32>, buttons: &[&Button]) {
        self.mouse_over_ui = false;

        for button in buttons {
            let end = button.position + button.size;
            if position.x > button.position.x && position.y > button.position.y &&
                position.x < end.x && position.y < end.y {
                self.mouse_over_ui = true;
            }
        }
    }
}
