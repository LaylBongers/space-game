use {
    rivr::{
        attributes::{PanelText, PanelSize, AxisSize, PanelBox, Orientation, Srgba},
        input::{FrameCollision},
        panels::{ButtonPanel, StackPanel},
        Ui, Event, PanelId,
    },

    spacegame_game::state::{BuildState, BuildChoice},
};

pub struct UiSystem {
    pub ui: Ui,
    pub frame: FrameCollision,

    top_bar: TopBar,
}

impl UiSystem {
    pub fn new() -> Self {
        let mut ui = Ui::new();

        let (top_bar, top_bar_id) = TopBar::new(&mut ui);

        let mut root = StackPanel::new(
            PanelSize::max(),
            PanelBox::default(),
            Orientation::Vertical, 0.0,
        );
        root.add_child(top_bar_id);
        ui.add_root(root).unwrap();

        UiSystem {
            ui,
            frame: FrameCollision::new(),

            top_bar,
        }
    }

    pub fn update(&self, build_state: &mut BuildState) {
        self.top_bar.update(build_state);
    }
}

struct TopBar {
    build_pressed: Event,
    destroy_pressed: Event,
    destroy_all_pressed: Event,
}

impl TopBar {
    pub fn new(ui: &mut Ui) -> (Self, PanelId) {
        let panel_bg = Srgba::new(1.0, 1.0, 1.0, 0.8);
        let hover_bg = Srgba::new(0.9, 0.9, 0.9, 0.8);
        let panel_box = PanelBox {
            background: Some(panel_bg),
            .. PanelBox::default()
        };
        let button_box = PanelBox {
            background: Some(panel_bg),
            background_hovering: Some(hover_bg),
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
