use {
    rivr::{
        Ui, PanelId,
        attributes::{PanelSize, AxisSize, PanelBox, Orientation, Srgba},
        panels::{EmptyPanel, StackPanel},
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
            let button1 = EmptyPanel::new(
                PanelSize::absolute(72.0, 24.0),
                button_box.clone(),
            );
            let button1_id = ui.add_panel(button1);

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
            top_bar.add_child(button1_id);
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
