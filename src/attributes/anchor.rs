use std::fmt::Debug;
use strum::AsRefStr;
use url::Url;

use super::{Attribute, ReferrerPolicyOption, TargetOption};

pub trait AnchorAttribute: Attribute {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/a#attr-download>
#[derive(Debug, Attribute)]
#[attribute("lowercase", Option<String>)]
pub struct Download(String);
impl AnchorAttribute for Download {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/a#attr-href>
#[derive(Debug, Attribute)]
#[attribute("lowercase", String)] // TODO: Make a type definiton that better handles the options of the href attribute
pub struct Href(String);

impl AnchorAttribute for Href {}

// TODO: create a data structure to help generate language tags.
// https://gist.github.com/msikma/8912e62ed866778ff8cd
/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/a#attr-hreflang>
#[derive(Debug, Attribute)]
#[attribute("lowercase", String)]
pub struct HrefLang(String);
impl AnchorAttribute for HrefLang {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/a#attr-ping>
#[derive(Debug, Attribute)]
#[attribute("lowercase", Vec<Url>)]
pub struct Ping(String);
impl AnchorAttribute for Ping {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/a#attr-referrerpolicy>
#[derive(Debug, Attribute)]
#[attribute("lowercase", ReferrerPolicyOption)]
pub struct ReferrerPolicy(ReferrerPolicyOption);
impl AnchorAttribute for ReferrerPolicy {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/a#attr-rel>
#[derive(Debug, Attribute)]
#[attribute("lowercase", Vec<ATagRel>)]
pub struct Rel(String);
impl AnchorAttribute for Rel {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/a#attr-target>
#[derive(Debug, Attribute)]
#[attribute("lowercase", TargetOption)]
pub struct Target(TargetOption);
impl AnchorAttribute for Target {}

// TODO: create an enum representing the various MIME types or a struct for generating mime types.
// Ref: (https://developer.mozilla.org/en-US/docs/Glossary/MIME_type)
/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/a#attr-type>
#[derive(Debug, Attribute)]
#[attribute("lowercase", String)]
pub struct Type(String);
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
