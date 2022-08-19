use super::{Attribute, TargetOption};
use std::fmt::Debug;
use strum::AsRefStr;
use url::Url;

pub trait ButtonAttribute: Debug + Attribute {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/button#attr-autofocus>
#[derive(Debug, Attribute)]
#[attribute("lowercase")]
pub struct Autofocus;
impl ButtonAttribute for Autofocus {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/button#attr-disabled>
#[derive(Debug, Attribute)]
#[attribute("lowercase")]
pub struct Disabled;
impl ButtonAttribute for Disabled {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/button#attr-form>
#[derive(Debug, Attribute)]
#[attribute("lowercase", String)]
pub struct Form(String);
impl ButtonAttribute for Form {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/button#attr-formaction>
#[derive(Debug, Attribute)]
#[attribute("lowercase", Url)]
pub struct FormAction(String);
impl ButtonAttribute for FormAction {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/button#attr-formenctype>
#[derive(Debug, Attribute)]
#[attribute("lowercase", FormEncTypeOption)]
pub struct FormEncType(FormEncTypeOption);
impl ButtonAttribute for FormEncType {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/button#attr-formmethod>
#[derive(Debug, Attribute)]
#[attribute("lowercase", FormMethodOption)]
pub struct FormMethod(FormMethodOption);
impl ButtonAttribute for FormMethod {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/button#attr-formnovalidate>
#[derive(Debug, Attribute)]
#[attribute("lowercase")]
pub struct FormNoValidate;
impl ButtonAttribute for FormNoValidate {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/button#attr-formtarget>
#[derive(Debug, Attribute)]
#[attribute("lowercase", TargetOption)]
pub struct FormTarget(TargetOption);
impl ButtonAttribute for FormTarget {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/button#attr-name>
#[derive(Debug, Attribute)]
#[attribute("lowercase", String)]
pub struct Name(String);
impl ButtonAttribute for Name {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/button#attr-type>
#[derive(Debug, Attribute)]
#[attribute("lowercase", ButtonTypeOption)]
pub struct Type(ButtonTypeOption);
impl ButtonAttribute for Type {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/button#attr-value>
#[derive(Debug, Attribute)]
#[attribute("lowercase", String)]
pub struct Value(String);
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
