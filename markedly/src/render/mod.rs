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
    render_component(renderer, ui, ui.root_id())
}

fn render_component<R: Renderer>(
    renderer: &mut R, ui: &Ui, component_id: ComponentId
) -> Result<(), Box<Error>> {
    let component = ui.get(component_id).unwrap();
    component.class.render(renderer, component.position, component.size)?;

    for child in &component.children {
        render_component(renderer, ui, *child)?;
    }

    Ok(())
}
