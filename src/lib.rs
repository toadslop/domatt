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

use std::fmt::{self, Display};

use web_sys::Element;

pub mod aria_attributes;
pub mod button_html_attributes;
pub mod html_attributes;
pub mod misc_html_attributes;
pub mod svg_attributes;

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
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum NumberOrString {
    Number(i64),
    String(String),
}
