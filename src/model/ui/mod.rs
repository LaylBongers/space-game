use ggez::graphics::{Text};
use nalgebra::{Point2, Vector2};

pub struct Ui {
    buttons: Vec<Button>,
}

impl Ui {
    pub fn new() -> Self {
        Ui {
            buttons: Vec::new(),
        }
    }

    pub fn get(&self, id: ButtonId) -> &Button {
        &self.buttons[id.0]
    }

    pub fn get_mut(&mut self, id: ButtonId) -> &mut Button {
        &mut self.buttons[id.0]
    }

    pub fn buttons(&self) -> &[Button] {
        &self.buttons
    }

    pub fn buttons_mut(&mut self) -> &mut [Button] {
        &mut self.buttons
    }

    pub fn add(&mut self, button: Button) -> ButtonId {
        self.buttons.push(button);
        ButtonId(self.buttons.len() - 1)
    }
}

#[derive(Clone, Copy)]
pub struct ButtonId(usize);

// TODO: Add function to button to check and reset pressed

pub struct Button {
    pub position: Point2<i32>,
    pub size: Vector2<i32>,
    pub text: Text,
    pub pressed: bool,
}

impl Button {
    pub fn new(position: Point2<i32>, size: Vector2<i32>, text: Text) -> Self {
        Button {
            position,
            size,
            text,
            pressed: false,
        }
    }
}
