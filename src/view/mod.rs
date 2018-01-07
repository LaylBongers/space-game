use ggez::{graphics, Context, GameResult};
use nalgebra::{Point2};

use controller::{self, ShipInputController, BuildState};
use model::{Ship, Camera};
use model::ui::{Button};

pub fn draw_ship(ctx: &mut Context, ship: &Ship, camera: &Camera) -> GameResult<()> {
    // Find the tiles we are drawing
    let (start, end) = camera.world_bounds();
    let size = ship.size();
    let start_x = (start.x.floor() as i32).max(0);
    let start_y = (start.y.floor() as i32).max(0);
    let end_x = (end.x.ceil() as i32).min(size.x);
    let end_y = (end.y.ceil() as i32).min(size.y);

    // Draw the ship's tiles
    graphics::set_color(ctx, (150, 150, 150).into())?;
    for y in start_y..end_y {
        for x in start_x..end_x {
            let tile = ship.tile(Point2::new(x, y)).unwrap();

            if !tile.floor {
                continue
            }

            graphics::rectangle(
                ctx, graphics::DrawMode::Fill,
                graphics::Rect::new(x as f32, y as f32, 1.0, 1.0),
            )?;
        }
    }

    Ok(())
}

pub fn draw_build_indicator(ctx: &mut Context, ship_input: &ShipInputController) -> GameResult<()> {
    graphics::set_color(ctx, (255, 255, 255, 100).into())?;

    match *ship_input.build_state() {
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

pub fn draw_button(ctx: &mut Context, button: &Button) -> GameResult<()> {
    graphics::set_color(ctx, (255, 255, 255, 200).into())?;
    graphics::rectangle(
        ctx, graphics::DrawMode::Fill,
        graphics::Rect::new(
            button.position.x as f32, button.position.y as f32,
            button.size.x as f32, button.size.y as f32,
        ),
    )?;

    Ok(())
}
