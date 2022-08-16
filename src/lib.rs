//! # Domatt
//!
//! `domatt` provides types representing DOM attributes and is used to
//! provide static type safety when interacting with the DOM in WASM
//! applications. Instead of simply passing strings, you can use these
//! types to generate keys and values in the proper format. No typos!
//!
//! Credits: the type categories themselves come from [DefinitelyTyped](https://github.com/DefinitelyTyped/DefinitelyTyped).
//! The rest of the design is my own.
//!
//! ## Limitations
//!
//! Currently `domatt` does not type check the formatting of dates or unit strings ("2px", "80%").
//! These would be excellent features to add. Feel free to put in a PR.
//!
//! ## Issues
//!
//! Are we missing any attributes? Are any of the attributes not serializing correctly?
//! Please file an [issue](https://github.com/toadslop/domatt/issues), or if you'd like
//! to solve it yourself feel free to put in a PR.

use std::fmt::{self, Debug};
use strum::AsRefStr;
use web_sys::Element;

#[cfg(feature = "anchor")]
pub mod anchor;

#[cfg(feature = "area")]
pub mod area;

#[cfg(feature = "aria")]
pub mod aria;

#[cfg(feature = "button")]
pub mod button;

#[cfg(feature = "global")]
pub mod global;

#[cfg(feature = "svg")]
pub mod svg;

#[cfg(feature = "audio")]
pub mod audio;

#[cfg(feature = "base")]
pub mod base;

#[cfg(feature = "blockquote")]
pub mod blockquote;

#[cfg(feature = "canvas")]
pub mod canvas;

#[cfg(feature = "col")]
pub mod col;

#[cfg(feature = "colgroup")]
pub mod colgroup;

#[cfg(feature = "data")]
mod data;

#[cfg(feature = "details")]
pub mod details;

/// Marks a type as a DOM attribute.
pub trait Attribute: Debug {
    type InputType;

    fn new(value: Self::InputType) -> Self;

    /// Returns a string representing the key of a DOM attribute.
    fn get_key(&self) -> &str;

    /// Returns an `Option<String>` representing the value of a DOM
    /// attribute. `None` indicates a boolean attribute, such as `disabled`,
    /// which has no value.
    fn get_val(&self) -> Option<&str>;
}

/// Convenience method for setting an attribute on an element.
pub fn set_attribute<T: Attribute>(element: &Element, attribute: &T) -> Result<(), AttributeError> {
    let key = attribute.get_key().to_owned();
    let value = attribute.get_val().unwrap_or_default().to_owned();
    element
        .set_attribute(&key, &value)
        .map_err(|_err| AttributeError {
            key,
            value,
            tag: element.tag_name(),
            message: "Failed to set attribute.".to_owned(),
        })?;
    Ok(())
}

/// Error indicating an error with performing operations with attributes on an element.
#[derive(Debug, Clone)]
pub struct AttributeError {
    key: String,
    value: String,
    tag: String,
    message: String,
}

impl fmt::Display for AttributeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} Key: {}, Value: {}, Element Tag: {}",
            self.message, self.key, self.value, self.tag
        )
    }
}

/// An enum representing a value that could be either a number or string. It's typically
/// used to represent a number value that could have an optional unit attached to it.
#[derive(Debug)]
pub enum NumberOrString {
    Number(f64),
    String(String),
}

impl NumberOrString {
    pub fn to_string(&self) -> String {
        match self {
            NumberOrString::Number(num) => num.to_string(),
            NumberOrString::String(string) => string.to_owned(),
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

macro_rules! add_impls {
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
        impl SvgAttribute for $attr_struct {}
    };
}

pub(crate) use add_impls;
