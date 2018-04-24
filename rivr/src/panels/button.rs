use {
    nalgebra::{Point2},
    cassowary::{
        Solver,
        WeightedRelation::*,
        strength::{MEDIUM},
    },

    attributes::{PanelSize, PanelBox},
    input::{FrameCollision},
    panels::{Panel},
    rendering::{Renderer},
    Ui, PanelId, Error, Event, LayoutVariables, PanelLayout, Resources,
};

pub struct ButtonPanel {
    size: PanelSize,
    panel_box: PanelBox,
    children: Vec<PanelId>,

    hovering: bool,
    pressed: Event,
}

impl ButtonPanel {
    pub fn new(size: PanelSize, panel_box: PanelBox, content: Option<PanelId>) -> Self {
        ButtonPanel {
            size,
            panel_box,
            children: content.map(|c| vec!(c)).unwrap_or_default(),

            hovering: false,
            pressed: Event::new(),
        }
    }
}

impl ButtonPanel {
    pub fn event_pressed(&self) -> Event {
        self.pressed.clone()
    }
}

impl Panel for ButtonPanel {
    fn visible_children(&self) -> Option<&Vec<PanelId>> { Some(&self.children) }

    fn add_constraints(
        &self,
        solver: &mut Solver, ui: &Ui,
        this: &LayoutVariables,
        c_depth: f64,
    ) {
        self.size.add_constraints(solver, this, c_depth);

        // We need to be at least the size of the content unless size specifies otherwise
        if let Some(content_id) = self.children.get(0) {
            let content = &ui.get(*content_id).unwrap().layout.variables;
            solver.add_constraints(&[
                this.width |GE(MEDIUM)| content.width,
                this.height |GE(MEDIUM)| content.height,
            ]).unwrap();
        }
    }

    fn render(
        &self,
        ui: &Ui, _resources: &Resources, renderer: &mut Renderer,
        this_id: PanelId, this_layout: &PanelLayout,
        frame: &mut FrameCollision,
    ) -> Result<(), Error> {
        self.panel_box.render(renderer, this_id, this_layout, self.hovering)?;

        if let Some(content_id) = self.children.get(0) {
            let content = &ui.get(*content_id).unwrap().layout;

            // Center the content
            let offset_x = (this_layout.size.x - content.size.x) * 0.5;
            let offset_y = (this_layout.size.y - content.size.y) * 0.5;
            let position = Point2::new(offset_x, offset_y);

            renderer.render_panel(this_id, *content_id, position)?;
            frame.set(*content_id, position, content.size);
        }

        Ok(())
    }

    fn is_capturing_cursor(&self) -> bool { true }

    fn handle_hover_start(&mut self) -> bool { self.hovering = true; true }

    fn handle_hover_end(&mut self) -> bool { self.hovering = false; true }

    fn handle_pressed(&mut self) { self.pressed.raise() }
}
