use strum::Display;

use crate::Attribute;

/// An enum defining the different canvas-element-specific attribute keys. Each variant takes either tuple
/// that represents the valid values for the attributes or nothing to represent a boolean
/// attribute.
#[derive(Debug, Display)]
#[strum(serialize_all = "lowercase")]
pub enum ColgroupHtmlAttributes {
    Span(u16),
}

impl Attribute for ColgroupHtmlAttributes {
    fn get_key(&self) -> String {
        self.to_string()
    }

    fn get_val(&self) -> Option<String> {
        match &self {
            ColgroupHtmlAttributes::Span(val) => Some(val.to_string()),
        }
    }
}
