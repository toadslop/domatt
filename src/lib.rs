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

use std::fmt::{self, Debug, Display};

use strum::Display;
use web_sys::Element;

mod aria_attributes;
pub use aria_attributes::*;

mod button_html_attributes;
pub use button_html_attributes::*;

mod html_attributes;
pub use html_attributes::*;

mod misc_html_attributes;
pub use misc_html_attributes::*;

mod svg_attributes;
pub use svg_attributes::*;

mod anchor_html_attributes;
pub use anchor_html_attributes::*;

mod audio_html_attributes;
pub use audio_html_attributes::*;

mod area_html_attributes;
pub use area_html_attributes::*;

mod base_html_attributes;
pub use base_html_attributes::*;

mod blockquote_html_attributes;
pub use blockquote_html_attributes::*;

mod canvas_html_attributes;
pub use canvas_html_attributes::*;

mod col_html_attributes;
pub use col_html_attributes::*;

mod colgroup_html_attributes;
pub use colgroup_html_attributes::*;

/// Marks a type as a DOM attribute.
pub trait Attribute: Display {
    /// Returns a string representing the key of a DOM attribute.
    fn get_key(&self) -> String;

    /// Returns an `Option<String>` representing the value of a DOM
    /// attribute. `None` indicates a boolean attribute, such as `disabled`,
    /// which has no value.
    fn get_val(&self) -> Option<String>;
}

/// Convenience method for setting an attribute on an element.
pub fn set_attribute<T: Attribute>(element: &Element, attribute: &T) -> Result<(), AttributeError> {
    let key = attribute.get_key();
    let value = attribute.get_val().unwrap_or_default();
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

#[derive(Debug, Display)]
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

#[derive(Debug, Clone, Display)]
pub enum HtmlAttributeAnchorTarget {
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
