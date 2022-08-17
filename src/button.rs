use std::fmt::Debug;

use crate::{Attribute, TargetOption};
use strum::AsRefStr;
use url::Url;

pub trait ButtonAttribute: Debug + Attribute {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/button#attr-autofocus>
#[derive(Debug)]
pub struct Autofocus;

impl Attribute for Autofocus {
    fn get_val(&self) -> Option<&str> {
        None
    }

    fn get_key(&self) -> &str {
        "autofocus"
    }

    type InputType = Self;

    fn new(value: Self::InputType) -> Self {
        Self
    }
}

impl ButtonAttribute for Autofocus {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/button#attr-disabled>
#[derive(Debug)]
pub struct Disabled;

impl Attribute for Disabled {
    fn get_val(&self) -> Option<&str> {
        None
    }

    fn get_key(&self) -> &str {
        "disabled"
    }

    type InputType = Self;

    fn new(value: Self::InputType) -> Self {
        Self
    }
}

impl ButtonAttribute for Disabled {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/button#attr-form>
#[derive(Debug)]
pub struct Form(String);

impl Attribute for Form {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_str())
    }

    fn get_key(&self) -> &str {
        "form"
    }

    type InputType = String;

    fn new(value: Self::InputType) -> Self {
        Self(value)
    }
}

impl ButtonAttribute for Form {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/button#attr-formaction>
#[derive(Debug)]
pub struct FormAction(String);

impl Attribute for FormAction {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_str())
    }

    fn get_key(&self) -> &str {
        "formaction"
    }

    type InputType = Url;

    fn new(value: Self::InputType) -> Self {
        Self(value.to_string())
    }
}

impl ButtonAttribute for FormAction {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/button#attr-formenctype>
#[derive(Debug)]
pub struct FormEncType(FormEncTypeOption);

impl Attribute for FormEncType {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_ref())
    }

    fn get_key(&self) -> &str {
        "formenctype"
    }

    type InputType = FormEncTypeOption;

    fn new(value: Self::InputType) -> Self {
        Self(value)
    }
}

impl ButtonAttribute for FormEncType {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/button#attr-formmethod>
#[derive(Debug)]
pub struct FormMethod(FormMethodOption);

impl Attribute for FormMethod {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_ref())
    }

    fn get_key(&self) -> &str {
        "formmethod"
    }

    type InputType = FormMethodOption;

    fn new(value: Self::InputType) -> Self {
        Self(value)
    }
}

impl ButtonAttribute for FormMethod {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/button#attr-formnovalidate>
#[derive(Debug)]
pub struct FormNoValidate;

impl Attribute for FormNoValidate {
    fn get_val(&self) -> Option<&str> {
        None
    }

    fn get_key(&self) -> &str {
        "formnovalidate"
    }

    type InputType = Self;

    fn new(value: Self::InputType) -> Self {
        Self
    }
}

impl ButtonAttribute for FormNoValidate {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/button#attr-formtarget>
#[derive(Debug)]
pub struct FormTarget(TargetOption);

impl Attribute for FormTarget {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_ref())
    }

    fn get_key(&self) -> &str {
        "formtarget"
    }

    type InputType = TargetOption;

    fn new(value: Self::InputType) -> Self {
        Self(value)
    }
}

impl ButtonAttribute for FormTarget {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/button#attr-name>
#[derive(Debug)]
pub struct Name(String);

impl Attribute for Name {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_str())
    }

    fn get_key(&self) -> &str {
        "name"
    }

    type InputType = String;

    fn new(value: Self::InputType) -> Self {
        Self(value)
    }
}

impl ButtonAttribute for Name {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/button#attr-type>
#[derive(Debug)]
pub struct Type(ButtonTypeOption);

impl Attribute for Type {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_ref())
    }

    fn get_key(&self) -> &str {
        "type"
    }

    type InputType = ButtonTypeOption;

    fn new(value: Self::InputType) -> Self {
        Self(value)
    }
}

impl ButtonAttribute for Type {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/button#attr-value>
#[derive(Debug)]
pub struct Value(String);

impl Attribute for Value {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_str())
    }

    fn get_key(&self) -> &str {
        "value"
    }

    type InputType = String;

    fn new(value: Self::InputType) -> Self {
        Self(value)
    }
}

impl ButtonAttribute for Value {}

/// An enum representing the different options for the type attribute of a button element.
///
///  <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/button#attr-type>
#[derive(Debug, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum ButtonTypeOption {
    Submit,
    Reset,
    Button,
}

/// An enum representing the different options for the formenctype attribute of a button element.
///
///  <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/button#attr-type>
#[derive(Debug, AsRefStr)]
pub enum FormEncTypeOption {
    #[strum(serialize = "application/x-www-form-urlencoded")]
    Application,
    #[strum(serialize = "multipart/form-data")]
    Multipart,
    #[strum(serialize = "text/plain")]
    Text,
}

/// An enum representing the different options for the formmethod attribute of a button element.
///
///  <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/button#attr-type>
#[derive(Debug, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum FormMethodOption {
    Post,
    Get,
}
