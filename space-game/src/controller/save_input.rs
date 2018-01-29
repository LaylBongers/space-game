use nalgebra::{Point2, Vector2};
use ggez::{Context, GameResult};
use ggez::graphics::{Text, Font};
use slog::{Logger};
use serde::{Deserialize, Serialize};
use rmp_serde::{Deserializer, Serializer};

use model::ship::{Ship};
use model::ui::{UiOld, Button, ButtonId};

pub struct SaveInputController {
    load_game_button: ButtonId,
    save_game_button: ButtonId,
    new_game_button: ButtonId,
}

impl SaveInputController {
    pub fn new(
        ctx: &mut Context, ui: &mut UiOld, font: &Font
    ) -> GameResult<Self> {
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

        // Set up the UI for this controller
        //let view = ui_root.view("saving", "saving");

        Ok(SaveInputController {
            load_game_button,
            save_game_button,
            new_game_button,
        })
    }

    pub fn update(
        &mut self, log: &Logger, ctx: &mut Context, ui: &mut UiOld, ship: &mut Ship
    ) -> GameResult<()> {
        if ui.get_mut(self.load_game_button).check_pressed() {
            info!(log, "Loading game");

            let mut file = ctx.filesystem.open("/save.mp")?;
            let mut de = Deserializer::new(&mut file);
            *ship = Deserialize::deserialize(&mut de).unwrap();
        }

        if ui.get_mut(self.save_game_button).check_pressed() {
            info!(log, "Saving game");

            let mut file = ctx.filesystem.create("/save.mp")?;
            ship.serialize(&mut Serializer::new(&mut file)).unwrap();
        }

        if ui.get_mut(self.new_game_button).check_pressed() {
            info!(log, "Creating new game");

            *ship = Ship::starter(log);
        }

        Ok(())
    }
}
