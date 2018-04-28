use {
    nalgebra::{Vector2, Point2},
    metrohash::{MetroHashMap},

    ObjectClassId, ObjectClasses,
};

#[derive(Deserialize, Serialize)]
pub struct Tiles {
    tiles: Vec<Tile>,
    size: Vector2<i32>,

    tiles_with_behaviors: Vec<usize>,
    changed: bool,
}

impl Tiles {
    pub fn empty(size: Vector2<i32>) -> Self {
        let amount = (size.x * size.y) as usize;
        let mut tiles = Vec::with_capacity(amount);
        for _ in 0..amount { tiles.push(Tile::empty()) }

        Tiles {
            tiles,
            size,

            tiles_with_behaviors: Vec::new(),
            changed: false,
        }
    }

    pub fn size(&self) -> Vector2<i32> {
        self.size
    }

    pub fn get(&self, position: Point2<i32>) -> Result<&Tile, TilesError> {
        if self.is_in_bounds(position) {
            Ok(&self.tiles[self.index(position)])
        } else {
            Err(TilesError::OutOfBounds { position })
        }
    }

    pub fn get_mut(&mut self, position: Point2<i32>) -> Result<&mut Tile, TilesError> {
        if self.is_in_bounds(position) {
            let index = self.index(position);
            Ok(&mut self.tiles[index])
        } else {
            Err(TilesError::OutOfBounds { position })
        }
    }

    pub fn is_in_bounds(&self, position: Point2<i32>) -> bool {
        position.x >= 0 && position.y >= 0
            && position.x < self.size.x && position.y < self.size.y
    }

    pub fn mark_changed(&mut self) {
        self.changed = true
    }

    pub fn handle_changed(&mut self, object_classes: &ObjectClasses) -> bool {
        if self.changed {
            // Find any tiles that ask for update events
            self.tiles_with_behaviors.clear();
            for (i, tile) in self.tiles.iter().enumerate() {
                if let Some(ref object) = tile.object {
                    let class = object_classes.get(object.class).unwrap();
                    if class.behavior.is_some() {
                        self.tiles_with_behaviors.push(i);
                    }
                }
            }

            self.changed = false;
            true
        } else {
            false
        }
    }

    pub fn update(&mut self, object_classes: &ObjectClasses, delta: f32) {
        for i in &self.tiles_with_behaviors {
            let object = self.tiles[*i]
                .object.as_mut()
                    .expect("Found tile without object in tiles with behaviors");
            let behavior = object_classes.get(object.class).unwrap()
                .behavior.as_ref()
                    .expect("Found tile class without behavior in tiles with behaviors");

            behavior.update(object, delta);
        }
    }

    fn index(&self, position: Point2<i32>) -> usize {
        (position.x + (position.y * self.size.x)) as usize
    }
}

#[derive(Debug, PartialEq)]
pub enum TilesError {
    OutOfBounds { position: Point2<i32> },
}

#[derive(Deserialize, Serialize)]
pub struct Tile {
    pub floor: bool,
    pub object: Option<Object>,
}

impl Tile {
    pub fn empty() -> Self {
        Tile {
            floor: false,
            object: None,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Object {
    pub class: ObjectClassId,

    pub values: MetroHashMap<String, f32>,
}

impl Object {
    pub fn new(class: ObjectClassId) -> Self {
        Object {
            class,

            values: MetroHashMap::default(),
        }
    }
}
