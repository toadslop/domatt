#[cfg(feature = "anchor")]
pub mod anchor;

#[cfg(feature = "area")]
pub mod area;

#[cfg(feature = "aria")]
pub mod aria;

#[cfg(feature = "audio")]
pub mod audio;

#[cfg(feature = "base")]
pub mod base;

#[cfg(feature = "blockquote")]
pub mod blockquote;

#[cfg(feature = "button")]
pub mod button;

#[cfg(feature = "canvas")]
pub mod canvas;

#[cfg(feature = "col")]
pub mod col;

#[cfg(feature = "colgroup")]
pub mod colgroup;

#[cfg(feature = "data")]
pub mod data;

#[cfg(feature = "details")]
pub mod details;

#[cfg(feature = "details")]
pub mod global;

/// An enum representing a value that could be either a number or string. It's typically
/// used to represent a number value that could have an optional unit attached to it.
#[derive(Debug)]
pub enum NumberOrString<'a> {
    Number(f64),
    String(&'a str),
}

impl<'a> Display for NumberOrString<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NumberOrString::Number(val) => write!(f, "{}", val),
            NumberOrString::String(val) => write!(f, "{}", val),
        }
    }
}

#[derive(Debug, AsRefStr, Clone, PartialEq)]
#[strum(serialize_all = "kebab-case")]
pub enum HtmlAttributeReferrerPolicy {
    NoReferrer,
    NoReferrerWhenDowngrade,
    Origin,
    OriginWhenCrossOrigin,
    SameOrigin,
    StrictOrigin,
    StrictOriginWhenCrossOrigin,
    UnsafeUrl,
    #[strum(serialize = "")]
    Blank,
}

#[derive(Debug, AsRefStr, Clone, PartialEq)]
pub enum TargetOption {
    #[strum(serialize = "_self")]
    Self_,
    #[strum(serialize = "_blank")]
    Blank,
    #[strum(serialize = "_parent")]
    Parent,
    #[strum(serialize = "_top")]
    Top,
    Custom(String),
}

macro_rules! set_attributes {
    ($attr_struct:ty ) => {
        impl AnchorAttribute for $attr_struct {}
        impl AreaAttribute for $attr_struct {}
        impl AudioAttribute for $attr_struct {}
        impl BaseAttribute for $attr_struct {}
        impl BlockQuoteAttribute for $attr_struct {}
        impl ButtonAttribute for $attr_struct {}
        impl CanvasAttribute for $attr_struct {}
        impl ColAttribute for $attr_struct {}
        impl ColGroupAttribute for $attr_struct {}
        impl DataAttribute for $attr_struct {}
        impl DetailsAttribute for $attr_struct {}
    };
}

use std::fmt::Display;

pub(crate) use set_attributes;
use strum::AsRefStr;
