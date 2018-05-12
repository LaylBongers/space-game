mod grid;

pub use self::grid::{Grid, Error, IterPos, IterBounds};

use {
    nalgebra::{Vector2, Point2},
    serde::{Serialize, de::DeserializeOwned},
};

pub type Tiles<Tile> = Grid<Tile, Dim2>;

pub trait Dim {
    type Vector: Serialize + DeserializeOwned + Copy;
    type Point: Serialize + DeserializeOwned + Copy;

    fn start() -> Self::Point;
    fn area(size: Self::Vector) -> usize;
    fn is_in_bounds(position: Self::Point, size: Self::Vector) -> bool;
    fn index(position: Self::Point, size: Self::Vector) -> usize;
    fn next(position: Self::Point, size: Self::Vector) -> Option<Self::Point>;
}

pub enum Dim2 {}

impl Dim for Dim2 {
    type Vector = Vector2<i32>;
    type Point = Point2<i32>;

    fn start() -> Self::Point {
        Point2::new(0, 0)
    }

    fn area(size: Vector2<i32>) -> usize {
        (size.x * size.y) as usize
    }

    fn is_in_bounds(position: Point2<i32>, size: Vector2<i32>) -> bool {
        position.x >= 0 && position.y >= 0
            && position.x < size.x && position.y < size.y
    }

    fn index(position: Point2<i32>, size: Vector2<i32>) -> usize {
        (position.x + (position.y * size.x)) as usize
    }

    fn next(mut position: Point2<i32>, size: Vector2<i32>) -> Option<Point2<i32>> {
        position.x += 1;
        if position.x >= size.x {
            position.x = 0;
            position.y += 1;
        }

        if position.y >= size.y {
            None
        } else {
            Some(position)
        }
    }
}
