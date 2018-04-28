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

        if !object.values.contains_key("is_opening") {
            object.values.insert("is_opening".to_string(), 0.0);
        }
        if !object.values.contains_key("openness") {
            object.values.insert("openness".to_string(), 0.0);
        }

        let is_opening = object.values["is_opening"] == 1.0;
        let mut openess = object.values["openness"];

        if is_opening {
            openess += delta / OPEN_TIME;
        } else {
            openess -= delta / OPEN_TIME;
        }

        openess = openess.min(1.0).max(0.0);

        *object.values.get_mut("openness").unwrap() = openess;
    }
}
