extern crate ggez;
extern crate rivr;

use {
    ggez::{
        Context, GameError,
        graphics::{self, Mesh, Rect},
    },
    rivr::{
        Error, Point2, Srgba,
        rendering::{Renderer},
    }
};

pub struct GgezRivrRenderer<'a> {
    ctx: &'a mut Context,
}

impl<'a> GgezRivrRenderer<'a> {
    pub fn new(ctx: &'a mut Context) -> Self {
        GgezRivrRenderer {
            ctx,
        }
    }
}

impl<'a> Renderer for GgezRivrRenderer<'a> {
    fn finish(&mut self) -> Result<(), Error> {
        let window_size = graphics::get_size(self.ctx);
        graphics::set_screen_coordinates(self.ctx, Rect::new(
            0.0, 0.0,
            window_size.0 as f32, window_size.1 as f32,
        )).map_err(egtm)?;
        graphics::apply_transformations(self.ctx).map_err(egtm)?;

        Ok(())
    }

    fn vertices(
        &mut self,
        vertices: &[Point2<f32>], indices: &[u16], color: Srgba,
    ) -> Result<(), Error> {
        graphics::set_color(self.ctx, color_convert(color)).map_err(egtm)?;

        // Convert the vertices+indices to triangles and then a mesh
        let mut flattened_vertices = Vec::new();
        for index in indices {
            flattened_vertices.push(vertices[*index as usize]);
        }
        let mesh = Mesh::from_triangles(self.ctx, &flattened_vertices).map_err(egtm)?;

        graphics::draw(self.ctx, &mesh, Point2::new(0.0, 0.0), 0.0).map_err(egtm)?;

        Ok(())
    }
}

fn color_convert(color: Srgba) -> ::ggez::graphics::Color {
    ::ggez::graphics::Color::new(color.red, color.green, color.blue, color.alpha)
}

/// Converts a ggez error to a rivr error.
pub fn egtm(e: GameError) -> Error {
    Error::Rendering { error: Box::new(e) }
}

/// Converts a rivr error to a ggez error.
pub fn emtg(e: Error) -> GameError {
    GameError::UnknownError(format!("{:#?}", e))
}
