use std::hash::Hash;
use strum::Display;

use crate::{Attribute, NumberOrString};

/// An enum of many different html attribute keys, regardless of element type. This can be
/// used in conjunction with [html_attributes] to give you access to practically all HTML
/// attributes. Each variant takes either tuple that represents the valid values for the
/// attributes or nothing to represent a boolean attribute.
#[derive(Debug, Clone, Display, Eq)]
#[strum(serialize_all = "lowercase")]
pub enum MiscHtmlAttributes {
    // Standard HTML Attributes
    Accept(String),
    #[strum(serialize = "accept-charset")]
    AcceptCharset(String),
    Action(String),
    AllowFullScreen(bool),
    AllowTransparency(bool),
    Alt(String),
    As(String),
    Async,
    AutoComplete(String),
    AutoFocus,
    AutoPlay,
    Capture(Capture),
    CellPadding(NumberOrString),
    CellSpacing(NumberOrString),
    CharSet(String),
    Challenge(String),
    Checked,
    Cite(String),
    ClassID(String),
    Cols(i64),
    ColSpan(i64),
    Content(String),
    Controls,
    Coords(String),
    CrossOrigin(String),
    Data(String),
    DateTime(String), // TODO: accept Chrono types and convert them to the appropriate strings
    Default,
    Defer,
    Disabled,
    Download,
    EncType(String),
    Form(String),
    FormAction(String),
    FormEncType(String),
    FormMethod(String),
    FormNoValidate,
    FormTarget(String),
    FrameBorder(FrameBorder),
    Headers(String),
    Height(NumberOrString),
    High(i64),
    Href(String),
    HrefLang(String),
    For(String),
    HttpEquiv(String),
    Integrity(String),
    KeyParams(String),
    KeyType(String),
    Kind(String),
    Label(String),
    List(String),
    Loop,
    Low(i64),
    Manifest(String),
    MarginHeight(i64),
    MarginWidth(i64),
    Max(NumberOrString),
    MaxLength(i64),
    Media(String),
    MediaGroup(String),
    Method(String),
    Min(NumberOrString),
    MinLength(i64),
    Multiple,
    Muted,
    Name(String),
    Nonce(String),
    NoValidate,
    Open,
    Optimum(i64),
    Pattern(String),
    Placeholder(String),
    PlaysInline,
    Poster(String),
    Preload(String),
    ReadOnly,
    Rel(String),
    Required,
    Reversed,
    Rows(i64),
    RowSpan(i64),
    Sandbox(String),
    Scope(String),
    Scoped,
    Scrolling(String),
    Seamless,
    Selected,
    Shape(String),
    Size(i64),
    Sizes(String),
    Span(i64),
    Src(String),
    SrcDoc(String),
    SrcLang(String),
    SrcSet(String),
    Start(i64),
    Step(NumberOrString),
    Summary(String),
    Target(String),
    Type(String),
    UseMap(String),
    Value(String),
    Width(NumberOrString),
    Wwmode(String),
    Wrap(String),
}

impl Attribute for MiscHtmlAttributes {
    fn get_key(&self) -> String {
        self.to_string()
    }

