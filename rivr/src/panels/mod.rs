use std::any::{Any};
use {Size, Orientation, PanelId};

pub trait Panel: Any {
}

pub struct EmptyPanel {
    size: (Size, Size),
    draw_background: bool,
}

impl EmptyPanel {
    pub fn new(size: (Size, Size), draw_background: bool) -> Self {
        EmptyPanel {
            size,
            draw_background,
        }
    }
}

impl Panel for EmptyPanel {
}

pub struct StackPanel {
    orientation: Orientation,
    children: Vec<PanelId>
}

impl StackPanel {
    pub fn new(orientation: Orientation) -> Self {
        StackPanel {
            orientation,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, panel: PanelId) {
        self.children.push(panel);
    }
}

impl Panel for StackPanel {
}
