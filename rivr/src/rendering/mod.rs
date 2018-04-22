use {
    nalgebra::{Point2, Vector2},
    palette::{Srgba},
    Error, Ui, PanelId,
};

pub trait Renderer {
    /// Gets the render target's size in UI units.
    fn target_size(&mut self) -> Vector2<f32>;

    /// Finishes rendering by displaying the root panel's cache on the target.
    fn finish_to_target(&mut self, root_id: PanelId) -> Result<(), Error>;

    /// Creates or resizes a cache for the panel, ensuring it's available.
    /// Returns true if the cache has been created or recreated, and thus is empty.
    fn create_resize_cache(
        &mut self, panel_id: PanelId, size: Vector2<u32>
    ) -> Result<bool, Error>;

    fn vertices(
        &mut self,
        panel_id: PanelId,
        vertices: &[Point2<f32>], indices: &[u16], color: Srgba,
    ) -> Result<(), Error>;
}

pub fn render<R: Renderer>(ui: &Ui, root_id: PanelId, renderer: &mut R) -> Result<(), Error> {
    let size = renderer.target_size();

    // Make sure this component's cache is created and of the correct size
    let cache_empty = renderer.create_resize_cache(root_id, Vector2::new(
        size.x.ceil() as u32, size.y.ceil() as u32,
    ))?;

    if cache_empty {
        renderer.vertices(root_id, &[
            Point2::new(0.0, 0.0),
            Point2::new(0.0, 32.0),
            Point2::new(32.0, 32.0),
            Point2::new(32.0, 0.0),
        ], &[0, 1, 3, 2, 3, 1], Srgba::new(1.0, 0.5, 0.5, 1.0))?;
    }

    renderer.finish_to_target(root_id)?;

    Ok(())
}
