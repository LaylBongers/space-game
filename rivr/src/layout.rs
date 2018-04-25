use {
    nalgebra::{Vector2},
    cassowary::{
        Solver, Variable,
        WeightedRelation::*,
        strength::{STRONG, REQUIRED},
    },

    Ui, PanelId
};

pub fn layout(ui: &mut Ui, root_id: PanelId, target_size: Vector2<f32>) {
    let mut solver = Solver::new();

    // Recursively add the constraints for all our panels
    add_panel_constraints(&mut solver, ui, root_id, 1);

    // Constrain the root panel to the window
    {
        let target = ui.target_variables();

        let root = &ui.get(root_id).unwrap().layout.variables;
        solver.add_constraints(&[
            root.width |LE(REQUIRED)| target.width,
            root.height |LE(REQUIRED)| target.height,
        ]).unwrap();

        solver.add_edit_variable(target.width, STRONG).unwrap();
        solver.suggest_value(target.width, target_size.x as f64).unwrap();
        solver.add_edit_variable(target.height, STRONG).unwrap();
        solver.suggest_value(target.height, target_size.y as f64).unwrap();
    }

    // Finally, retrieve the solved data
    for (_panel_id, entry) in &mut ui.entries {
        // TODO: The preferred way to get values is fetch_changes, check if there's a special
        // reason for this or if get_value is fine
        entry.layout.size = Vector2::new(
            solver.get_value(entry.layout.variables.width) as f32,
            solver.get_value(entry.layout.variables.height) as f32,
        );
    }
}

pub fn add_panel_constraints(
    solver: &mut Solver, ui: &Ui,
    panel_id: PanelId,
    depth: u32,
) {
    let panel_entry = ui.get(panel_id).unwrap();
    let panel_variables = &panel_entry.layout.variables;

    panel_entry.panel.add_constraints(
        solver, ui,
        panel_variables,
        // This value is used to put priority on WEAK constraints, currently it's set to prefer
        // parents over children, but this may change depending on what turns out to be more
        // intuitive
        1.0 / depth as f64,
    );

    if let Some(children) = panel_entry.panel.visible_children() {
        for child_id in children {
            add_panel_constraints(solver, ui, *child_id, depth + 1);
        }
    }
}

pub struct LayoutVariables {
    pub width: Variable,
    pub height: Variable,
}

impl LayoutVariables {
    pub fn new() -> Self {
        LayoutVariables {
            width: Variable::new(),
            height: Variable::new(),
        }
    }
}

pub struct PanelLayout {
    pub size: Vector2<f32>,
    pub variables: LayoutVariables,
}

impl PanelLayout {
    pub fn new() -> Self {
        PanelLayout {
            size: Vector2::new(0.0, 0.0),
            variables: LayoutVariables::new(),
        }
    }
}
