use ggez::event::{MouseButton};
use nalgebra::{Point2};

use model::ui::{Button};

pub struct UiInputController {
    mouse_over_ui: bool,
    mouse_position: Point2<i32>,
}

impl UiInputController {
    pub fn new() -> Self {
        UiInputController {
            mouse_over_ui: false,
            mouse_position: Point2::new(0, 0),
        }
    }

    pub fn mouse_over_ui(&self) -> bool {
        self.mouse_over_ui
    }

    pub fn handle_mouse_up(
        &self, mouse_button: MouseButton, position: Point2<i32>, buttons: &mut [&mut Button]
    ) {
        // We're only listening for clicks
        if mouse_button != MouseButton::Left {
            return
        }

        // Set the click for any buttons we were hovering over
        for button in buttons {
            if is_hovering(position, &button) {
                button.pressed = true;
            }
        }
    }

    pub fn handle_mouse_move(&mut self, position: Point2<i32>, buttons: &[&Button]) {
        self.mouse_over_ui = false;
        self.mouse_position = position;

        for button in buttons {
            if is_hovering(position, &button) {
                self.mouse_over_ui = true;
            }
        }
    }
}

fn is_hovering(position: Point2<i32>, button: &Button) -> bool {
    let end = button.position + button.size;
    position.x > button.position.x && position.y > button.position.y &&
        position.x < end.x && position.y < end.y
}
