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
    pub is_walkable: bool,

    pub behavior: Option<Box<ObjectBehavior>>,
}

pub trait ObjectBehavior {
    fn update(&self, object: &mut Object, delta: f32);
}

pub struct DoorObjectBehavior;

impl ObjectBehavior for DoorObjectBehavior {
    fn update(&self, object: &mut Object, delta: f32) {
        const OPEN_TIME: f32 = 0.25;

        if object.is_opening {
            object.openness += delta / OPEN_TIME;
        } else {
            object.openness -= delta / OPEN_TIME;
        }

        object.openness = object.openness.min(1.0).max(0.0);
    }
}
