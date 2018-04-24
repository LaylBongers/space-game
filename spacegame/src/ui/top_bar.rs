use {
    rivr::{
        attributes::{PanelText, PanelSize, AxisSize, PanelBox, Orientation, Srgba},
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
        let button_box = PanelBox {
            background: Some(Srgba::new(1.0, 1.0, 1.0, 1.0)),
            background_hovering: Some(Srgba::new(0.95, 0.95, 0.95, 1.0)),
            border_radius: 3.0,
            .. PanelBox::default()
        };

        let build_button = ButtonPanel::new(
            PanelSize::absolute(72.0, 24.0),
            button_box.clone(),
            Some(PanelText::new("Build", 9)),
        );
        let build_pressed = build_button.event_pressed();
        let build_button_id = ui.add_panel(build_button);

        let destroy_button = ButtonPanel::new(
            PanelSize::absolute(72.0, 24.0),
            button_box.clone(),
            Some(PanelText::new("Destroy", 9)),
        );
        let destroy_pressed = destroy_button.event_pressed();
        let destroy_button_id = ui.add_panel(destroy_button);

        let destroy_all_button = ButtonPanel::new(
            PanelSize::absolute(72.0, 24.0),
            button_box.clone(),
            Some(PanelText::new("Destroy All", 8)),
        );
        let destroy_all_pressed = destroy_all_button.event_pressed();
        let destroy_all_button_id = ui.add_panel(destroy_all_button);

        let test_label = LabelPanel::new(resources, "Test", font, 9.0).unwrap();
        let test_label_id = ui.add_panel(test_label);

        let mut top_bar = StackPanel::new(
            PanelSize::new(AxisSize::Max, AxisSize::Min),
            panel_box.clone(),
            Orientation::Horizontal, 3.0,
        );
        top_bar.add_child(build_button_id);
        top_bar.add_child(destroy_button_id);
        top_bar.add_child(test_label_id);
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
