use {
    metrohash::{MetroHashMap},

    layouting::{PanelLayout},
    panels::{Panel},
};

pub struct Ui {
    entries: MetroHashMap<PanelId, PanelEntry>,
    next_id: u32,
}

impl Ui {
    pub fn new() -> Self {
        Ui {
            entries: Default::default(),
            next_id: 0,
        }
    }

    pub fn entries_mut(&mut self) -> ::std::collections::hash_map::ValuesMut<PanelId, PanelEntry> {
        self.entries.values_mut()
    }

    pub fn get(&self, panel_id: PanelId) -> Option<&PanelEntry> {
        self.entries.get(&panel_id)
    }

    pub fn get_mut(&mut self, panel_id: PanelId) -> Option<&mut PanelEntry> {
        self.entries.get_mut(&panel_id)
    }

    pub fn add_panel<P: Panel>(&mut self, panel: P) -> PanelId {
        let id = self.next_id;
        self.next_id += 1;

        self.entries.insert(PanelId { id }, PanelEntry {
            panel: Box::new(panel),
            layout: PanelLayout::new(),
        });

        PanelId { id }
    }
}

pub struct PanelEntry {
    pub panel: Box<Panel>,
    pub layout: PanelLayout,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct PanelId {
    id: u32,
}
