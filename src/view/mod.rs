use ggez::{graphics, Context, GameResult};
use nalgebra::{Point2};

use model::{Ship, Camera};

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
            let tile = ship.tile(Point2::new(x, y));

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
