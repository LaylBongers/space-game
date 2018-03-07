//! Rendering functions and backend traits.

use nalgebra::{Point2, Vector2};
use {ComponentId, Ui, Error, Color};

/// A renderer backend, implements how individual rendering operations are done.
pub trait Renderer {
    fn render_cache_to_target(&mut self, id: ComponentId) -> Result<(), Error>;

    fn create_resize_clear_cache(
        &mut self, id: ComponentId, size: Vector2<u32>
    ) -> Result<(), Error>;

    fn render_cache(
        &mut self, id: ComponentId,
        source_id: ComponentId, position: Point2<f32>
    ) -> Result<(), Error>;

    /// Renders a rectangle to the component's cache.
    fn rectangle(
        &mut self, id: ComponentId,
        position: Point2<f32>, size: Vector2<f32>, color: Color,
    ) -> Result<(), Error>;

    /// Renders text centered in an area to the component's cache.
    fn text(
        &mut self, id: ComponentId,
        text: &str, position: Point2<f32>, size: Vector2<f32>, color: Color,
    ) -> Result<(), Error>;
}

/// Renders a UI using a renderer backend.
pub fn render<R: Renderer>(
    renderer: &mut R, ui: &Ui
) -> Result<(), Error> {
    // TODO: Clear the cache of elements that don't exist anymore

    let root_id = ui.root_id();

    update_component_cache(
        renderer, ui, root_id, None, ui.target_size(),
    )?;

    renderer.render_cache_to_target(root_id)?;

    Ok(())
}

fn update_component_cache<R: Renderer>(
    renderer: &mut R, ui: &Ui, component_id: ComponentId, parent_id: Option<ComponentId>,
    parent_size: Vector2<f32>,
) -> Result<(), Error> {
    let component = ui.get(component_id).unwrap();
    let computed_position = component.compute_position(parent_size);

    // TODO: Don't re-render if it's not needed

    // Make sure this component's cache is of the correct size, and cleared
    renderer.create_resize_clear_cache(component_id, Vector2::new(
        component.attributes.size.x.ceil() as u32,
        component.attributes.size.y.ceil() as u32,
    ))?;

    // Let the component's class render itself to that cache
    component.class.render(component_id, &component.attributes, renderer)?;

    for child in &component.children {
        update_component_cache(
            renderer, ui, *child, Some(component_id),
            component.attributes.size
        )?;
    }

    // Render our cache to the parent's cache now that we've got everything
    if let Some(parent_id) = parent_id {
        renderer.render_cache(parent_id, component_id, computed_position)?;
    }

    Ok(())
}
