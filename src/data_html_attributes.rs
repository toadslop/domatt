use strum::Display;

use crate::Attribute;

/// An enum defining the different canvas-element-specific attribute keys. Each variant takes either tuple
/// that represents the valid values for the attributes or nothing to represent a boolean
/// attribute.
#[derive(Debug, Display)]
#[strum(serialize_all = "lowercase")]
pub enum DataHtmlAttributes<'a> {
    Value(&'a str),
}

impl<'a> Attribute for DataHtmlAttributes<'a> {
    fn get_key(&self) -> String {
        self.to_string()
    }

    fn get_val(&self) -> Option<String> {
        match self {
            DataHtmlAttributes::Value(val) => Some(val.to_string()),
        }
    }
}
