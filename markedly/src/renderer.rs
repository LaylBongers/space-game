use nalgebra::{Point2, Vector2};
use {Component, Color};

pub trait Renderer {
    type Error;
    type Context;

    /// Renders a rectangle.
    fn rectangle(
        &self, context: &mut Self::Context,
        position: Point2<f32>, size: Vector2<f32>, color: Color,
    ) -> Result<(), Self::Error>;

    /// Renders text centered in an area.
    fn text(
        &self, context: &mut Self::Context,
        text: &str, position: Point2<f32>, size: Vector2<f32>, color: Color,
    ) -> Result<(), Self::Error>;
}

pub fn render<R: Renderer>(
    renderer: &R, context: &mut R::Context, component: &Component<R>
) -> Result<(), R::Error> {
    render_component(renderer, context, component)
}

fn render_component<R: Renderer>(
    renderer: &R, context: &mut R::Context, component: &Component<R>
) -> Result<(), R::Error> {
    component.class.render(renderer, context, component.position, component.size)?;

    for child in &component.children {
        render_component(renderer, context, child)?;
    }

    Ok(())
}
