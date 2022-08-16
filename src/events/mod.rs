use std::fmt::Debug;

pub trait Event {
    /// Returns a string representing the key of a DOM attribute.
    fn get_key(&self) -> &str;

    /// Returns an `Option<String>` representing the value of a DOM
    /// attribute. `None` indicates a boolean attribute, such as `disabled`,
    /// which has no value.
    fn get_callback(&self) -> Box<dyn Fn() -> ()>;
}

struct Abort {}

impl Event for Abort {
    fn get_key(&self) -> &str {
        "abort"
    }

    fn get_callback(&self) -> Box<dyn Fn() -> ()> {
        Box::new(|| ())
    }
}

pub struct EventHandlerTest {
    attributes: Vec<(ProcessAction, String, Option<String>)>,
    listeners: Vec<(ProcessAction, String, Box<dyn Fn() -> ()>)>,
}

impl EventHandlerTest {
    pub fn add_event(&mut self, event: Box<dyn Event>) {
        let action = ProcessAction::Add;
        let key = String::from(event.get_key());
        let cb = event.get_callback();
        self.listeners.push((action, key, cb))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProcessAction {
    Add,
    Remove,
}