    fn get_val(&self) -> Option<String> {
        match self {
            MiscHtmlAttributes::Accept(val) => Some(val.to_string()),
            MiscHtmlAttributes::AcceptCharset(val) => Some(val.to_string()),
            MiscHtmlAttributes::Action(val) => Some(val.to_string()),
            MiscHtmlAttributes::AllowFullScreen(val) => Some(val.to_string()),
            MiscHtmlAttributes::AllowTransparency(val) => Some(val.to_string()),
            MiscHtmlAttributes::Alt(val) => Some(val.to_string()),
            MiscHtmlAttributes::As(val) => Some(val.to_string()),
            MiscHtmlAttributes::Async => None,
            MiscHtmlAttributes::AutoComplete(val) => Some(val.to_string()),
            MiscHtmlAttributes::AutoFocus => None,
            MiscHtmlAttributes::AutoPlay => None,
            MiscHtmlAttributes::Capture(val) => Some(val.to_string()),
            MiscHtmlAttributes::CellPadding(val) => Some(val.to_string()),
            MiscHtmlAttributes::CellSpacing(val) => Some(val.to_string()),
            MiscHtmlAttributes::CharSet(val) => Some(val.to_string()),
            MiscHtmlAttributes::Challenge(val) => Some(val.to_string()),
            MiscHtmlAttributes::Checked => None,
            MiscHtmlAttributes::Cite(val) => Some(val.to_string()),
            MiscHtmlAttributes::ClassID(val) => Some(val.to_string()),
            MiscHtmlAttributes::Cols(val) => Some(val.to_string()),
            MiscHtmlAttributes::ColSpan(val) => Some(val.to_string()),
            MiscHtmlAttributes::Content(val) => Some(val.to_string()),
            MiscHtmlAttributes::Controls => None,
            MiscHtmlAttributes::Coords(val) => Some(val.to_string()),
            MiscHtmlAttributes::CrossOrigin(val) => Some(val.to_string()),
            MiscHtmlAttributes::Data(val) => Some(val.to_string()),
            MiscHtmlAttributes::DateTime(val) => Some(val.to_string()),
            MiscHtmlAttributes::Default => None,
            MiscHtmlAttributes::Defer => None,
            MiscHtmlAttributes::Disabled => None,
            MiscHtmlAttributes::Download => None,
            MiscHtmlAttributes::EncType(val) => Some(val.to_string()),
            MiscHtmlAttributes::Form(val) => Some(val.to_string()),
            MiscHtmlAttributes::FormAction(val) => Some(val.to_string()),
            MiscHtmlAttributes::FormEncType(val) => Some(val.to_string()),
            MiscHtmlAttributes::FormMethod(val) => Some(val.to_string()),
            MiscHtmlAttributes::FormNoValidate => None,
            MiscHtmlAttributes::FormTarget(val) => Some(val.to_string()),
            MiscHtmlAttributes::FrameBorder(val) => Some(val.to_string()),
            MiscHtmlAttributes::Headers(val) => Some(val.to_string()),
            MiscHtmlAttributes::Height(val) => Some(val.to_string()),
            MiscHtmlAttributes::High(val) => Some(val.to_string()),
            MiscHtmlAttributes::Href(val) => Some(val.to_string()),
            MiscHtmlAttributes::HrefLang(val) => Some(val.to_string()),
            MiscHtmlAttributes::For(val) => Some(val.to_string()),
            MiscHtmlAttributes::HttpEquiv(val) => Some(val.to_string()),
            MiscHtmlAttributes::Integrity(val) => Some(val.to_string()),
            MiscHtmlAttributes::KeyParams(val) => Some(val.to_string()),
            MiscHtmlAttributes::KeyType(val) => Some(val.to_string()),
            MiscHtmlAttributes::Kind(val) => Some(val.to_string()),
            MiscHtmlAttributes::Label(val) => Some(val.to_string()),
            MiscHtmlAttributes::List(val) => Some(val.to_string()),
            MiscHtmlAttributes::Loop => None,
            MiscHtmlAttributes::Low(val) => Some(val.to_string()),
            MiscHtmlAttributes::Manifest(val) => Some(val.to_string()),
            MiscHtmlAttributes::MarginHeight(val) => Some(val.to_string()),
            MiscHtmlAttributes::MarginWidth(val) => Some(val.to_string()),
            MiscHtmlAttributes::Max(val) => Some(val.to_string()),
            MiscHtmlAttributes::MaxLength(val) => Some(val.to_string()),
            MiscHtmlAttributes::Media(val) => Some(val.to_string()),
            MiscHtmlAttributes::MediaGroup(val) => Some(val.to_string()),
            MiscHtmlAttributes::Method(val) => Some(val.to_string()),
            MiscHtmlAttributes::Min(val) => Some(val.to_string()),
            MiscHtmlAttributes::MinLength(val) => Some(val.to_string()),
            MiscHtmlAttributes::Multiple => None,
            MiscHtmlAttributes::Muted => None,
            MiscHtmlAttributes::Name(val) => Some(val.to_string()),
            MiscHtmlAttributes::Nonce(val) => Some(val.to_string()),
            MiscHtmlAttributes::NoValidate => None,
            MiscHtmlAttributes::Open => None,
            MiscHtmlAttributes::Optimum(val) => Some(val.to_string()),
            MiscHtmlAttributes::Pattern(val) => Some(val.to_string()),
            MiscHtmlAttributes::Placeholder(val) => Some(val.to_string()),
            MiscHtmlAttributes::PlaysInline => None,
            MiscHtmlAttributes::Poster(val) => Some(val.to_string()),
            MiscHtmlAttributes::Preload(val) => Some(val.to_string()),
            MiscHtmlAttributes::ReadOnly => None,
            MiscHtmlAttributes::Rel(val) => Some(val.to_string()),
            MiscHtmlAttributes::Required => None,
            MiscHtmlAttributes::Reversed => None,
            MiscHtmlAttributes::Rows(val) => Some(val.to_string()),
            MiscHtmlAttributes::RowSpan(val) => Some(val.to_string()),
            MiscHtmlAttributes::Sandbox(val) => Some(val.to_string()),
            MiscHtmlAttributes::Scope(val) => Some(val.to_string()),
            MiscHtmlAttributes::Scoped => None,
            MiscHtmlAttributes::Scrolling(val) => Some(val.to_string()),
            MiscHtmlAttributes::Seamless => None,
            MiscHtmlAttributes::Selected => None,
            MiscHtmlAttributes::Shape(val) => Some(val.to_string()),
            MiscHtmlAttributes::Size(val) => Some(val.to_string()),
            MiscHtmlAttributes::Sizes(val) => Some(val.to_string()),
            MiscHtmlAttributes::Span(val) => Some(val.to_string()),
            MiscHtmlAttributes::Src(val) => Some(val.to_string()),
            MiscHtmlAttributes::SrcDoc(val) => Some(val.to_string()),
            MiscHtmlAttributes::SrcLang(val) => Some(val.to_string()),
            MiscHtmlAttributes::SrcSet(val) => Some(val.to_string()),
            MiscHtmlAttributes::Start(val) => Some(val.to_string()),
            MiscHtmlAttributes::Step(val) => Some(val.to_string()),
            MiscHtmlAttributes::Summary(val) => Some(val.to_string()),
            MiscHtmlAttributes::Target(val) => Some(val.to_string()),
            MiscHtmlAttributes::Type(val) => Some(val.to_string()),
            MiscHtmlAttributes::UseMap(val) => Some(val.to_string()),
            MiscHtmlAttributes::Value(val) => Some(val.to_string()),
            MiscHtmlAttributes::Width(val) => Some(val.to_string()),
            MiscHtmlAttributes::Wwmode(val) => Some(val.to_string()),
            MiscHtmlAttributes::Wrap(val) => Some(val.to_string()),
        }
    }
}

/// An enum representing the different options for the `frameborder` attribute.
///
/// <https://www.w3resource.com/html/attributes/html-frameborder-attribute.php#:~:text=The%20purpose%20of%20the%20HTML,1%E2%80%9D%20or%20%E2%80%9C0%E2%80%9D.&text=HTML%20frameborder%20attribute%20supports%20frame%20and%20iframe%20elements.>
#[derive(Debug, PartialEq, Clone, Display, Eq)]
pub enum FrameBorder {
    #[strum(serialize = "0")]
    Zero,
    #[strum(serialize = "1")]
    One,
}

/// An enum representing the different options for the `capture` attribute.
///
/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/inputmode>
#[derive(Debug, PartialEq, Clone, Display, Eq)]
#[strum(serialize_all = "lowercase")]
pub enum Capture {
    User,
    Environment,
}

impl PartialEq for MiscHtmlAttributes {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

impl Hash for MiscHtmlAttributes {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}
