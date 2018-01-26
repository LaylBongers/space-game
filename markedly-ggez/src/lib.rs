extern crate ggez;
extern crate nalgebra;
extern crate markedly;

use ggez::{Context, GameResult};
use ggez::error::{GameError};
use ggez::graphics::{self, DrawMode, Rect, Font, Text};
use nalgebra::{Point2, Vector2};

use markedly::{Renderer, Color};

pub struct GgezRenderer {
    font: Font,
}

impl GgezRenderer {
    pub fn new(font: Font) -> Self {
        GgezRenderer {
            font,
        }
    }
}

impl Renderer for GgezRenderer {
    type Error = GameError;
    type Context = Context;

    fn rectangle(
        &self, ctx: &mut Context, position: Point2<f32>, size: Vector2<f32>, color: Color,
    ) -> GameResult<()> {
        graphics::set_color(ctx, (color.red, color.green, color.blue, color.alpha).into())?;

        graphics::rectangle(ctx, DrawMode::Fill, Rect::new(
            position.x, position.y,
            size.x, size.y,
        ))?;

        Ok(())
    }

    fn text(
        &self, ctx: &mut Context,
        text: &str, position: Point2<f32>, size: Vector2<f32>, color: Color,
    ) -> GameResult<()> {
        graphics::set_color(ctx, (color.red, color.green, color.blue, color.alpha).into())?;

        let text = Text::new(ctx, text, &self.font)?;

        let x_offset = ((size.x - text.width() as f32) * 0.5).round();
        let y_offset = ((size.y - text.height() as f32) * 0.5).round();
        graphics::set_color(ctx, (0, 0, 0, 200).into())?;
        graphics::draw(ctx, &text, Point2::new(
            position.x + x_offset,
            position.y + y_offset,
        ), 0.0)?;

        Ok(())
    }
}
