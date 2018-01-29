use nalgebra::{Point2};

use render::{Renderer};
use {Ui, ComponentId};

pub struct UiInput {
}

impl UiInput {
    pub fn new() -> Self {
        UiInput {
        }
    }

    pub fn start_drag<R: Renderer>(
        &mut self, _position: Point2<f32>, _ui: &mut Ui<R>
    ) {
    }

    pub fn end_drag<R: Renderer>(
        &mut self, position: Point2<f32>, ui: &mut Ui<R>
    ) {
        if let Some(component_id) = find_at_position(position, ui, ui.root_id()) {
            let component = ui.get_mut(component_id).unwrap();
            component.class.pressed_event();
        }
    }
}

fn find_at_position<R: Renderer>(
    position: Point2<f32>, ui: &Ui<R>, id: ComponentId,
) -> Option<ComponentId> {
    // If the position isn't over us, it also won't be over any children, so just return none
    let component = ui.get(id).unwrap();
    if position.x < component.position.x ||
        position.y < component.position.y ||
        position.x > component.position.x + component.size.x ||
        position.y > component.position.y + component.size.y {
        return None
    }

    let mut found_id = id;

    // Go through all children, if any of them find a hit, replace the ID we found, we want to find
    // the last one that matches because it's the one rendered on top. The function will
    // recursively find the deepest matching child like this.
    for child_id in &component.children {
        if let Some(id) = find_at_position(position, ui, *child_id) {
            found_id = id;
        }
    }

    Some(found_id)
}
