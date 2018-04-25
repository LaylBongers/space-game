mod top_bar;

use {
    std::io::{Read},

    ggez::{Context, GameResult},
    rivr::{
        attributes::{PanelSize, PanelBox, Orientation},
        input::{FrameCollision},
        panels::{StackPanel},
        Ui,
    },

    spacegame_game::{
        state::{BuildState},
        ObjectClasses,
    },
    ui::top_bar::{TopBar},
};

pub struct UiSystem {
    pub ui: Ui,
    pub frame: FrameCollision,

    top_bar: TopBar,
}

impl UiSystem {
    pub fn new(ctx: &mut Context, object_classes: &ObjectClasses) -> GameResult<Self> {
        let mut ui = Ui::new();

        // Add resources for use in the UI globally
        let mut file = ctx.filesystem.open("/DejaVuSansMono.ttf")?;
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).unwrap();
        let font = ui.resources.add_font(bytes);

        // Set up the UI itself
        let (top_bar, top_bar_id) = TopBar::new(&mut ui, font, object_classes);

        let mut root = StackPanel::new(
            PanelSize::max(),
            PanelBox::default(),
            Orientation::Vertical, 0.0,
        );
        root.add_child(top_bar_id);
        ui.add_root(root).unwrap();

        Ok(UiSystem {
            ui,
            frame: FrameCollision::new(),

            top_bar,
        })
    }

    pub fn update(&self, build_state: &mut BuildState) {
        self.top_bar.update(build_state);
    }
}
