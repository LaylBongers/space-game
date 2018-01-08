use ggez::{Context, GameResult};
use ggez::graphics::{self, MeshBuilder};
use nalgebra::{Point2};

use controller::{self, BuildInputController, BuildState};
use model::{Ship, Camera};
use model::ui::{Ui, Button};

pub fn draw_ship(ctx: &mut Context, ship: &Ship, camera: &Camera) -> GameResult<()> {
    // Find the tiles we are drawing
    let (start, end) = camera.world_bounds();
    let size = ship.size();
    let start_x = (start.x.floor() as i32).max(0);
    let start_y = (start.y.floor() as i32).max(0);
    let end_x = (end.x.ceil() as i32).min(size.x);
    let end_y = (end.y.ceil() as i32).min(size.y);

    // Draw the ship's tiles
    let mut floor_builder = MeshBuilder::new();
    let mut object_builder = MeshBuilder::new();
    for y in start_y..end_y {
        for x in start_x..end_x {
            let tile = ship.tile(Point2::new(x, y)).unwrap();

            let (fx, fy) = (x as f32, y as f32);

            if tile.floor {
                floor_builder.triangles(&[
                    Point2::new(fx, fy),
                    Point2::new(fx + 1.0, fy),
                    Point2::new(fx, fy + 1.0),

                    Point2::new(fx + 1.0, fy + 1.0),
                    Point2::new(fx, fy + 1.0),
                    Point2::new(fx + 1.0, fy),
                ]);
            }

            if let Some(_) = tile.object {
                object_builder.triangles(&[
                    Point2::new(fx + 0.05, fy + 0.05),
                    Point2::new(fx + 0.95, fy + 0.05),
                    Point2::new(fx + 0.05, fy + 0.95),

                    Point2::new(fx + 0.95, fy + 0.95),
                    Point2::new(fx + 0.05, fy + 0.95),
                    Point2::new(fx + 0.95, fy + 0.05),
                ]);
            }
        }
    }
    let floor_mesh = floor_builder.build(ctx)?;
    let object_mesh = object_builder.build(ctx)?;

    graphics::set_color(ctx, (150, 150, 150).into())?;
    graphics::draw(ctx, &floor_mesh, Point2::new(0.0, 0.0), 0.0)?;

    graphics::set_color(ctx, (50, 50, 50).into())?;
    graphics::draw(ctx, &object_mesh, Point2::new(0.0, 0.0), 0.0)?;

    Ok(())
}

pub fn draw_build_indicator(
    ctx: &mut Context, build_input: &BuildInputController
) -> GameResult<()> {
    graphics::set_color(ctx, (255, 255, 255, 100).into())?;

    match *build_input.build_state() {
        BuildState::Hovering { position: Some(hovered_tile) } => {
            graphics::rectangle(
                ctx, graphics::DrawMode::Fill,
                graphics::Rect::new(
                    hovered_tile.x as f32, hovered_tile.y as f32,
                    1.0, 1.0,
                ),
            )?;
        },
        BuildState::Dragging { start, end } => {
            let (start, end) = controller::build_area(start, end);
            graphics::rectangle(
                ctx, graphics::DrawMode::Fill,
                    graphics::Rect::new(
                    start.x as f32, start.y as f32,
                    (end.x - start.x) as f32, (end.y - start.y) as f32,
                ),
            )?;
        },
        _ => {},
    }

    Ok(())
}

pub fn draw_ui(ctx: &mut Context, ui: &Ui) -> GameResult<()> {
    for button in ui.buttons() {
        draw_button(ctx, button)?;
    }

    Ok(())
}

fn draw_button(ctx: &mut Context, button: &Button) -> GameResult<()> {
    // Draw the background
    graphics::set_color(ctx, (255, 255, 255, 200).into())?;
    graphics::rectangle(
        ctx, graphics::DrawMode::Fill,
        graphics::Rect::new(
            button.position.x as f32, button.position.y as f32,
            button.size.x as f32, button.size.y as f32,
        ),
    )?;

    // Draw the text
    let x_offset = (button.size.x - button.text.width() as i32) / 2;
    let y_offset = (button.size.y - button.text.height() as i32) / 2;
    graphics::set_color(ctx, (0, 0, 0, 200).into())?;
    graphics::draw(ctx, &button.text, Point2::new(
        (button.position.x + x_offset) as f32,
        (button.position.y + y_offset) as f32,
    ), 0.0)?;

    Ok(())
}
