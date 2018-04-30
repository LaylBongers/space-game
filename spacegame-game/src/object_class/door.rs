use {
    object_class::{ObjectBehavior},
    pathfinding::{Walkable},
    state::ship::{Object},
};

pub struct DoorObjectBehavior;

const DOOR_MOVE_TIME: f32 = 0.5;

impl ObjectBehavior for DoorObjectBehavior {
    fn walkable(&self) -> Walkable {
        Walkable::Openable
    }

    fn initialize(&self, object: &mut Object) {
        object.values.insert("openness".to_string(), 0.0);
        object.values.insert("cooldown".to_string(), 0.0);
    }

    fn update(&self, object: &mut Object, delta: f32) {

        let mut openess = object.values["openness"];
        let mut cooldown = object.values["cooldown"];

        cooldown -= delta;
        if cooldown <= 0.0 {
            openess -= delta / DOOR_MOVE_TIME;
        }

        cooldown = cooldown.max(0.0);
        openess = openess.min(1.0).max(0.0);

        *object.values.get_mut("openness").unwrap() = openess;
        *object.values.get_mut("cooldown").unwrap() = cooldown;
    }

    fn is_open(&self, object: &Object) -> bool {
        object.values["openness"] >= 1.0
    }

    fn work_open(&self, object: &mut Object, delta: f32) -> bool {
        *object.values.get_mut("openness").unwrap() += (delta * 2.0) / DOOR_MOVE_TIME;
        *object.values.get_mut("cooldown").unwrap() = 1.0;
        self.is_open(object)
    }
}
