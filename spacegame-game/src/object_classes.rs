use ggez::graphics::{Rect};

pub trait ObjectClass {
    fn uvs(&self) -> Rect;
    fn is_walkable(&self) -> bool;
}

pub struct ObjectClasses {
    classes: Vec<ObjectClassEntry>,
}

impl ObjectClasses {
    pub fn new() -> Self {
        ObjectClasses {
            classes: Vec::new()
        }
    }

    pub fn classes(&self) -> &Vec<ObjectClassEntry> {
        &self.classes
    }

    pub fn register<S: Into<String>, C: ObjectClass + 'static>(
        &mut self, friendly_name: S, class: C
    ) -> ObjectClassId {
        self.classes.push(ObjectClassEntry {
            friendly_name: friendly_name.into(),
            class: Box::new(class),
        });
        ObjectClassId { id: self.classes.len() - 1 }
    }

    pub fn get(&self, id: ObjectClassId) -> Option<&ObjectClass> {
        self.classes.get(id.id)
            .map(|v| v.class.as_ref())
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct ObjectClassId {
    pub id: usize,
}

pub struct ObjectClassEntry {
    pub friendly_name: String,
    pub class: Box<ObjectClass>,
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
