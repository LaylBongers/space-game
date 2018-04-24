use {
    rivr::{
        attributes::{PanelSize, AxisSize, PanelBox, Orientation, Srgba},
        panels::{ButtonPanel, StackPanel, LabelPanel},
        Ui, Event, PanelId, Resources, FontId,
    },

    spacegame_game::state::{BuildState, BuildChoice},
};

pub struct TopBar {
    build_pressed: Event,
    destroy_pressed: Event,
    destroy_all_pressed: Event,
}

impl TopBar {
    pub fn new(ui: &mut Ui, resources: &Resources, font: FontId) -> (Self, PanelId) {
        let panel_box = PanelBox {
            background: Some(Srgba::new(1.0, 1.0, 1.0, 0.8)),
            .. PanelBox::default()
        };

        let (build_button_id, build_pressed) =
            labeled_button(ui, resources, "Build", font);
        let (destroy_button_id, destroy_pressed) =
            labeled_button(ui, resources, "Destroy", font);
        let (destroy_all_button_id, destroy_all_pressed) =
            labeled_button(ui, resources, "Destroy All", font);

        let mut top_bar = StackPanel::new(
            PanelSize::new(AxisSize::Max, AxisSize::Min),
            panel_box.clone(),
            Orientation::Horizontal, 3.0,
        );
        top_bar.add_child(build_button_id);
        top_bar.add_child(destroy_button_id);
        top_bar.add_child(destroy_all_button_id);
        let top_bar_id = ui.add_panel(top_bar);

        (TopBar {
            build_pressed,
            destroy_pressed,
            destroy_all_pressed,
        }, top_bar_id)
    }

    pub fn update(&self, build_state: &mut BuildState) {
        if self.build_pressed.check() {
            build_state.choice = BuildChoice::Floor;
        }
        if self.destroy_pressed.check() {
            build_state.choice = BuildChoice::Destroy;
        }
        if self.destroy_all_pressed.check() {
            build_state.choice = BuildChoice::DestroyAll;
        }
    }
}

fn labeled_button(
    ui: &mut Ui, resources: &Resources, text: &str, font: FontId
) -> (PanelId, Event) {
    let label = LabelPanel::new(resources, text, font, 9.0).unwrap();
    let label_id = ui.add_panel(label);

    let button = ButtonPanel::new(
        PanelSize::absolute(72.0, 24.0),
        PanelBox {
            background: Some(Srgba::new(1.0, 1.0, 1.0, 1.0)),
            background_hovering: Some(Srgba::new(0.95, 0.95, 0.95, 1.0)),
            border_radius: 3.0,
            .. PanelBox::default()
        },
        Some(label_id),
    );
    let pressed = button.event_pressed();
    let button_id = ui.add_panel(button);

    (button_id, pressed)
}
