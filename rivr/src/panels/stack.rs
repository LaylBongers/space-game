use {
    nalgebra::{Point2},
    cassowary::{
        Solver, Expression, Variable,
        WeightedRelation::*,
        strength::{MEDIUM},
    },

    Ui, PanelId, Error,
    attributes::{PanelSize, PanelBox, Orientation},
    input::{FrameCollision},
    layouting::{LayoutVariables, PanelLayout},
    panels::{Panel},
    rendering::{Renderer},
};


pub struct StackPanel {
    size: PanelSize,
    panel_box: PanelBox,
    orientation: Orientation,
    margin: f32,

    children: Vec<PanelId>,
}

impl StackPanel {
    pub fn new(
        size: PanelSize, panel_box: PanelBox, orientation: Orientation, margin: f32
    ) -> Self {
        StackPanel {
            size,
            panel_box,
            orientation,
            margin,

            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, panel: PanelId) {
        self.children.push(panel);
    }

    fn constrain_axis_to_children<F1, F2>(
        &self, solver: &mut Solver, ui: &Ui,
        this: &LayoutVariables, major_axis_map: F1, minor_axis_map: F2,
    ) where
        F1: Fn(&LayoutVariables) -> Variable,
        F2: Fn(&LayoutVariables) -> Variable,
    {
        let mut major_total_width = Expression::from_constant(self.margin as f64);
        let mut major_total_margin = 0.0;

        if self.children.len() != 0 {
            for child_id in &self.children {
                let child = &ui.get(*child_id).unwrap().layout.variables;

                major_total_width = major_total_width + major_axis_map(child);
                major_total_margin += self.margin;

                // We need to size our minor axis to be bigger than the size of children + margin
                solver.add_constraint(
                    minor_axis_map(this)
                    |GE(MEDIUM)|
                    minor_axis_map(child) + (self.margin * 2.0)
                ).unwrap();
            }
        } else {
            // If we don't have any children at all, we need to do some corrections to still get
            // a valid size based on the margins
            if self.margin != 0.0 {
                major_total_margin += self.margin;
                solver.add_constraint(
                    minor_axis_map(this) |GE(MEDIUM)| self.margin * 2.0
                ).unwrap();
            }
        }

        solver.add_constraint(
            major_axis_map(this) |GE(MEDIUM)| major_total_width + major_total_margin
        ).unwrap();
    }
}

impl Panel for StackPanel {
    fn visible_children(&self) -> Option<&Vec<PanelId>> {
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
                self.constrain_axis_to_children(solver, ui, this, |c| c.width, |c| c.height),
            Orientation::Vertical =>
                self.constrain_axis_to_children(solver, ui, this, |c| c.height, |c| c.width),
        }
    }

    fn render(
        &self,
        renderer: &mut Renderer, ui: &Ui, this_id: PanelId, this_layout: &PanelLayout,
        frame: &mut FrameCollision,
    ) -> Result<(), Error> {
        self.panel_box.render(renderer, this_id, this_layout)?;

        let mut stack_position = self.margin;
        for child_id in &self.children {
            let child = ui.get(*child_id).unwrap();

            let position = match self.orientation {
                Orientation::Horizontal => {
                    let position = Point2::new(stack_position, self.margin);
                    stack_position += child.layout.size.x + self.margin;
                    position
                },
                Orientation::Vertical => {
                    let position = Point2::new(self.margin, stack_position);
                    stack_position += child.layout.size.y + self.margin;
                    position
                },
            };

            renderer.render_cache(this_id, *child_id, position)?;
            frame.set(*child_id, position, child.layout.size);
        }

        Ok(())
    }
}
