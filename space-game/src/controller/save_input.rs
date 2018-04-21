use ggez::{Context, GameResult};
use slog::{Logger};
//use serde::{Deserialize, Serialize};
//use rmp_serde::{Deserializer, Serializer};

use game::state::ship::{Ship};

pub struct SaveInputController {
}

impl SaveInputController {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        Ok(SaveInputController {
        })
    }

    pub fn update(
        &mut self, _log: &Logger, _ctx: &mut Context, _ship: &mut Ship
    ) -> GameResult<()> {
        /*while let Some(event) = self.ui.event_sink().next() {
            match event.as_str() {
                "load-game" => {
                    info!(log, "Loading game");

                    let mut file = ctx.filesystem.open("/save.mp")?;
                    let mut de = Deserializer::new(&mut file);
                    *ship = Deserialize::deserialize(&mut de).unwrap();
                },
                "save-game" => {
                    info!(log, "Saving game");

                    let mut file = ctx.filesystem.create("/save.mp")?;
                    ship.serialize(&mut Serializer::new(&mut file)).unwrap();
                },
                "new-game" => {
                    info!(log, "Creating new game");

                    *ship = Ship::starter(log);
                },
                _ => {},
            }
        }*/

        Ok(())
    }
}
