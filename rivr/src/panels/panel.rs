use {
    std::any::{Any},

    cassowary::{Solver},

    input::{FrameCollision},
    rendering::{Renderer},
    Ui, PanelId, Error, LayoutVariables, PanelLayout,
};

pub trait Panel: Any {
    /// Returns a vector of the children that need to be layouted, and rendered.
    fn visible_children(&self) -> Option<&Vec<PanelId>> { None }

    fn add_constraints(
        &self,
        solver: &mut Solver, ui: &Ui,
        this: &LayoutVariables,
        c_depth: f64,
    );

    fn render(
        &self,
        ui: &Ui, renderer: &mut Renderer,
        this_id: PanelId, this_layout: &PanelLayout,
        frame: &mut FrameCollision,
    ) -> Result<(), Error>;

    fn is_capturing_cursor(&self) -> bool { false }

    /// If returns true, component will be re-rendered.
    fn handle_hover_start(&mut self) -> bool { false }

    /// If returns true, component will be re-rendered.
    fn handle_hover_end(&mut self) -> bool { false }

    fn handle_pressed(&mut self) {}
}
