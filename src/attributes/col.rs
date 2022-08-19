use super::Attribute;

pub trait ColAttribute: Attribute {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/col#attr-span>
#[derive(Debug, Attribute)]
#[attribute("lowercase", u8)]
pub struct Span(String);
impl ColAttribute for Span {}
