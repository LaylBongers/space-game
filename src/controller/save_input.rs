use nalgebra::{Point2, Vector2};
use ggez::{Context, GameResult};
use ggez::graphics::{Text, Font};
use slog::{Logger};

use model::ship::{Ship};
use model::ui::{Ui, Button, ButtonId};

pub struct SaveInputController {
    load_game_button: ButtonId,
    save_game_button: ButtonId,
    new_game_button: ButtonId,
}

impl SaveInputController {
    pub fn new(ctx: &mut Context, ui: &mut Ui, font: &Font) -> GameResult<Self> {
        let mut pos = 1280 - 6 - 72;
        let load_game_button = ui.add(Button::new(
            Point2::new(pos, 6),
            Vector2::new(72, 24),
            Text::new(ctx, "Load Game", font)?,
        ));

        pos -= 6 + 72;
        let save_game_button = ui.add(Button::new(
            Point2::new(pos, 6),
            Vector2::new(72, 24),
            Text::new(ctx, "Save Game", font)?,
        ));

        pos -= 6 + 72;
        let new_game_button = ui.add(Button::new(
            Point2::new(pos, 6),
            Vector2::new(72, 24),
            Text::new(ctx, "New Game", font)?,
        ));

        Ok(SaveInputController {
            load_game_button,
            save_game_button,
            new_game_button,
        })
    }

    pub fn update(&mut self, log: &Logger, ui: &mut Ui, ship: &mut Ship) {
        if ui.get_mut(self.load_game_button).check_pressed() {
        }

        if ui.get_mut(self.save_game_button).check_pressed() {
        }

        if ui.get_mut(self.new_game_button).check_pressed() {
            *ship = Ship::starter(log);
        }
    }
}
