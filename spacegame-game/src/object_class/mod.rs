mod door;

pub use self::{
    door::{DoorObjectBehavior},
};

use {
    ggez::graphics::{Rect},

    state::ship::{Object},
};

pub struct ObjectClasses {
    classes: Vec<ObjectClass>,
}

impl ObjectClasses {
    pub fn new() -> Self {
        ObjectClasses {
            classes: Vec::new()
        }
    }

    pub fn classes(&self) -> &Vec<ObjectClass> {
        &self.classes
    }

    pub fn register(&mut self, class: ObjectClass) -> ObjectClassId {
        self.classes.push(class);
        ObjectClassId { id: self.classes.len() - 1 }
    }

    pub fn get(&self, id: ObjectClassId) -> Option<&ObjectClass> {
        self.classes.get(id.id)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct ObjectClassId {
    pub id: usize,
}

pub struct ObjectClass {
    pub friendly_name: String,
    pub uvs: Rect,
    pub walk_cost: WalkCost,

    pub behavior: Option<Box<ObjectBehavior>>,
}

#[derive(PartialEq, Copy, Clone)]
pub enum WalkCost {
    NotWalkable,
    Multiplier(f32),
}

pub trait ObjectBehavior {
    fn update(&self, object: &mut Object, delta: f32);
}
