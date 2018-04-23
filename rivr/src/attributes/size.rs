use {
    cassowary::{
        Solver, Variable,
        WeightedRelation::*,
        strength::{WEAK, MEDIUM, STRONG, REQUIRED},
    },

    layouting::{LayoutVariables},
};

pub struct PanelSize {
    pub x: AxisSize,
    pub y: AxisSize,
}

impl PanelSize {
    pub fn new(x: AxisSize, y: AxisSize) -> Self {
        PanelSize {
            x,
            y,
        }
    }

    pub fn absolute(x: f32, y: f32) -> Self {
        PanelSize {
            x: AxisSize::Absolute(x),
            y: AxisSize::Absolute(y),
        }
    }

    pub fn fill() -> Self {
        PanelSize {
            x: AxisSize::Fill,
            y: AxisSize::Fill,
        }
    }

    pub fn max() -> Self {
        PanelSize {
            x: AxisSize::Max,
            y: AxisSize::Max,
        }
    }

    pub fn min() -> Self {
        PanelSize {
            x: AxisSize::Min,
            y: AxisSize::Min,
        }
    }

    pub fn add_constraints(
        &self, solver: &mut Solver,
        this: &LayoutVariables,
        parent: &LayoutVariables,
        c_depth: f64
    ) {
        self.x.add_constraints(solver, this.width, parent.width, c_depth);
        self.y.add_constraints(solver, this.height, parent.height, c_depth);
    }
}

#[derive(Copy, Clone)]
pub enum AxisSize {
    /// Tries to set an absolute panel size.
    Absolute(f32),
    /// Tries to fill the panel's parent.
    Fill,
    /// Tries to make the panel as big as possible.
    Max,
    /// Tries to make the panel as small as possible.
    Min,
}

impl AxisSize {
    pub fn add_constraints(
        self, solver: &mut Solver, axis: Variable, parent_axis: Variable, c_depth: f64
    ) {
        let constraint = match self {
            AxisSize::Absolute(value) =>
                axis |EQ(STRONG)| value as f64,
            AxisSize::Fill =>
                axis |EQ(STRONG)| parent_axis,
            AxisSize::Max =>
                axis |EQ(WEAK * c_depth)| 1_000_000.0,
            AxisSize::Min =>
                axis |EQ(WEAK * c_depth)| 0.0,
        };

        solver.add_constraints(&[
            // Must be non-negative size
            axis |GE(REQUIRED)| 0.0,
            // Prefer parent to be at least our size unless specified otherwise, this allows Min on
            // parents to wrap around children
            parent_axis |GE(MEDIUM)| axis,
            // The size constraint
            constraint,
        ]).unwrap();
    }
}
