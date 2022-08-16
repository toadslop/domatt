use crate::Attribute;

pub trait DataAttribute {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/data#attr-value>
#[derive(Debug)]
pub struct Value(String);

impl Attribute for Value {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_str())
    }

    fn get_key(&self) -> &str {
        "value"
    }
}

impl DataAttribute for Value {}
