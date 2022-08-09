use strum::AsRefStr;
use url::Url;

use crate::{Attribute, HtmlAttributeAnchorTarget, HtmlAttributeReferrerPolicy};

/// An enum defining the different anchor-element-specific attribute keys. Each variant takes either tuple
/// that represents the valid values for the attributes or nothing to represent a boolean
/// attribute.
#[derive(Debug, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum AnchorHtmlAttributes {
    Download(Option<&'static str>),
    Href(&'static Url),
    HrefLang(&'static str),
    Media(&'static str),
    Ping(&'static Vec<Url>),
    Rel(&'static Vec<ATagRel>),
    // Target(&'static HtmlAttributeAnchorTarget),
    Type(&'static str), // TODO: create an enum representing the various MIME types. Ref: (https://developer.mozilla.org/en-US/docs/Glossary/MIME_type)
    ReferrerPolicy(&'static HtmlAttributeReferrerPolicy),
}

impl<'a> Attribute<'a> for AnchorHtmlAttributes
where
    Self: 'static,
{
    fn get_key(&self) -> &str {
        self.as_ref()
    }

    fn get_val(&self) -> Option<&'static str> {
        match &self {
            AnchorHtmlAttributes::Download(val) => val.map(|val| val),
            AnchorHtmlAttributes::Href(val) => Some(val.as_str()),
            AnchorHtmlAttributes::HrefLang(val) => Some(val),
            AnchorHtmlAttributes::Media(val) => Some(val),
            AnchorHtmlAttributes::Ping(val) => Some(
                val.iter()
                    .map(|url| url.as_ref())
                    .collect::<Vec<&str>>()
                    .join(" ")
                    .as_str(),
            ),
            AnchorHtmlAttributes::Rel(val) => Some(
                val.iter()
                    .map(|rel| rel.as_ref())
                    .collect::<Vec<&str>>()
                    .join(" ")
                    .as_str(),
            ),
            AnchorHtmlAttributes::Target(val) => Some(val.as_ref()),
            AnchorHtmlAttributes::Type(val) => Some(val),
            AnchorHtmlAttributes::ReferrerPolicy(val) => Some(val.as_ref()),
        }
    }
}

/// An enum defining the options for the rel attribute of a link tag.
///
/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Link_types>
#[derive(Debug, AsRefStr)]
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
