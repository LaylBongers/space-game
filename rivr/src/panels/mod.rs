use {
    std::any::{Any},

    nalgebra::{Point2},
    palette::{Srgba},
    cassowary::{
        Solver, Expression,
        WeightedRelation::*,
        strength::{WEAK, STRONG, REQUIRED},
    },

    Size, Orientation, Ui, PanelId, Error,
    layouting::{LayoutVariables, PanelLayout},
    rendering::{Renderer},
};

pub trait Panel: Any {
    /// Return a vector of the children of this panel, if applicable.
    /// These children will be rendered before this one.
    fn children(&self) -> Option<&Vec<PanelId>>;

    fn add_constraints(
        &self,
        solver: &mut Solver, ui: &Ui,
        this: &LayoutVariables, parent: &LayoutVariables,
        depth: f64,
    );

    fn render(
        &self, renderer: &mut Renderer, ui: &Ui, this_id: PanelId, this_layout: &PanelLayout
    ) -> Result<(), Error>;
}

pub struct EmptyPanel {
    size: (Size, Size),
    background: Option<Srgba>,
}

impl EmptyPanel {
    pub fn new(size: (Size, Size), background: Option<Srgba>) -> Self {
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
        depth: f64,
    ) {
        solver.add_constraints(&[
            // Must be non-negative size
            this.width |GE(REQUIRED)| 0.0,
            this.height |GE(REQUIRED)| 0.0,
            // Restrict to parent sizes
            this.width |LE(STRONG)| parent.width,
            this.height |LE(STRONG)| parent.height,
        ]).unwrap();

        // Preferred sizes
        match self.size.0 {
            Size::Absolute(x) =>
                solver.add_constraint(this.width |EQ(STRONG)| x as f64).unwrap(),
            Size::Max =>
                solver.add_constraint(this.width |EQ(WEAK * depth)| 1_000_000.0).unwrap(),
        }
        match self.size.1 {
            Size::Absolute(y) =>
                solver.add_constraint(this.height |EQ(STRONG)| y as f64).unwrap(),
            Size::Max =>
                solver.add_constraint(this.height |EQ(WEAK * depth)| 1_000_000.0).unwrap(),
        }
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
        &self,
        solver: &mut Solver, ui: &Ui,
        this: &LayoutVariables, parent: &LayoutVariables,
        depth: f64,
    ) {
        solver.add_constraints(&[
            // Must be non-negative size
            this.width |GE(REQUIRED)| 0.0,
            this.height |GE(REQUIRED)| 0.0,
            // Restrict to parent sizes
            this.width |LE(STRONG)| parent.width,
            this.height |LE(STRONG)| parent.height,
            // Minimize size to wrap around children
            this.width |EQ(WEAK * depth)| 0.0,
            this.height |EQ(WEAK * depth)| 0.0,
        ]).unwrap();

        match self.orientation {
            Orientation::Horizontal => {
                // Prefer a size that contains all children
                let mut expression = Expression::from_constant(0.0);
                for child_id in &self.children {
                    let child = &ui.get(*child_id).unwrap().layout.variables;
                    expression = expression + child.width;
                }
                solver.add_constraint(this.width |EQ(STRONG)| expression).unwrap();
            }
            Orientation::Vertical => {
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

    fn render(
        &self, renderer: &mut Renderer, ui: &Ui, this_id: PanelId, _this_layout: &PanelLayout
    ) -> Result<(), Error> {
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
