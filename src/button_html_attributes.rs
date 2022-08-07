use crate::Attribute;
use strum::Display;

/// An enum defining the different button-specific attribute keys. Each variant takes either tuple
/// that represents the valid values for the attributes or nothing to represent a boolean
/// attribute.
#[derive(Debug, Display)]
#[strum(serialize_all = "camelCase")]
pub enum ButtonHtmlAttributes<'a> {
    AutoFocus,
    Disabled,
    Form(&'a str),
    FormAction(&'a str),
    FormEncType(&'a str),
    FormMethod(&'a str),
    FormNoValidate,
    FormTarget(&'a str),
    Name(&'a str),
    Type(&'a ButtonType),
    Value(&'a str),
}

impl<'a> Attribute for ButtonHtmlAttributes<'a> {
    fn get_key(&self) -> String {
        self.to_string()
    }

    fn get_val(&self) -> Option<String> {
        match self {
            ButtonHtmlAttributes::AutoFocus => None,
            ButtonHtmlAttributes::Disabled => None,
            ButtonHtmlAttributes::Form(val) => Some(val.to_string()),
            ButtonHtmlAttributes::FormAction(val) => Some(val.to_string()),
            ButtonHtmlAttributes::FormEncType(val) => Some(val.to_string()),
            ButtonHtmlAttributes::FormMethod(val) => Some(val.to_string()),
            ButtonHtmlAttributes::FormNoValidate => None,
            ButtonHtmlAttributes::FormTarget(val) => Some(val.to_string()),
            ButtonHtmlAttributes::Name(val) => Some(val.to_string()),
            ButtonHtmlAttributes::Type(val) => Some(val.to_string()),
            ButtonHtmlAttributes::Value(val) => Some(val.to_string()),
        }
    }
}

/// An enum representing the different options for the type attribute of a button element.
///
///  <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/button#attr-type>
#[derive(Debug, Display)]
#[strum(serialize_all = "kebab-case")]
pub enum ButtonType {
    Submit,
    Reset,
    Button,
}
