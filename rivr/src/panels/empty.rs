use {
    cassowary::{Solver},

    attributes::{PanelSize, PanelBox},
    layouting::{LayoutVariables, PanelLayout},
    input::{FrameCollision},
    panels::{Panel},
    rendering::{Renderer},
    Ui, PanelId, Error,
};

pub struct EmptyPanel {
    size: PanelSize,
    panel_box: PanelBox,
}

impl EmptyPanel {
    pub fn new(size: PanelSize, panel_box: PanelBox) -> Self {
        EmptyPanel {
            size,
            panel_box,
        }
    }
}

impl Panel for EmptyPanel {
    fn add_constraints(
        &self,
        solver: &mut Solver, _ui: &Ui,
        this: &LayoutVariables,
        c_depth: f64,
    ) {
        self.size.add_constraints(solver, this, c_depth);
    }

    fn render(
        &self,
        _ui: &Ui, renderer: &mut Renderer,
        this_id: PanelId, this_layout: &PanelLayout,
        _frame: &mut FrameCollision,
    ) -> Result<(), Error> {
        self.panel_box.render(renderer, this_id, this_layout, false)?;

        Ok(())
    }
}
