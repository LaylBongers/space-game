use {
    rivr::{
        Ui, PanelId,
        attributes::{PanelText, PanelSize, AxisSize, PanelBox, Orientation, Srgba},
        panels::{ButtonPanel, EmptyPanel, StackPanel},
    },
};

pub struct UiSystem {
    pub ui: Ui,
    pub root_id: PanelId,
}

impl UiSystem {
    pub fn new() -> Self {
        let mut ui = Ui::new();

        let panel_bg = Srgba::new(1.0, 1.0, 1.0, 0.8);
        let panel_box = PanelBox {
            background: Some(panel_bg),
            .. PanelBox::default()
        };
        let button_box = PanelBox {
            background: Some(panel_bg),
            border_radius: 3.0,
            .. PanelBox::default()
        };

        let top_bar_id = {
            let build_button = ButtonPanel::new(
                PanelSize::absolute(72.0, 24.0),
                button_box.clone(),
                Some(PanelText::new("Build", 9)),
            );
            let build_button_id = ui.add_panel(build_button);

            let destroy_button = ButtonPanel::new(
                PanelSize::absolute(72.0, 24.0),
                button_box.clone(),
                Some(PanelText::new("Destroy", 9)),
            );
            let destroy_button_id = ui.add_panel(destroy_button);

            let destroy_all_button = ButtonPanel::new(
                PanelSize::absolute(72.0, 24.0),
                button_box.clone(),
                Some(PanelText::new("Destroy All", 8)),
            );
            let destroy_all_button_id = ui.add_panel(destroy_all_button);

            let button2 = EmptyPanel::new(
                PanelSize::absolute(72.0, 24.0),
                button_box.clone(),
            );
            let button2_id = ui.add_panel(button2);

            let mut top_bar = StackPanel::new(
                PanelSize::new(AxisSize::Fill, AxisSize::Min),
                panel_box.clone(),
                Orientation::Horizontal, 3.0,
            );
            top_bar.add_child(build_button_id);
            top_bar.add_child(destroy_button_id);
            top_bar.add_child(destroy_all_button_id);
            top_bar.add_child(button2_id);
            ui.add_panel(top_bar)
        };

        let mut root = StackPanel::new(
            PanelSize::fill(),
            PanelBox::default(),
            Orientation::Vertical, 0.0,
        );
        root.add_child(top_bar_id);
        let root_id = ui.add_panel(root);

        UiSystem {
            ui,
            root_id,
        }
    }
}
