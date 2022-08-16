use crate::Attribute;
use url::Url;

pub trait BlockQuoteAttribute: Attribute {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/blockquote#attr-cite>
#[derive(Debug, Clone, PartialEq)]
pub struct Cite(Url);

impl Attribute for Cite {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_str())
    }

    fn get_key(&self) -> &str {
        "cite"
    }
}

impl BlockQuoteAttribute for Cite {}
