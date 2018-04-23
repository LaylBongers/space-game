use {
    std::any::{Any},

    cassowary::{Solver},

    Ui, PanelId, Error,
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
        c_depth: f64,
    );

    fn render(
        &self, renderer: &mut Renderer, ui: &Ui, this_id: PanelId, this_layout: &PanelLayout
    ) -> Result<(), Error>;
}
