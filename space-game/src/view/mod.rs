mod ship;

pub use self::ship::{draw_ship};

use ggez::{Context, GameResult};
use ggez::graphics::{self};

use controller::{self, BuildInputController, BuildState, BuildChoice};

pub fn draw_build_indicator(
    ctx: &mut Context, build_input: &BuildInputController
) -> GameResult<()> {
    // If clicking won't do anything, we don't want to draw an indicator
    if *build_input.build_choice() == BuildChoice::None {
        return Ok(())
    }

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
