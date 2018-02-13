mod ship;

pub use self::ship::{draw_ship};

use ggez::{Context, GameResult};
use ggez::graphics::spritebatch::{SpriteBatch};
use ggez::graphics::{self, MeshBuilder, Rect, DrawParam};
use nalgebra::{Point2, Vector2};

use controller::{self, BuildInputController, BuildState, BuildChoice};
use model::ship::{Ship};
use model::{Camera};
use view::ship::{Bounds};

pub fn draw_build_graphics(
    ctx: &mut Context, build_input: &BuildInputController,
    ship: &Ship, camera: &Camera, tiles: &mut SpriteBatch,
) -> GameResult<()> {
    // If clicking won't do anything, we don't want to draw an indicator
    if *build_input.build_choice() == BuildChoice::None {
        return Ok(())
    }

    draw_grid(ctx, ship, camera)?;
    draw_build_placeholder(ctx, build_input, tiles)?;

    Ok(())
}

fn draw_grid(
    ctx: &mut Context, ship: &Ship, camera: &Camera,
) -> GameResult<()> {
    let bounds = Bounds::calculate(ship, camera);

    // Draw a build grid
    let mut grid_builder = MeshBuilder::new();
    for y in bounds.start.y..(bounds.end.y+1) {
        grid_builder.line(
            &[
                Point2::new(bounds.start.x as f32, y as f32),
                Point2::new(bounds.end.x as f32, y as f32)
            ],
            0.025
        );
    }
    for x in bounds.start.x..(bounds.end.x+1) {
        grid_builder.line(
            &[
                Point2::new(x as f32, bounds.start.y as f32),
                Point2::new(x as f32, bounds.end.y as f32)
            ],
            0.02
        );
    }
    let grid_mesh = grid_builder.build(ctx)?;

    graphics::set_color(ctx, (255, 255, 255, 8).into())?;
    graphics::draw(ctx, &grid_mesh, Point2::new(0.0, 0.0), 0.0)?;

    Ok(())
}

fn draw_build_placeholder(
    ctx: &mut Context, build_input: &BuildInputController, tiles: &mut SpriteBatch,
) -> GameResult<()> {
    // Check what we need to draw
    let uvs = match *build_input.build_choice() {
        BuildChoice::Floor =>
            Some(Rect::new(0.0, 0.5, 0.5, 0.5)),
        BuildChoice::Object(id) =>
            Some(match id.0 {
                0 => Rect::new(0.0, 0.0, 0.5, 0.5),
                1 => Rect::new(0.5, 0.0, 0.5, 0.5),
                _ => unreachable!()
            }),
        _ => None
    };

    // Check where we need to draw it
    let (start, end) = match *build_input.build_state() {
        BuildState::Hovering { position: Some(hovered_tile) } => {
            (hovered_tile, hovered_tile + Vector2::new(1, 1))
        },
        BuildState::Dragging { start, end } => {
            controller::build_area(start, end)
        },
        _ => (Point2::new(0, 0), Point2::new(0, 0)),
    };

    // Actually draw
    if let Some(uvs) = uvs {
        for y in start.y..end.y {
            for x in start.x..end.x {
                let (fx, fy) = (x as f32, y as f32);
                tiles.add(DrawParam {
                    src: uvs,
                    dest: Point2::new(fx, fy + 1.0),
                    scale: Point2::new(1.0 / 64.0, -1.0 / 64.0),
                    .. Default::default()
                });
            }
        }

        graphics::set_color(ctx, (255, 255, 255, 100).into())?;
        graphics::draw(ctx, tiles, Point2::new(0.0, 0.0), 0.0)?;
        tiles.clear();
    } else {
        graphics::set_color(ctx, (255, 120, 120, 50).into())?;
        graphics::rectangle(
            ctx, graphics::DrawMode::Fill,
                graphics::Rect::new(
                start.x as f32, start.y as f32,
                (end.x - start.x) as f32, (end.y - start.y) as f32,
            ),
        )?;
    }

    Ok(())
}
