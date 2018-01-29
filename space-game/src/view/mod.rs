mod ship;

pub use self::ship::{draw_ship};

use ggez::{Context, GameResult};
use ggez::graphics::{self};
use nalgebra::{Point2};

use controller::{self, BuildInputController, BuildState, BuildChoice};
use model::ui::{UiOld, Button};

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

pub fn draw_ui(ctx: &mut Context, ui: &UiOld) -> GameResult<()> {
    for button in ui.buttons() {
        draw_button(ctx, button)?;
    }

    Ok(())
}

fn draw_button(ctx: &mut Context, button: &Button) -> GameResult<()> {
    // Draw the background
    graphics::set_color(ctx, (button.color.0, button.color.1, button.color.2, 200).into())?;
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
