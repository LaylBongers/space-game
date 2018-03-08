use std::collections::{VecDeque};
use std::rc::{Rc};
use std::cell::{RefCell, Ref, RefMut};

use scripting::{ScriptTable};

/// Data for interacting with an active UI component tree inserted through a template.
#[derive(Clone)]
pub struct ComponentEvents {
    event_sink: Rc<RefCell<VecDeque<String>>>,
    model: Rc<RefCell<ScriptTable>>,
    model_changed: Rc<RefCell<bool>>,
}

impl ComponentEvents {
    pub(crate) fn new(model: ScriptTable) -> Self {
        ComponentEvents {
            event_sink: Default::default(),
            model: Rc::new(RefCell::new(model)),
            model_changed: Rc::new(RefCell::new(false)),
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

    pub fn model(&self) -> Ref<ScriptTable> {
        self.model.borrow()
    }

    /// Retrieves the model, allowing the caller to change it, then marks it changed.
    pub fn change_model(&self) -> RefMut<ScriptTable> {
        // Mark this model as changed
        *self.model_changed.borrow_mut() = true;

        self.model.borrow_mut()
    }

    pub(crate) fn model_changed(&self) -> bool {
        *self.model_changed.borrow()
    }

    pub(crate) fn clear_changed(&self) {
        *self.model_changed.borrow_mut() = false;
    }
}
