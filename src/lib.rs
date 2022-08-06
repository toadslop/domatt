//! # Domatt
//!
//! `domatt` provides types representing DOM attributes and is used to
//! provide static type safety when interacting with the DOM in WASM
//! applications. Instead of simply passing strings, you can use these
//! types to generate keys and values in the proper format. No typos!

use std::fmt::Display;

pub mod aria_attributes;
pub mod button_html_attributes;
pub mod html_attributes;
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
