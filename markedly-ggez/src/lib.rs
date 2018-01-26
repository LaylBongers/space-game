extern crate ggez;
extern crate nalgebra;
extern crate markedly;

use ggez::{Context, GameResult};
use ggez::error::{GameError};
use ggez::graphics::{self, DrawMode, Rect};
use nalgebra::{Point2, Vector2};

use markedly::{Renderer};

pub struct GgezRenderer {
}

impl Renderer for GgezRenderer {
    type Error = GameError;
    type Context = Context;

    fn rectangle(
        &self, ctx: &mut Context, position: Point2<f32>, size: Vector2<f32>
    ) -> GameResult<()> {
        graphics::set_color(ctx, (255, 255, 255).into())?;
        graphics::rectangle(ctx, DrawMode::Fill, Rect::new(
            position.x, position.y,
            size.x, size.y,
        ))?;

        Ok(())
    }
}
