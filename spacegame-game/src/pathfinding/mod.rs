use {
    nalgebra::{Point2},
    pathfindingc::{astar},

    mtk::{self, grid::{Tiles}},

    object_class::{ObjectClasses},
    state::ship::{Tile},
    Error,
};

const COST_MULTIPLIER: f32 = 100.0;

/// Finds a path to the goal, returns None if no path could be found.
pub fn find_path(
    start: Point2<i32>, goal: Point2<i32>, goal_inclusive: bool, seconds_per_unit: f32,
    tiles: &Tiles<Tile>, object_classes: &ObjectClasses,
) -> Option<Vec<Point2<i32>>> {
    // Calculate some advance values relevant to pathfinding
    let costs = Costs {
        straight: seconds_per_unit,
        diagonal: f32::sqrt(2.0) * seconds_per_unit,
    };

    // Now do the actual pathfinding
    // Keep in mind our path following wants the path in reverse, so we A* in reverse
    let result = astar::astar(
        &goal,
        |node| neighbors(*node, start, goal, goal_inclusive, &costs, tiles, object_classes),
        |node| heuristic(*node, start, &costs),
        |node| *node == start,
    );

    if let Some((mut path, _cost)) = result {
        // If we don't need the goal, remove it
        if !goal_inclusive {
            path.remove(0);
        }

        // Everything's set for path following, return that we found a path
        Some(path)
    } else {
        // We didn't find a path, return that this is unreachable
        None
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum Walkable {
    Never,
    Always,
    Openable,
}

impl Walkable {
    pub fn from_tile_res(
        tile_res: Result<&Tile, mtk::grid::Error>, object_classes: &ObjectClasses
    ) -> Result<Walkable, Error> {
        if let Ok(tile) = tile_res {
            tile.walkable(object_classes)
        } else {
            Ok(Walkable::Never)
        }
    }
}

struct Costs {
    // Can only use Ord cost, f32 isn't Ord
    straight: f32,
    diagonal: f32,
}

fn neighbors(
    node: Point2<i32>, start: Point2<i32>, goal: Point2<i32>,
    goal_inclusive: bool, costs: &Costs,
    tiles: &Tiles<Tile>, object_classes: &ObjectClasses
) -> Vec<(Point2<i32>, i32)> {
    let mut neighbors = Vec::new();

    for y in node.y-1..node.y+2 {
        for x in node.x-1..node.x+2 {
            let neighbor = Point2::new(x, y);

            // If this is the same one, skip it
            if node == neighbor {
                continue
            }

            // Make sure we can walk over this tile
            let walkable = Walkable::from_tile_res(tiles.get(neighbor), object_classes).unwrap();
            if walkable == Walkable::Never {
                // We always allow the start, we want to move off where we are even if it's
                // blocked. We start pathing at the goal anyways, so we don't have to add an
                // exception for that
                if neighbor != start {
                    continue
                }
            }

            // TODO: The way walk cost is used in calculations is technically currently incorrect,
            // because moving to this tile we only spend half of the time on this one and half on
            // the previous one. Therefore, both should be taken into account.

            // Cost differ for straight and diagonal movement
            let cost = if x == node.x || y == node.y {
                (costs.straight * COST_MULTIPLIER) as i32
            } else {
                // Hard corner check is not needed if we don't actually need to move to it, which
                // is the case if we're not goal inclusive and the node is the goal
                if !(!goal_inclusive && node == goal) {
                    // Make sure we're not moving through a hard corner
                    if !is_walkable(tiles.get(Point2::new(x, node.y)), object_classes).unwrap() ||
                       !is_walkable(tiles.get(Point2::new(node.x, y)), object_classes).unwrap() {
                        continue
                    }
                }

                (costs.diagonal * COST_MULTIPLIER) as i32
            };

            neighbors.push((neighbor, cost));
        }
    }

    neighbors
}

fn is_walkable(
    tile_res: Result<&Tile, mtk::grid::Error>, object_classes: &ObjectClasses
) -> Result<bool, Error> {
    Ok(Walkable::from_tile_res(tile_res, object_classes)? != Walkable::Never)
}

fn heuristic(node: Point2<i32>, start: Point2<i32>, costs: &Costs) -> i32 {
    let cost_straight = (costs.straight * COST_MULTIPLIER) as i32;
    let cost_diagonal = (costs.diagonal * COST_MULTIPLIER) as i32;

    let dx = (node.x - start.x).abs();
    let dy = (node.y - start.y).abs();
    cost_straight*(dx + dy) + (cost_diagonal - 2*cost_straight) * dx.min(dy)
}
