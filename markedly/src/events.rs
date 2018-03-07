use std::collections::{VecDeque};
use std::rc::{Rc};
use std::cell::{RefCell};

use scripting::{Model};

/// Data for interacting with an active UI component tree inserted through a template.
#[derive(Clone)]
pub struct ComponentEvents {
    event_sink: Rc<RefCell<VecDeque<String>>>,
    pub(crate) model: Rc<RefCell<(Model, bool)>>,
}

impl ComponentEvents {
    pub(crate) fn new(model: Model) -> Self {
        ComponentEvents {
            event_sink: Default::default(),
            model: Rc::new(RefCell::new((model, false))),
        }
    }

    /// Retrieves the next event raised by a component, or returns None.
    pub fn next(&self) -> Option<String> {
        self.event_sink.borrow_mut().pop_front()
    }

    /// Raises an event.
    pub fn raise(&self, event: String) {
        self.event_sink.borrow_mut().push_back(event);
    }

    /// Retrieves the model, allowing the caller to change it, then marks it changed.
    pub fn change_model<F: FnOnce(&mut Model)>(&self, f: F) {
        let mut model = self.model.borrow_mut();
        f(&mut model.0);

        // Mark this model as changed
        model.1 = true;
    }

    pub(crate) fn model_changed(&self) -> bool {
        self.model.borrow().1
    }

    pub(crate) fn clear_changed(&self) {
        self.model.borrow_mut().1 = false;
    }
}
