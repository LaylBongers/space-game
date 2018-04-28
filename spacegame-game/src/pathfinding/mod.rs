use {
    nalgebra::{Point2},
    pathfindingc::{astar},

    state::ship::{Tiles, Tile, TilesError},
    ObjectClasses,
};

/// Finds a path to the goal, returns None if no path could be found.
pub fn find_path(
    start: Point2<i32>, goal: Point2<i32>, goal_inclusive: bool,
    tiles: &Tiles, object_classes: &ObjectClasses,
) -> Option<Vec<Point2<i32>>> {
    // Calculate some advance values relevant to pathfinding
    let costs = Costs {
        straight: 100,
        diagonal: (f32::sqrt(2.0) * 100.0) as i32,
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

struct Costs {
    // Can only use Ord cost, f32 isn't Ord
    straight: i32,
    diagonal: i32,
}

fn neighbors(
    node: Point2<i32>, start: Point2<i32>, goal: Point2<i32>,
    goal_inclusive: bool, costs: &Costs,
    tiles: &Tiles, object_classes: &ObjectClasses
) -> Vec<(Point2<i32>, i32)> {
    let mut neighbors = Vec::new();

    for y in node.y-1..node.y+2 {
        for x in node.x-1..node.x+2 {
            let neighbor = Point2::new(x, y);

            // If this is the same one, skip it
            if node == neighbor {
                continue
            }

            // Retrieve the tile data itself
            let tile_res =  tiles.get(neighbor);

            // Make sure we can walk over this tile
            // We always allow the start, we want to move off where we are even if it's blocked
            // We start pathing at the goal anyways, so we don't have to add an exception for that
            if !is_walkable(tile_res, object_classes) &&
                !(neighbor == start)
            {
                continue
            }

            // Cost differ for straight and diagonal movement
            let cost = if x == node.x || y == node.y {
                costs.straight
            } else {
                // Hard corner check is not needed if we don't actually need to move to it, which
                // is the case if we're not goal inclusive and the node is the goal
                if !(!goal_inclusive && node == goal) {
                    // Make sure we're not moving through a hard corner
                    if !is_walkable(tiles.get(Point2::new(x, node.y)), object_classes) ||
                       !is_walkable(tiles.get(Point2::new(node.x, y)), object_classes) {
                        continue
                    }
                }

                costs.diagonal
            };

            neighbors.push((neighbor, cost));
        }
    }

    neighbors
}

fn is_walkable(tile_res: Result<&Tile, TilesError>, object_classes: &ObjectClasses) -> bool {
    if let Ok(tile) = tile_res {
        tile.floor && tile.object
            .as_ref()
            .map(|o| object_classes.get(o.class).unwrap().is_walkable)
            .unwrap_or(true)
    } else {
        false
    }
}

fn heuristic(node: Point2<i32>, start: Point2<i32>, costs: &Costs) -> i32 {
    let dx = (node.x - start.x).abs();
    let dy = (node.y - start.y).abs();
    costs.straight*(dx + dy) + (costs.diagonal - 2*costs.straight) * dx.min(dy)
}
