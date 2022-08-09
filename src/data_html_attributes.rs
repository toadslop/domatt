use strum::{AsRefStr, Display};

use crate::Attribute;

/// An enum defining the different canvas-element-specific attribute keys. Each variant takes either tuple
/// that represents the valid values for the attributes or nothing to represent a boolean
/// attribute.
#[derive(Debug, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum DataHtmlAttributes<'a> {
    Value(&'a str),
}

impl<'a> Attribute<'a> for DataHtmlAttributes<'a> {
    fn get_key(&self) -> &str {
        self.as_ref()
    }

    fn get_val(&self) -> Option<&str> {
        match self {
            DataHtmlAttributes::Value(val) => Some(val),
        }
    }
}
