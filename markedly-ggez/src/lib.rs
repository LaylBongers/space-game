extern crate ggez;
extern crate nalgebra;
extern crate markedly;

use std::error::{Error};

use ggez::{Context};
use ggez::graphics::{self, DrawMode, Rect, Font, Text};
use nalgebra::{Point2, Vector2};

use markedly::render::{Renderer};
use markedly::{Color};

pub struct GgezRenderer<'a> {
    ctx: &'a mut Context,
    font: &'a Font,
}

impl<'a> GgezRenderer<'a> {
    pub fn new(ctx: &'a mut Context, font: &'a Font) -> Self {
        GgezRenderer {
            ctx,
            font,
        }
    }
}

impl<'a> Renderer for GgezRenderer<'a> {
    fn rectangle(
        &mut self, position: Point2<f32>, size: Vector2<f32>, color: Color,
    ) -> Result<(), Box<Error>> {
        graphics::set_color(self.ctx, color_convert(color))?;

        graphics::rectangle(self.ctx, DrawMode::Fill, Rect::new(
            position.x, position.y,
            size.x, size.y,
        ))?;

        Ok(())
    }

    fn text(
        &mut self,
        text: &str, position: Point2<f32>, size: Vector2<f32>, color: Color,
    ) -> Result<(), Box<Error>> {
        graphics::set_color(self.ctx, color_convert(color))?;

        let text = Text::new(self.ctx, text, self.font)?;

        let x_offset = ((size.x - text.width() as f32) * 0.5).round();
        let y_offset = ((size.y - text.height() as f32) * 0.5).round();
        graphics::set_color(self.ctx, (0, 0, 0, 200).into())?;
        graphics::draw(self.ctx, &text, Point2::new(
            position.x + x_offset,
            position.y + y_offset,
        ), 0.0)?;

        Ok(())
    }
}

fn color_convert(color: Color) -> ::ggez::graphics::Color {
    ::ggez::graphics::Color::new(color.red, color.green, color.blue, color.alpha)
}
