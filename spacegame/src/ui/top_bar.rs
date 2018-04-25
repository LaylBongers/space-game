use {
    rivr::{
        attributes::{PanelSize, AxisSize, PanelBox, Orientation, Srgba},
        panels::{ButtonPanel, StackPanel, LabelPanel},
        Ui, Event, PanelId, FontId,
    },

    spacegame_game::{
        state::{BuildState, BuildChoice},
        ObjectClasses, ObjectClassId,
    },
};

pub struct TopBar {
    build_floor_pressed: Event,
    destroy_pressed: Event,
    destroy_all_pressed: Event,

    build_buttons: Vec<(Event, ObjectClassId)>,
}

impl TopBar {
    pub fn new(ui: &mut Ui, font: FontId, object_classes: &ObjectClasses) -> (Self, PanelId) {
        let panel_box = PanelBox {
            background: Some(Srgba::new(1.0, 1.0, 1.0, 0.8)),
            .. PanelBox::default()
        };

        let (build_floor_button_id, build_floor_pressed) =
            labeled_button(ui, "Build Floor", font);
        let (destroy_button_id, destroy_pressed) =
            labeled_button(ui, "Destroy", font);
        let (destroy_all_button_id, destroy_all_pressed) =
            labeled_button(ui, "Destroy All", font);

        let mut top_bar = StackPanel::new(
            PanelSize::new(AxisSize::Max, AxisSize::Min),
            panel_box.clone(),
            Orientation::Horizontal, 3.0,
        );
        top_bar.add_child(build_floor_button_id);

        // Add all the buttons for different objects
        let mut build_buttons = Vec::new();
        for (id, class) in object_classes.classes().iter().enumerate() {
            let (build_button_id, build_pressed) =
                labeled_button(ui, &format!("Build {}", class.friendly_name), font);
            top_bar.add_child(build_button_id);
            build_buttons.push((build_pressed, ObjectClassId { id }));
        }

        top_bar.add_child(destroy_button_id);
        top_bar.add_child(destroy_all_button_id);
        let top_bar_id = ui.add_panel(top_bar);

        (TopBar {
            build_floor_pressed,
            destroy_pressed,
            destroy_all_pressed,

            build_buttons,
        }, top_bar_id)
    }

    pub fn update(&self, build_state: &mut BuildState) {
        if self.build_floor_pressed.check() {
            build_state.choice = BuildChoice::Floor;
        }
        if self.destroy_pressed.check() {
            build_state.choice = BuildChoice::Destroy;
        }
        if self.destroy_all_pressed.check() {
            build_state.choice = BuildChoice::DestroyAll;
        }

        for (event, id) in &self.build_buttons {
            if event.check() {
                build_state.choice = BuildChoice::Object(*id);
            }
        }
    }
}

fn labeled_button(
    ui: &mut Ui, text: &str, font: FontId
) -> (PanelId, Event) {
    let label = LabelPanel::new(ui, text, font, 12.0).unwrap();
    let label_id = ui.add_panel(label);

    let button = ButtonPanel::new(
        PanelSize::absolute(84.0, 24.0),
        PanelBox {
            background: Some(Srgba::new(1.0, 1.0, 1.0, 1.0)),
            background_hovering: Some(Srgba::new(0.95, 0.95, 0.95, 1.0)),
            border_radius: 3.0,
            .. PanelBox::default()
        },
        Some(label_id), 3.0,
    );
    let pressed = button.event_pressed();
    let button_id = ui.add_panel(button);

    (button_id, pressed)
}
