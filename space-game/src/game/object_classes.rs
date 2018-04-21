use ggez::graphics::{Rect};

pub trait ObjectClass {
    fn uvs(&self) -> Rect;
    fn is_walkable(&self) -> bool;
}

#[derive(Copy, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct ObjectClassId(pub i32);

pub struct ObjectClasses {
    classes: Vec<Box<ObjectClass>>,
}

impl ObjectClasses {
    pub fn new() -> Self {
        ObjectClasses {
            classes: Vec::new()
        }
    }

    pub fn register<C: ObjectClass + 'static>(&mut self, class: C) -> ObjectClassId {
        self.classes.push(Box::new(class));
        ObjectClassId(self.classes.len() as i32 - 1)
    }

    pub fn get(&self, id: ObjectClassId) -> Option<&ObjectClass> {
        self.classes.get(id.0 as usize).map(|v| v.as_ref())
    }
}

pub struct GenericObjectClass {
    pub uvs: Rect,
    pub walkable: bool,
}

impl ObjectClass for GenericObjectClass {
    fn uvs(&self) -> Rect {
        self.uvs
    }

    fn is_walkable(&self) -> bool {
        self.walkable
    }
}
