use {
    rivr::{
        Ui, PanelId,
        attributes::{PanelSize, Orientation, Srgba},
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

        let button = EmptyPanel::new(
            PanelSize::absolute(30.0, 30.0),
            Some(Srgba::new(1.0, 1.0, 1.0, 0.9)),
        );
        let button_id = ui.add_panel(button);

        let mut top_bar = StackPanel::new(PanelSize::min(), Orientation::Horizontal);
        top_bar.add_child(button_id);
        let top_bar_id = ui.add_panel(top_bar);

        let spacer = EmptyPanel::new(PanelSize::max(), None);
        let spacer_id = ui.add_panel(spacer);

        let bottom_bar = EmptyPanel::new(
            PanelSize::y_absolute(30.0),
            Some(Srgba::new(0.5, 0.5, 0.5, 1.0)),
        );
        let bottom_bar_id = ui.add_panel(bottom_bar);

        let mut root = StackPanel::new(PanelSize::fill(), Orientation::Vertical);
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
