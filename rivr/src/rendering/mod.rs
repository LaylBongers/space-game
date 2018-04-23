use {
    nalgebra::{Point2, Vector2},
    palette::{Srgba},
    Error, RenderingError, Ui, PanelId,
    layouting,
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

    fn render_cache(
        &mut self,
        target_id: PanelId,
        source_id: PanelId,
        position: Point2<f32>,
    ) -> Result<(), Error>;

    fn render_vertices(
        &mut self,
        panel_id: PanelId,
        vertices: &[Point2<f32>], indices: &[u16], color: Srgba,
    ) -> Result<(), Error>;

    fn render_text(
        &mut self,
        panel_id: PanelId,
        text: &String, /* text_font: Option<&String>, */ text_size: u32,
        position: Point2<f32>, size: Vector2<f32>, color: Srgba,
    ) -> Result<(), Error>;
}

pub fn render<R: Renderer>(ui: &mut Ui, root_id: PanelId, renderer: &mut R) -> Result<(), Error> {
    // First re-layout the UI, we only need to do this during rendering, input should make use of
    // the cached information gathered here to be consistent with what's visible on screen
    let size = renderer.target_size();
    layouting::layout(ui, root_id, size);

    // Make sure the root panel is rendered, then display it to the target
    render_panel(ui, root_id, renderer)?;
    renderer.finish_to_target(root_id)?;

    Ok(())
}

fn render_panel<R: Renderer>(ui: &Ui, panel_id: PanelId, renderer: &mut R) -> Result<bool, Error> {
    let panel_entry = ui.get(panel_id).unwrap();
    let panel_size = panel_entry.layout.size;

    // If we got a zero (or less) size, skip this. It's not renderable
    // TODO: Clear the cache entry if this is the case
    if panel_size.x.ceil() < 1.0 || panel_size.y.ceil() < 1.0 {
        return Ok(false)
    }

    // If we got 1_000_000 or more, that means a panel has been told to maximize size without
    // anything to constrain it
    if panel_size.x >= 1_000_000.0 || panel_size.y >= 1_000_000.0 {
        return Err(Error::Rendering(RenderingError::PanelTooLarge))
    }

    // Make sure this panel's cache is created and of the correct size
    let cache_empty = renderer.create_resize_cache(panel_id, Vector2::new(
        panel_size.x.ceil() as u32, panel_size.y.ceil() as u32,
    ))?;

    // The parent's children need to be rendered first
    let mut child_rendered = false;
    if let Some(children) = panel_entry.panel.children() {
        for child_id in children {
            child_rendered |= render_panel(ui, *child_id, renderer)?
        }
    }

    // Render the component itself if we need to
    if cache_empty || child_rendered {
        panel_entry.panel.render(renderer, ui, panel_id, &panel_entry.layout)?;

        Ok(true)
    } else {
        Ok(false)
    }
}
