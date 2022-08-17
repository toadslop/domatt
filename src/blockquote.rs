use crate::Attribute;
use url::Url;

pub trait BlockQuoteAttribute: Attribute {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/blockquote#attr-cite>
#[derive(Debug)]
pub struct Cite(String);

impl Attribute for Cite {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_str())
    }

    fn get_key(&self) -> &str {
        "cite"
    }

    type InputType = Url;

    fn new(value: Self::InputType) -> Self {
        Self(value.to_string())
    }
}

impl BlockQuoteAttribute for Cite {}
