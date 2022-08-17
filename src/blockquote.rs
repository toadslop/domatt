use crate::Attribute;
use url::Url;

pub trait BlockQuoteAttribute: Attribute {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/blockquote#attr-cite>
#[derive(Debug, Attribute)]
#[attribute("lowercase", Url)]
pub struct Cite(String);
impl BlockQuoteAttribute for Cite {}
