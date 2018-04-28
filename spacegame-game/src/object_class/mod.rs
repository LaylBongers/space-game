mod door;

pub use self::{
    door::{DoorObjectBehavior},
};

use {
    ggez::graphics::{Rect},

    pathfinding::{Walkable},
    state::ship::{Object},
    Error,
};

pub struct ObjectClasses {
    entries: Vec<ObjectClass>,
}

impl ObjectClasses {
    pub fn new() -> Self {
        ObjectClasses {
            entries: Vec::new()
        }
    }

    pub fn entries(&self) -> &Vec<ObjectClass> {
        &self.entries
    }

    pub fn register(&mut self, class: ObjectClass) -> ObjectClassId {
        self.entries.push(class);
        ObjectClassId { id: self.entries.len() - 1 }
    }

    pub fn get(&self, id: ObjectClassId) -> Result<&ObjectClass, Error> {
        self.entries.get(id.id)
            .ok_or(Error::ObjectClassNotFound(id))
    }

    pub fn create_object(&self, id: ObjectClassId) -> Result<Object, Error> {
        let class = self.get(id)?;

        let mut object = Object::new(id);
        if let Some(ref behavior) = class.behavior {
            behavior.initialize(&mut object);
        }

        Ok(object)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct ObjectClassId {
    pub id: usize,
}

pub struct ObjectClass {
    pub friendly_name: String,
    pub uvs: Rect,

    pub behavior: Option<Box<ObjectBehavior>>,
}

pub trait ObjectBehavior {
    fn walkable(&self) -> Walkable;

    fn initialize(&self, object: &mut Object);
    fn update(&self, object: &mut Object, delta: f32);
}
