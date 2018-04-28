use {
    object_class::{ObjectBehavior},
    pathfinding::{Walkable},
    state::ship::{Object},
};

pub struct DoorObjectBehavior;

impl ObjectBehavior for DoorObjectBehavior {
    fn walkable(&self) -> Walkable {
        Walkable::Openable
    }

    fn initialize(&self, object: &mut Object) {
        object.values.insert("is_opening".to_string(), 0.0);
        object.values.insert("openness".to_string(), 0.0);
    }

    fn update(&self, object: &mut Object, delta: f32) {
        const OPEN_TIME: f32 = 0.25;

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
