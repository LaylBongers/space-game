use {
    nalgebra::{Vector2},
    cassowary::{
        Solver, Variable,
        strength::{STRONG},
    },

    Ui, PanelId
};

pub fn layout(ui: &mut Ui, root_id: PanelId, target_size: Vector2<f32>) {
    let mut solver = Solver::new();

    // Recursively add the constraints for all our panels
    add_panel_constraints(&mut solver, ui, root_id, &ui.target_variables, 1.0);

    // Constrain the total UI to the window
    solver.add_edit_variable(ui.target_variables.width, STRONG).unwrap();
    solver.suggest_value(ui.target_variables.width, target_size.x as f64).unwrap();
    solver.add_edit_variable(ui.target_variables.height, STRONG).unwrap();
    solver.suggest_value(ui.target_variables.height, target_size.y as f64).unwrap();

    // Finally, retrieve the solved data
    for entry in ui.entries_mut() {
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
    panel_id: PanelId, parent_variables: &LayoutVariables,
    c_depth: f64,
) {
    let panel_entry = ui.get(panel_id).unwrap();
    let panel_variables = &panel_entry.layout.variables;

    panel_entry.panel.add_constraints(
        solver, ui,
        panel_variables, parent_variables,
        c_depth,
    );

    if let Some(children) = panel_entry.panel.children() {
        for child_id in children {
            add_panel_constraints(solver, ui, *child_id, panel_variables, c_depth + 0.01);
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
