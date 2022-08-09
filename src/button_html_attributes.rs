use crate::Attribute;
use strum::AsRefStr;

/// An enum defining the different button-specific attribute keys. Each variant takes either tuple
/// that represents the valid values for the attributes or nothing to represent a boolean
/// attribute.
#[derive(Debug, AsRefStr)]
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

impl<'a> Attribute<'a> for ButtonHtmlAttributes<'a> {
    fn get_key(&self) -> &str {
        self.as_ref()
    }

    fn get_val(&self) -> Option<&str> {
        match self {
            ButtonHtmlAttributes::AutoFocus => None,
            ButtonHtmlAttributes::Disabled => None,
            ButtonHtmlAttributes::Form(val) => Some(val),
            ButtonHtmlAttributes::FormAction(val) => Some(val),
            ButtonHtmlAttributes::FormEncType(val) => Some(val),
            ButtonHtmlAttributes::FormMethod(val) => Some(val),
            ButtonHtmlAttributes::FormNoValidate => None,
            ButtonHtmlAttributes::FormTarget(val) => Some(val),
            ButtonHtmlAttributes::Name(val) => Some(val),
            ButtonHtmlAttributes::Type(val) => Some(val.as_ref()),
            ButtonHtmlAttributes::Value(val) => Some(val),
        }
    }
}

/// An enum representing the different options for the type attribute of a button element.
///
///  <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/button#attr-type>
#[derive(Debug, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum ButtonType {
    Submit,
    Reset,
    Button,
}
