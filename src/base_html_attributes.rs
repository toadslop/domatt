use strum::Display;
use url::Url;

use crate::{Attribute, HtmlAttributeAnchorTarget};

/// An enum defining the different base-element-specific attribute keys. Each variant takes either tuple
/// that represents the valid values for the attributes or nothing to represent a boolean
/// attribute.
#[derive(Debug, Display)]
#[strum(serialize_all = "lowercase")]
pub enum BaseHtmlAttributes<'a> {
    Href(&'a Url),
    Target(HtmlAttributeAnchorTarget),
}

impl<'a> Attribute for BaseHtmlAttributes<'a> {
    fn get_key(&self) -> String {
        self.to_string()
    }

    fn get_val(&self) -> Option<String> {
        match &self {
            BaseHtmlAttributes::Href(val) => Some(val.to_string()),
            BaseHtmlAttributes::Target(val) => Some(val.to_string()),
        }
    }
}
