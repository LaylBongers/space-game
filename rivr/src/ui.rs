use {
    metrohash::{MetroHashMap},

    layouting::{PanelLayout, LayoutVariables},
    panels::{Panel},
    Error,
};

pub struct Ui {
    pub(crate) entries: MetroHashMap<PanelId, PanelEntry>,
    next_id: u32,

    root_id: Option<PanelId>,
    pub(crate) target_variables: LayoutVariables,
}

impl Ui {
    pub fn new() -> Self {
        Ui {
            entries: Default::default(),
            next_id: 0,

            root_id: None,
            target_variables: LayoutVariables::new(),
        }
    }

    pub fn get(&self, panel_id: PanelId) -> Option<&PanelEntry> {
        self.entries.get(&panel_id)
    }

    pub fn get_mut(&mut self, panel_id: PanelId) -> Option<&mut PanelEntry> {
        self.entries.get_mut(&panel_id)
    }

    pub fn root_id(&self) -> Result<PanelId, Error> {
        if let Some(root_id) = self.root_id {
            Ok(root_id)
        } else {
            Err(Error::NoRoot)
        }
    }

    pub fn add_panel<P: Panel>(&mut self, panel: P) -> PanelId {
        let id = self.next_id;
        self.next_id += 1;

        self.entries.insert(PanelId { id }, PanelEntry {
            panel: Box::new(panel),
            layout: PanelLayout::new(),
            needs_rendering: true,
        });

        PanelId { id }
    }

    pub fn add_root<P: Panel>(&mut self, panel: P) -> Result<PanelId, Error> {
        if self.root_id.is_some() {
            return Err(Error::RootAlreadyExists)
        }

        let panel_id = self.add_panel(panel);
        self.root_id = Some(panel_id);

        Ok(panel_id)
    }
}

pub struct PanelEntry {
    pub panel: Box<Panel>,
    pub layout: PanelLayout,
    pub needs_rendering: bool,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct PanelId {
    id: u32,
}
