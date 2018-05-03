mod ship;
mod tasks;
mod unit;

pub use self::{
    ship::{Ship},
    tasks::{TaskQueue, TaskId, Task},
    unit::{Unit},
};

use {
    metrohash::{MetroHashMap},

    object_class::{ObjectClassId, ObjectClasses},
    pathfinding::{Walkable},
    Error,
};

#[derive(Deserialize, Serialize)]
pub struct Tile {
    pub floor: bool,
    pub object: Option<Object>,
}

impl Tile {
    pub fn walkable(&self, object_classes: &ObjectClasses) -> Result<Walkable, Error> {
        if !self.floor {
            return Ok(Walkable::Never)
        }

        Ok(if let Some(ref object) = self.object {
            let class = object_classes.get(object.class)?;
            if let Some(ref behavior) = class.behavior {
                behavior.walkable()
            } else {
                Walkable::Never
            }
        } else {
            Walkable::Always
        })
    }
}

impl Default for Tile {
    fn default() -> Self {
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
