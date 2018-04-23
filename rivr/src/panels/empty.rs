use {
    nalgebra::{Point2},
    palette::{Srgba},
    cassowary::{Solver},

    Ui, PanelId, Error,
    attributes::{PanelSize},
    layouting::{LayoutVariables, PanelLayout},
    panels::{Panel},
    rendering::{Renderer},
};

pub struct EmptyPanel {
    size: PanelSize,
    background: Option<Srgba>,
}

impl EmptyPanel {
    pub fn new(size: PanelSize, background: Option<Srgba>) -> Self {
        EmptyPanel {
            size,
            background,
        }
    }
}

impl Panel for EmptyPanel {
    fn children(&self) -> Option<&Vec<PanelId>> {
        None
    }

    fn add_constraints(
        &self,
        solver: &mut Solver, _ui: &Ui,
        this: &LayoutVariables, parent: &LayoutVariables,
        c_depth: f64,
    ) {
        self.size.add_constraints(solver, this, parent, c_depth);
    }

    fn render(
        &self, renderer: &mut Renderer, _ui: &Ui, this_id: PanelId, this_layout: &PanelLayout
    ) -> Result<(), Error> {
        if let Some(background) = self.background {
            renderer.render_vertices(this_id, &[
                Point2::new(0.0, 0.0),
                Point2::new(0.0, this_layout.size.y),
                Point2::new(this_layout.size.x, this_layout.size.y),
                Point2::new(this_layout.size.x, 0.0),
            ], &[0, 1, 3, 2, 3, 1], background)?;
        }

        Ok(())
    }
}
