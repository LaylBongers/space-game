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

        let top_bar_id = {
            let button = EmptyPanel::new(
                PanelSize::absolute(30.0, 30.0),
                PanelBox {
                    background: Some(panel_bg),
                    border_radius: 3.0,
                    .. PanelBox::default()
                },
            );
            let button_id = ui.add_panel(button);

            let mut top_bar = StackPanel::new(
                PanelSize::new(AxisSize::Fill, AxisSize::Min),
                PanelBox {
                    background: Some(panel_bg),
                    .. PanelBox::default()
                },
                Orientation::Horizontal
            );
            top_bar.add_child(button_id);
            ui.add_panel(top_bar)
        };

        let spacer = EmptyPanel::new(PanelSize::max(), PanelBox::default());
        let spacer_id = ui.add_panel(spacer);

        let bottom_bar = EmptyPanel::new(
            PanelSize::y_absolute(30.0),
            PanelBox {
                background: Some(Srgba::new(0.5, 0.5, 0.5, 1.0)),
                .. PanelBox::default()
            },
        );
        let bottom_bar_id = ui.add_panel(bottom_bar);

        let mut root = StackPanel::new(
            PanelSize::fill(),
            PanelBox::default(),
            Orientation::Vertical
        );
        root.add_child(top_bar_id);
        root.add_child(spacer_id);
        root.add_child(bottom_bar_id);
        let root_id = ui.add_panel(root);

        UiSystem {
            ui,
            root_id,
        }
    }
}
