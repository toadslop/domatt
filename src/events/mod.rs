use std::{fmt::Debug, rc::Rc};

pub trait Event {
    /// Returns a string representing the key of a DOM attribute.
    fn get_key(&self) -> &str;

    /// Returns an `Option<String>` representing the value of a DOM
    /// attribute. `None` indicates a boolean attribute, such as `disabled`,
    /// which has no value.
    fn get_callback(&self) -> Rc<Box<dyn Fn() -> ()>>;
}

struct Abort {
    id: String,
    func: Rc<Box<dyn Fn() -> ()>>,
}

impl Abort {
    const KEY: &'static str = "abort";
}

impl Event for Abort {
    fn get_key(&self) -> &str {
        Self::KEY
    }

    fn get_callback(&self) -> Rc<Box<dyn Fn() -> ()>> {
        self.func.clone()
    }
}

pub struct ButtonProps {
    attributes: Vec<(ProcessAction, String, Option<String>)>,
    listeners: Vec<(ProcessAction, String, Rc<Box<dyn Fn() -> ()>>)>,
}

impl ButtonProps {
    pub fn new() -> Self {
        Self {
            attributes: Vec::new(),
            listeners: Vec::new(),
        }
    }
    pub fn add_event(&mut self, event: Box<dyn Event>) {
        let action = ProcessAction::Add;
        let key = String::from(event.get_key());
        let cb = event.get_callback();
        self.listeners.push((action, key, cb.clone()))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProcessAction {
    Add,
    Remove,
}
