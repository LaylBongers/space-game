use ggez::{Context, GameResult};
use ggez::graphics::spritebatch::{SpriteBatch};
use ggez::graphics::{self, MeshBuilder, DrawParam, Rect};
use nalgebra::{Point2};

use model::{Camera};
use model::ship::{Ship};

pub struct Bounds {
    pub start: Point2<i32>,
    pub end: Point2<i32>,
}

impl Bounds {
    pub fn calculate(ship: &Ship, camera: &Camera) -> Self {
        let (start, end) = camera.world_bounds();
        let size = ship.tiles.size();
        let start_x = (start.x.floor() as i32).max(0);
        let start_y = (start.y.floor() as i32).max(0);
        let end_x = (end.x.ceil() as i32).min(size.x);
        let end_y = (end.y.ceil() as i32).min(size.y);

        Bounds {
            start: Point2::new(start_x, start_y),
            end: Point2::new(end_x, end_y),
        }
    }
}

pub fn draw_ship(
    ctx: &mut Context, ship: &Ship, camera: &Camera, tiles: &mut SpriteBatch,
) -> GameResult<()> {

    draw_tiles(ctx, ship, camera, tiles)?;
    draw_tasks(ctx, ship)?;
    draw_units(ctx, ship)?;

    Ok(())
}

fn draw_tiles(
    ctx: &mut Context, ship: &Ship, camera: &Camera, tiles: &mut SpriteBatch,
) -> GameResult<()> {
    let bounds = Bounds::calculate(ship, camera);

    for y in bounds.start.y..bounds.end.y {
        for x in bounds.start.x..bounds.end.x {
            let tile = ship.tiles.tile(Point2::new(x, y)).unwrap();

            let (fx, fy) = (x as f32, y as f32);

            // Add graphic for the floor
            if tile.floor {
                tiles.add(DrawParam {
                    src: Rect::new(0.0, 0.5, 0.5, 0.5),
                    dest: Point2::new(fx, fy + 1.0),
                    scale: Point2::new(1.0 / 64.0, -1.0 / 64.0),
                    .. Default::default()
                });
            }

            // Add graphic for objects
            if tile.object.is_some() {
                tiles.add(DrawParam {
                    src: Rect::new(0.0, 0.0, 0.5, 0.5),
                    dest: Point2::new(fx, fy + 1.0),
                    scale: Point2::new(1.0 / 64.0, -1.0 / 64.0),
                    .. Default::default()
                });
            }
        }
    }

    graphics::set_color(ctx, (255, 255, 255).into())?;
    graphics::draw(ctx, tiles, Point2::new(0.0, 0.0), 0.0)?;
    tiles.clear();

    Ok(())
}

fn draw_tasks(
    ctx: &mut Context, ship: &Ship
) -> GameResult<()> {
    let mut tasks_builder = MeshBuilder::new();
    let mut unreachable_tasks_builder = MeshBuilder::new();

    for (_, task) in ship.task_queue.tasks() {
        let (fx, fy) = (task.position().x as f32, task.position().y as f32);

        let builder = if !task.unreachable() {
            &mut tasks_builder
        } else {
            &mut unreachable_tasks_builder
        };

        // Add graphic for the task
        builder.triangles(&[
            Point2::new(fx + 0.25, fy + 0.25),
            Point2::new(fx + 0.75, fy + 0.25),
            Point2::new(fx + 0.25, fy + 0.75),

            Point2::new(fx + 0.75, fy + 0.75),
            Point2::new(fx + 0.25, fy + 0.75),
            Point2::new(fx + 0.75, fy + 0.25),
        ]);
    }

    let tasks_mesh = tasks_builder.build(ctx)?;
    let unreachable_tasks_mesh = unreachable_tasks_builder.build(ctx)?;

    graphics::set_color(ctx, (255, 255, 255, 25).into())?;
    graphics::draw(ctx, &tasks_mesh, Point2::new(0.0, 0.0), 0.0)?;

    graphics::set_color(ctx, (255, 120, 120, 50).into())?;
    graphics::draw(ctx, &unreachable_tasks_mesh, Point2::new(0.0, 0.0), 0.0)?;

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
