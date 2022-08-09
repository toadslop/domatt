use strum::{AsRefStr, Display};

use crate::Attribute;

/// An enum defining the different canvas-element-specific attribute keys. Each variant takes either tuple
/// that represents the valid values for the attributes or nothing to represent a boolean
/// attribute.
#[derive(Debug, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum DetailsHtmlAttributes<'a> {
    Open(&'a str),
}
// Note: this element also has a special toggle event.
// be sure to handle that in the yew-dom-attributes crate

impl<'a> Attribute<'a> for DetailsHtmlAttributes<'a> {
    fn get_key(&self) -> &str {
        self.as_ref()
    }

    fn get_val(&self) -> Option<&str> {
        match self {
            DetailsHtmlAttributes::Open(val) => Some(val),
        }
    }
}
