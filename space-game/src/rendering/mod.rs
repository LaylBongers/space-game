mod ship;

pub use self::ship::{draw_ship};

use ggez::{
    Context, GameResult,
    graphics::{
        self, Font, Text, Image, MeshBuilder, Rect, DrawParam,
        spritebatch::{SpriteBatch},
    },
    timer,
};
use nalgebra::{Point2, Vector2};

use controller::{self, BuildInputController, BuildState, BuildChoice};
use game::{
    ObjectClasses,
    state::{Camera, ship::{Ship}},
};
use rendering::ship::{Bounds};

pub struct Renderer {
    fps_font: Font,
    tiles_batch: SpriteBatch,
}

impl Renderer {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let fps_font = Font::new(ctx, "/DejaVuSansMono.ttf", 8)?;
        let tiles_batch = SpriteBatch::new(Image::new(ctx, "/tiles.png")?);

        Ok(Renderer {
            fps_font,
            tiles_batch,
        })
    }

    pub fn render_frame(
        &mut self, ctx: &mut Context,
        build_input: &BuildInputController,
        object_classes: &ObjectClasses,
        camera: &mut Camera, ship: &Ship,
    ) -> GameResult<()> {
        graphics::set_background_color(ctx, (10, 10, 15).into());
        graphics::clear(ctx);

        // Switch the projection to world rendering
        let size = graphics::get_size(ctx);
        camera.set_screen_size(Vector2::new(size.0 as i32, size.1 as i32));
        let pixels_projection = graphics::get_projection(ctx);
        graphics::set_projection(ctx, camera.projection());
        graphics::apply_transformations(ctx)?;

        // Draw everything in the world
        draw_ship(
            ctx, ship, camera, object_classes, &mut self.tiles_batch
        )?;
        draw_build_graphics(
            ctx, build_input, ship, camera, object_classes, &mut self.tiles_batch
        )?;

        // Swith the projection back to pixels rendering for UI
        graphics::set_projection(ctx, pixels_projection);
        graphics::apply_transformations(ctx)?;

        // Draw an FPS counter
        let fps = timer::get_fps(ctx);
        let text = Text::new(ctx, &format!("FPS: {:.2}", fps), &self.fps_font)?;
        graphics::set_color(ctx, (255, 255, 255, 200).into())?;
        graphics::draw(ctx, &text, Point2::new(0.0, 710.0), 0.0)?;

        graphics::present(ctx);
        Ok(())
    }
}

pub fn draw_build_graphics(
    ctx: &mut Context, build_input: &BuildInputController,
    ship: &Ship, camera: &Camera, object_classes: &ObjectClasses, tiles: &mut SpriteBatch,
) -> GameResult<()> {
    // If clicking won't do anything, we don't want to draw an indicator
    if *build_input.build_choice() == BuildChoice::None {
        return Ok(())
    }

    draw_grid(ctx, ship, camera)?;
    draw_build_placeholder(ctx, build_input, object_classes, tiles)?;

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
    ctx: &mut Context, build_input: &BuildInputController, object_classes: &ObjectClasses,
    tiles: &mut SpriteBatch,
) -> GameResult<()> {
    // Check what we need to draw
    let uvs = match *build_input.build_choice() {
        BuildChoice::Floor =>
            Some(Rect::new(0.0, 0.5, 0.5, 0.5)),
        BuildChoice::Object(id) =>
            Some(object_classes.get(id).unwrap().uvs()),
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
