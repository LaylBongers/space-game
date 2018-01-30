//! Rendering functions and backend traits

use std::error::{Error};
use nalgebra::{Point2, Vector2};
use {ComponentId, Color, Ui};

pub trait Renderer {
    /// Renders a rectangle to the canvas.
    fn rectangle(
        &mut self,
        position: Point2<f32>, size: Vector2<f32>, color: Color,
    ) -> Result<(), Box<Error>>;

    /// Renders text centered in an area to the canvas.
    fn text(
        &mut self,
        text: &str, position: Point2<f32>, size: Vector2<f32>, color: Color,
    ) -> Result<(), Box<Error>>;
}

pub fn render<R: Renderer>(
    renderer: &mut R, ui: &Ui
) -> Result<(), Box<Error>> {
    render_component(renderer, ui, ui.root_id(), Point2::new(0.0, 0.0), Vector2::new(0.0, 0.0))
}

fn render_component<R: Renderer>(
    renderer: &mut R, ui: &Ui, component_id: ComponentId,
    computed_parent_position: Point2<f32>, parent_size: Vector2<f32>,
) -> Result<(), Box<Error>> {
    let component = ui.get(component_id).unwrap();
    let computed_position = component.compute_position(computed_parent_position, parent_size);

    // Let the component's class render itself
    component.class.render(renderer, computed_position, component.size)?;

    for child in &component.children {
        render_component(renderer, ui, *child, computed_position, component.size)?;
    }

    Ok(())
}
