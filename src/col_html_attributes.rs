use strum::{AsRefStr, Display};

use crate::Attribute;

/// An enum defining the different canvas-element-specific attribute keys. Each variant takes either tuple
/// that represents the valid values for the attributes or nothing to represent a boolean
/// attribute.
#[derive(Debug, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum ColHtmlAttributes {
    Span(u16),
}

impl<'a> Attribute<'a> for ColHtmlAttributes {
    fn get_key(&self) -> &str {
        self.as_ref()
    }

    fn get_val(&self) -> Option<&str> {
        match &self {
            ColHtmlAttributes::Span(val) => Some(val.to_string().as_ref()),
        }
    }
}
