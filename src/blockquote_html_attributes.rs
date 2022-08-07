use strum::Display;
use url::Url;

use crate::Attribute;

/// An enum defining the different base-element-specific attribute keys. Each variant takes either tuple
/// that represents the valid values for the attributes or nothing to represent a boolean
/// attribute.
#[derive(Debug, Display)]
#[strum(serialize_all = "lowercase")]
pub enum BlockquoteHtmlAttributes<'a> {
    Cite(&'a Url),
}

impl<'a> Attribute for BlockquoteHtmlAttributes<'a> {
    fn get_key(&self) -> String {
        self.to_string()
    }

    fn get_val(&self) -> Option<String> {
        match &self {
            BlockquoteHtmlAttributes::Cite(val) => Some(val.to_string()),
        }
    }
}
