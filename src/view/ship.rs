use ggez::{Context, GameResult};
use ggez::graphics::{self, MeshBuilder};
use nalgebra::{Point2};

use model::{Camera};
use model::ship::{Ship};

pub fn draw_ship(ctx: &mut Context, ship: &Ship, camera: &Camera) -> GameResult<()> {
    // Find the tiles we are drawing
    let (start, end) = camera.world_bounds();
    let size = ship.tiles().size();
    let start_x = (start.x.floor() as i32).max(0);
    let start_y = (start.y.floor() as i32).max(0);
    let end_x = (end.x.ceil() as i32).min(size.x);
    let end_y = (end.y.ceil() as i32).min(size.y);

    draw_tiles(ctx, ship, start_x, start_y, end_x, end_y)?;
    draw_jobs(ctx, ship)?;
    draw_grid(ctx, start_x, start_y, end_x, end_y)?;
    draw_units(ctx, ship)?;

    Ok(())
}

fn draw_tiles(
    ctx: &mut Context, ship: &Ship, start_x: i32, start_y: i32, end_x: i32, end_y: i32
) -> GameResult<()> {
    let mut floor_builder = MeshBuilder::new();
    let mut object_builder = MeshBuilder::new();
    for y in start_y..end_y {
        for x in start_x..end_x {
            let tile = ship.tiles().tile(Point2::new(x, y)).unwrap();

            let (fx, fy) = (x as f32, y as f32);

            // Add graphic for the floor
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

            // Add graphic for objects
            if tile.object.is_some() {
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

fn draw_jobs(
    ctx: &mut Context, ship: &Ship
) -> GameResult<()> {
    let mut jobs_builder = MeshBuilder::new();
    for (_, job) in ship.job_queue().jobs() {
        let (fx, fy) = (job.position().x as f32, job.position().y as f32);

        // Add graphic for the job
        jobs_builder.triangles(&[
            Point2::new(fx + 0.25, fy + 0.25),
            Point2::new(fx + 0.75, fy + 0.25),
            Point2::new(fx + 0.25, fy + 0.75),

            Point2::new(fx + 0.75, fy + 0.75),
            Point2::new(fx + 0.25, fy + 0.75),
            Point2::new(fx + 0.75, fy + 0.25),
        ]);
    }

    let jobs_mesh = jobs_builder.build(ctx)?;

    graphics::set_color(ctx, (255, 255, 255, 16).into())?;
    graphics::draw(ctx, &jobs_mesh, Point2::new(0.0, 0.0), 0.0)?;

    Ok(())
}

fn draw_grid(
    ctx: &mut Context, start_x: i32, start_y: i32, end_x: i32, end_y: i32
) -> GameResult<()> {
    // Draw a build grid
    let mut grid_builder = MeshBuilder::new();
    for y in start_y..(end_y+1) {
        grid_builder.line(
            &[Point2::new(start_x as f32, y as f32), Point2::new(end_x as f32, y as f32)],
            0.025
        );
    }
    for x in start_x..(end_x+1) {
        grid_builder.line(
            &[Point2::new(x as f32, start_y as f32), Point2::new(x as f32, end_y as f32)],
            0.02
        );
    }
    let grid_mesh = grid_builder.build(ctx)?;

    graphics::set_color(ctx, (255, 255, 255, 16).into())?;
    graphics::draw(ctx, &grid_mesh, Point2::new(0.0, 0.0), 0.0)?;

    Ok(())
}

fn draw_units(
    ctx: &mut Context, ship: &Ship
) -> GameResult<()> {
    let mut units_builder = MeshBuilder::new();
    for unit in ship.units() {
        let pos = unit.position();
        units_builder.triangles(&[
            Point2::new(pos.x - 0.4, pos.y - 0.4),
            Point2::new(pos.x + 0.4, pos.y - 0.4),
            Point2::new(pos.x - 0.4, pos.y + 0.4),

            Point2::new(pos.x + 0.4, pos.y + 0.4),
            Point2::new(pos.x - 0.4, pos.y + 0.4),
            Point2::new(pos.x + 0.4, pos.y - 0.4),
        ]);
    }
    let units_mesh = units_builder.build(ctx)?;

    graphics::set_color(ctx, (150, 200, 150).into())?;
    graphics::draw(ctx, &units_mesh, Point2::new(0.0, 0.0), 0.0)?;

    Ok(())
}
