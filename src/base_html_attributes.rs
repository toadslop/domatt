use strum::{AsRefStr, Display};
use url::Url;

use crate::{Attribute, HtmlAttributeAnchorTarget};

/// An enum defining the different base-element-specific attribute keys. Each variant takes either tuple
/// that represents the valid values for the attributes or nothing to represent a boolean
/// attribute.
#[derive(Debug, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum BaseHtmlAttributes<'a> {
    Href(&'a Url),
    Target(HtmlAttributeAnchorTarget),
}

impl<'a> Attribute<'a> for BaseHtmlAttributes<'a> {
    fn get_key(&self) -> &str {
        self.as_ref()
    }

    fn get_val(&self) -> Option<&str> {
        match &self {
            BaseHtmlAttributes::Href(val) => Some(val.as_ref()),
            BaseHtmlAttributes::Target(val) => Some(val.as_ref()),
        }
    }
}
