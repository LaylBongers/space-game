use {
    nalgebra::{Point2},
    palette::{Srgba},
    cassowary::{Solver},

    Ui, PanelId, Error,
    attributes::{PanelSize, PanelBox, PanelText},
    input::{FrameCollision},
    layouting::{LayoutVariables, PanelLayout},
    panels::{Panel},
    rendering::{Renderer},
};

pub struct ButtonPanel {
    size: PanelSize,
    panel_box: PanelBox,
    label: Option<PanelText>,

    hovering: bool,
}

impl ButtonPanel {
    pub fn new(size: PanelSize, panel_box: PanelBox, label: Option<PanelText>) -> Self {
        ButtonPanel {
            size,
            panel_box,
            label,

            hovering: false,
        }
    }
}

impl Panel for ButtonPanel {
    fn add_constraints(
        &self,
        solver: &mut Solver, _ui: &Ui,
        this: &LayoutVariables, parent: &LayoutVariables,
        c_depth: f64,
    ) {
        self.size.add_constraints(solver, this, parent, c_depth);
    }

    fn render(
        &self,
        renderer: &mut Renderer, _ui: &Ui, this_id: PanelId, this_layout: &PanelLayout,
        _frame: &mut FrameCollision,
    ) -> Result<(), Error> {
        self.panel_box.render(renderer, this_id, this_layout, self.hovering)?;

        if let Some(ref label) = self.label {
            renderer.render_text(
                this_id,
                &label.text, label.size,
                Point2::new(0.0, 0.0), this_layout.size, Srgba::new(0.0, 0.0, 0.0, 1.0),
            )?;
        }

        Ok(())
    }

    fn is_capturing_cursor(&self) -> bool { true }

    fn handle_hover_start(&mut self) -> bool { self.hovering = true; true }

    fn handle_hover_end(&mut self) -> bool { self.hovering = false; true }

    fn handle_pressed(&mut self) {}
}
