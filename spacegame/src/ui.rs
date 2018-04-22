use {
    rivr::{
        Ui, PanelId, Size, Orientation, Srgba,
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
            (Size::Absolute(30.0), Size::Absolute(30.0)),
            Some(Srgba::new(1.0, 1.0, 1.0, 0.9)),
        );
        let button_id = ui.add_panel(button);

        let mut top_bar = StackPanel::new(Orientation::Horizontal);
        top_bar.add_child(button_id);
        let top_bar_id = ui.add_panel(top_bar);

        let spacer = EmptyPanel::new((Size::Max, Size::Max), None);
        let spacer_id = ui.add_panel(spacer);

        let bottom_bar = EmptyPanel::new(
            (Size::Max, Size::Absolute(30.0)),
            Some(Srgba::new(0.5, 0.5, 0.5, 1.0)),
        );
        let bottom_bar_id = ui.add_panel(bottom_bar);

        let mut root = StackPanel::new(Orientation::Vertical);
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
