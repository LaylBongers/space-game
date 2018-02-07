use std::collections::{VecDeque};
use std::rc::{Rc};
use std::cell::{RefCell};

pub struct ComponentEvents {
    event_sink: Rc<RefCell<VecDeque<String>>>,
}

impl ComponentEvents {
    pub(crate) fn new() -> Self {
        ComponentEvents {
            event_sink: Default::default(),
        }
    }

    pub fn next(&self) -> Option<String> {
        self.event_sink.borrow_mut().pop_front()
    }

    pub(crate) fn create_client(&self) -> ComponentEventsClient {
        ComponentEventsClient {
            event_sink: self.event_sink.clone(),
        }
    }
}

#[derive(Clone)]
pub struct ComponentEventsClient {
    event_sink: Rc<RefCell<VecDeque<String>>>,
}

impl ComponentEventsClient {
    pub fn raise(&self, event: String) {
        self.event_sink.borrow_mut().push_back(event);
    }
}
