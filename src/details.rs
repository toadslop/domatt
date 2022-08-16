use crate::Attribute;

pub trait DetailsAttribute {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/details#attr-open>
#[derive(Debug)]
pub struct Open;

impl Attribute for Open {
    fn get_val(&self) -> Option<&str> {
        None
    }

    fn get_key(&self) -> &str {
        "open"
    }
}

impl DetailsAttribute for Open {}
