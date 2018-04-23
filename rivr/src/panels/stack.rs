use {
    nalgebra::{Point2},
    cassowary::{
        Solver, Expression, Variable,
        WeightedRelation::*,
        strength::{MEDIUM},
    },

    Ui, PanelId, Error,
    attributes::{PanelSize, PanelBox, Orientation},
    layouting::{LayoutVariables, PanelLayout},
    panels::{Panel},
    rendering::{Renderer},
};


pub struct StackPanel {
    size: PanelSize,
    panel_box: PanelBox,
    orientation: Orientation,

    children: Vec<PanelId>,
}

impl StackPanel {
    pub fn new(size: PanelSize, panel_box: PanelBox, orientation: Orientation) -> Self {
        StackPanel {
            size,
            panel_box,
            orientation,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, panel: PanelId) {
        self.children.push(panel);
    }

    fn constrain_axis_to_children<F: Fn(&LayoutVariables) -> Variable>(
        &self, solver: &mut Solver,  ui: &Ui, axis: Variable, map: F,
    ) {
        let mut expression = Expression::from_constant(0.0);
        for child_id in &self.children {
            let child = &ui.get(*child_id).unwrap().layout.variables;
            expression = expression + map(child);
        }
        solver.add_constraint(axis |GE(MEDIUM)| expression).unwrap();
    }
}

impl Panel for StackPanel {
    fn children(&self) -> Option<&Vec<PanelId>> {
        Some(&self.children)
    }

    fn add_constraints(
        &self,
        solver: &mut Solver, ui: &Ui,
        this: &LayoutVariables, parent: &LayoutVariables,
        c_depth: f64,
    ) {
        self.size.add_constraints(solver, this, parent, c_depth);

        // Prefer a size that at least contains all children
        match self.orientation {
            Orientation::Horizontal =>
                self.constrain_axis_to_children(solver, ui, this.width, |c| c.width),
            Orientation::Vertical =>
                self.constrain_axis_to_children(solver, ui, this.height, |c| c.height),
        }
    }

    fn render(
        &self, renderer: &mut Renderer, ui: &Ui, this_id: PanelId, this_layout: &PanelLayout,
    ) -> Result<(), Error> {
        self.panel_box.render(renderer, this_id, this_layout)?;

        let mut stack_position = 0.0;
        for child_id in &self.children {
            let child = ui.get(*child_id).unwrap();

            let position = match self.orientation {
                Orientation::Horizontal => {
                    let position = Point2::new(stack_position, 0.0);
                    stack_position += child.layout.size.x;
                    position
                },
                Orientation::Vertical => {
                    let position = Point2::new(0.0, stack_position);
                    stack_position += child.layout.size.y;
                    position
                },
            };

            renderer.render_cache(this_id, *child_id, position)?;
        }

        Ok(())
    }
}
