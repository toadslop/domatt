use crate::{Attribute, HtmlAttributeReferrerPolicy, TargetOption};
use std::fmt::Debug;
use strum::AsRefStr;
use url::Url;

pub trait AnchorAttribute: Attribute {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/a#attr-download>
#[derive(Debug)]
pub struct Download(Option<String>);

impl Attribute for Download {
    type InputType = Option<String>;

    fn new(val: Option<String>) -> Self {
        Self(val)
    }

    fn get_val(&self) -> Option<&str> {
        self.0.as_ref().map(|x| &**x)
    }

    fn get_key(&self) -> &str {
        "download"
    }
}

impl AnchorAttribute for Download {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/a#attr-href>
#[derive(Debug)]
pub struct Href(Url);

impl Attribute for Href {
    type InputType = Url;

    fn new(val: Url) -> Self {
        Self(val)
    }

    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_str())
    }

    fn get_key(&self) -> &str {
        "href"
    }
}

impl AnchorAttribute for Href {}

// TODO: create a data structure to help generate language tags.
// https://gist.github.com/msikma/8912e62ed866778ff8cd
/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/a#attr-hreflang>
#[derive(Debug)]
pub struct HrefLang(String);

impl Attribute for HrefLang {
    type InputType = String;

    fn new(val: String) -> Self {
        Self(val)
    }

    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_str())
    }

    fn get_key(&self) -> &str {
        "hreflang"
    }
}

impl AnchorAttribute for HrefLang {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/a#attr-ping>
#[derive(Debug)]
pub struct Ping(String);

impl Attribute for Ping {
    type InputType = Vec<Url>;

    fn new(urls: Vec<Url>) -> Self {
        Self(
            urls.iter()
                .map(Url::as_str)
                .collect::<Vec<&str>>()
                .join(" "),
        )
    }

    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_ref())
    }

    fn get_key(&self) -> &str {
        "ping"
    }
}

impl AnchorAttribute for Ping {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/a#attr-referrerpolicy>
#[derive(Debug)]
pub struct ReferrerPolicy(HtmlAttributeReferrerPolicy);

impl Attribute for ReferrerPolicy {
    type InputType = HtmlAttributeReferrerPolicy;

    fn new(val: HtmlAttributeReferrerPolicy) -> Self {
        Self(val)
    }

    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_ref())
    }

    fn get_key(&self) -> &str {
        "referrerpolicy"
    }
}

impl AnchorAttribute for ReferrerPolicy {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/a#attr-rel>
#[derive(Debug)]
pub struct Rel(String);

impl Attribute for Rel {
    type InputType = Vec<ATagRel>;

    fn new(rels: Vec<ATagRel>) -> Self {
        Self(
            rels.iter()
                .map(ATagRel::as_ref)
                .collect::<Vec<&str>>()
                .join(" "),
        )
    }

    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_ref())
    }

    fn get_key(&self) -> &str {
        "rel"
    }
}

impl AnchorAttribute for Rel {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/a#attr-target>
#[derive(Debug)]
pub struct Target(TargetOption);

impl Attribute for Target {
    type InputType = TargetOption;

    fn new(val: TargetOption) -> Self {
        Self(val)
    }

    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_ref())
    }

    fn get_key(&self) -> &str {
        "target"
    }
}

impl AnchorAttribute for Target {}

// TODO: create an enum representing the various MIME types or a struct for generating mime types.
// Ref: (https://developer.mozilla.org/en-US/docs/Glossary/MIME_type)
/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/a#attr-type>
#[derive(Debug)]
pub struct Type(String);

impl Attribute for Type {
    type InputType = String;

    fn new(val: String) -> Self {
        Self(val)
    }

    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_ref())
    }

    fn get_key(&self) -> &str {
        "target"
    }
}

impl AnchorAttribute for Type {}

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
