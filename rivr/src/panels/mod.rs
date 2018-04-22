use {
    std::any::{Any},

    cassowary::{
        Solver, Expression,
        WeightedRelation::*,
        strength::{STRONG, WEAK},
    },

    Size, Orientation, Ui, PanelId,
    layouting::{LayoutVariables},
};

pub trait Panel: Any {
    /// Return a vector of the children of this panel, if applicable.
    fn children(&self) -> Option<&Vec<PanelId>>;

    fn add_constraints(
        &self, solver: &mut Solver, ui: &Ui, this: &LayoutVariables, parent: &LayoutVariables
    );
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
    fn children(&self) -> Option<&Vec<PanelId>> {
        None
    }

    fn add_constraints(
        &self, solver: &mut Solver, _ui: &Ui,
        this: &LayoutVariables, parent: &LayoutVariables
    ) {
        // Restrict to parent sizes
        solver.add_constraints(&[
            this.width |LE(STRONG)| parent.width,
            this.height |LE(STRONG)| parent.height,
        ]).unwrap();

        // Preferred sizes
        match self.size.0 {
            Size::Absolute(x) =>
                solver.add_constraint(this.width |EQ(STRONG)| x as f64).unwrap(),
            Size::Max =>
                solver.add_constraint(this.width |EQ(WEAK)| 1_000_000.0).unwrap(),
        }
        match self.size.1 {
            Size::Absolute(y) =>
                solver.add_constraint(this.height |EQ(STRONG)| y as f64).unwrap(),
            Size::Max =>
                solver.add_constraint(this.height |EQ(WEAK)| 1_000_000.0).unwrap(),
        }
    }
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
    fn children(&self) -> Option<&Vec<PanelId>> {
        Some(&self.children)
    }

    fn add_constraints(
        &self, solver: &mut Solver, ui: &Ui,
        this: &LayoutVariables, parent: &LayoutVariables
    ) {
        // Restrict to parent sizes
        solver.add_constraints(&[
            this.width |LE(STRONG)| parent.width,
            this.height |LE(STRONG)| parent.height,
        ]).unwrap();

        match self.orientation {
            Orientation::Horizontal => {
                // On the axis we don't need to scale on, prefer parent size
                solver.add_constraint(this.height |EQ(STRONG)| parent.height).unwrap();

                // Prefer a size that contains all children
                let mut expression = Expression::from_constant(0.0);
                for child_id in &self.children {
                    let child = &ui.get(*child_id).unwrap().layout.variables;
                    expression = expression + child.width;
                }
                solver.add_constraint(this.width |EQ(STRONG)| expression).unwrap();
            }
            Orientation::Vertical => {
                // On the axis we don't need to scale on, prefer parent size
                solver.add_constraint(this.width |EQ(STRONG)| parent.width).unwrap();

                // Prefer a size that contains all children
                let mut expression = Expression::from_constant(0.0);
                for child_id in &self.children {
                    let child = &ui.get(*child_id).unwrap().layout.variables;
                    expression = expression + child.height;
                }
                solver.add_constraint(this.height |EQ(STRONG)| expression).unwrap();
            }
        }
    }
}
