use rivr::{Ui, PanelId, Size, Orientation};
use rivr::panels::{EmptyPanel, StackPanel};

pub struct UiSystem {
    pub ui: Ui,
    pub root_id: PanelId,
}

impl UiSystem {
    pub fn new() -> Self {
        let mut ui = Ui::new();

        let top_bar = EmptyPanel::new((Size::Max, Size::Absolute(24.0)), true);
        let top_bar_id = ui.add_panel(top_bar);

        let spacer = EmptyPanel::new((Size::Max, Size::Max), false);
        let spacer_id = ui.add_panel(spacer);

        let bottom_bar = EmptyPanel::new((Size::Max, Size::Absolute(24.0)), true);
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
