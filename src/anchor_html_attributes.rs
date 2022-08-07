use strum::Display;
use url::Url;

use crate::{Attribute, HtmlAttributeAnchorTarget, HtmlAttributeReferrerPolicy};

/// An enum defining the different anchor-element-specific attribute keys. Each variant takes either tuple
/// that represents the valid values for the attributes or nothing to represent a boolean
/// attribute.
#[derive(Debug, Clone, Display)]
#[strum(serialize_all = "lowercase")]
pub enum AnchorHtmlAttributes<'a> {
    Download(Option<&'a str>),
    Href(&'a Url),
    HrefLang(&'a str),
    Media(&'a str),
    Ping(&'a Vec<Url>),
    Rel(&'a Vec<ATagRel>),
    Target(HtmlAttributeAnchorTarget),
    Type(&'a str), // TODO: create an enum representing the various MIME types. Ref: (https://developer.mozilla.org/en-US/docs/Glossary/MIME_type)
    ReferrerPolicy(&'a HtmlAttributeReferrerPolicy),
}

impl<'a> Attribute for AnchorHtmlAttributes<'a> {
    fn get_key(&self) -> String {
        self.to_string()
    }

    fn get_val(&self) -> Option<String> {
        match &self {
            AnchorHtmlAttributes::Download(val) => val.map(|val| val.to_owned()),
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

/// An enum defining the options for the rel attribute of a link tag.
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
