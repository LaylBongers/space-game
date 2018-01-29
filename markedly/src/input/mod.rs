//! Systems for handling user input

use nalgebra::{Point2};

use render::{Renderer};
use {Ui, ComponentId};

/// Handles user input, raising events on components and storing current input information.
pub struct UiInput {
    hovering_over: Option<ComponentId>,
}

impl UiInput {
    pub fn new() -> Self {
        UiInput {
            hovering_over: None,
        }
    }

    pub fn is_cursor_over_ui(&self) -> bool {
        self.hovering_over.is_some()
    }

    pub fn handle_cursor_moved<R: Renderer>(&mut self, position: Point2<f32>, ui: &Ui<R>) {
        self.hovering_over = find_at_position(position, ui, ui.root_id());
    }

    pub fn handle_drag_started<R: Renderer>(
        &mut self, _position: Point2<f32>, _ui: &mut Ui<R>
    ) {
    }

    pub fn handle_drag_ended<R: Renderer>(
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

    // If this component doesn't capture input, we still need to check children, but we can't
    // return this one.
    let mut found_id = if component.class.is_capturing_cursor() {
        Some(id)
    } else {
        None
    };

    // Go through all children, if any of them find a hit, replace the ID we found, we want to find
    // the last one that matches because it's the one rendered on top. The function will
    // recursively find the deepest matching child like this.
    for child_id in &component.children {
        if let Some(id) = find_at_position(position, ui, *child_id) {
            found_id = Some(id);
        }
    }

    found_id
}
