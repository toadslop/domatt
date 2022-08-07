use strum::Display;
use url::Url;

use crate::{Attribute, HtmlAttributeAnchorTarget, HtmlAttributeReferrerPolicy};

/// An enum defining the different anchor-element-specific attribute keys. Each variant takes either tuple
/// that represents the valid values for the attributes or nothing to represent a boolean
/// attribute.
#[derive(Debug, Clone, Display, Eq)]
#[strum(serialize_all = "lowercase")]
pub enum AnchorHtmlAttributes {
    Download(Option<String>),
    Href(Url),
    HrefLang(String),
    Media(String),
    Ping(Vec<Url>),
    Rel(Vec<ATagRel>),
    Target(HtmlAttributeAnchorTarget),
    Type(String), // TODO: create an enum representing the various MIME types. Ref: (https://developer.mozilla.org/en-US/docs/Glossary/MIME_type)
    ReferrerPolicy(HtmlAttributeReferrerPolicy),
}

impl Attribute for AnchorHtmlAttributes {
    fn get_key(&self) -> String {
        self.to_string()
    }

    fn get_val(&self) -> Option<String> {
        match self {
            AnchorHtmlAttributes::Download(val) => val.to_owned(),
            AnchorHtmlAttributes::Href(val) => Some(val.to_string()),
            AnchorHtmlAttributes::HrefLang(val) => Some(val.to_string()),
            AnchorHtmlAttributes::Media(val) => Some(val.to_string()),
            AnchorHtmlAttributes::Ping(val) => Some(
                val.iter()
                    .map(|url| url.to_string())
                    .collect::<Vec<String>>()
                    .join(" "),
            ),
            AnchorHtmlAttributes::Rel(val) => Some(
                val.iter()
                    .map(|rel| rel.to_string())
                    .collect::<Vec<String>>()
                    .join(" "),
            ),
            AnchorHtmlAttributes::Target(val) => Some(val.to_string()),
            AnchorHtmlAttributes::Type(val) => Some(val.to_string()),
            AnchorHtmlAttributes::ReferrerPolicy(val) => Some(val.to_string()),
        }
    }
}

/// An enum defining the options for the type attribute of a link tag.
///
/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Link_types>
#[derive(Debug, Clone, Display, Eq, PartialEq)]
#[strum(serialize_all = "lowercase")]
pub enum ATagRel {
    Alternate,
    Archives,
    Author,
    Bookmark,
    External,
    First,
    Help,
    Index,
    Last,
    License,
    Me,
    Next,
    Nofollow,
    Noopener,
    Noreferrer,
    Opener,
    Prev,
    Search,
    Sidebar,
    Tag,
    Up,
}

impl PartialEq for AnchorHtmlAttributes {
    fn eq(&self, other: &Self) -> bool {
        // For equality, we only care if the keys are identical.
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

impl std::hash::Hash for AnchorHtmlAttributes {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}
