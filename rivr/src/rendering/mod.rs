use {
    nalgebra::{Point2},
    palette::{Srgba},
    Error, Ui, PanelId,
};

pub trait Renderer {
    fn finish(&mut self) -> Result<(), Error>;

    fn vertices(
        &mut self,
        vertices: &[Point2<f32>], indices: &[u16], color: Srgba,
    ) -> Result<(), Error>;
}

pub fn render<R: Renderer>(ui: &Ui, root: PanelId, renderer: &mut R) -> Result<(), Error> {
    // TODO: Remove this this is a hack to reset the transforms
    renderer.finish()?;

    renderer.vertices(&[
        Point2::new(0.0, 0.0),
        Point2::new(0.0, 32.0),
        Point2::new(32.0, 32.0),
        Point2::new(32.0, 0.0),
    ], &[0, 1, 3, 2, 3, 1], Srgba::new(1.0, 0.5, 0.5, 1.0))?;

    renderer.finish()?;

    Ok(())
}
