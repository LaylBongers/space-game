use nalgebra::{Point2};

use render::{Renderer};
use {Ui};

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
        &mut self, _position: Point2<f32>, _ui: &mut Ui<R>
    ) {
        //let component = find_at_position(position, root_component);
    }
}

/*fn find_at_position<'a, R: Renderer>(
    position: Point2<f32>, root_component: &'a mut Component<R>
) -> &'a mut Component<R> {
    // We want the *last* child that matches the position, because it will be the last one rendered

    for i in root_component.children.len()..0 {
        let child = &mut root_component.children[i];

        if position.x < child.position.x ||
            position.y < child.position.y ||
            position.x > child.position.x + child.size.x ||
            position.y > child.position.y + child.size.y {
            continue
        }

        return find_at_position(position, child);
    }

    root_component
}*/
