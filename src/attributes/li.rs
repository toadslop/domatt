use super::Attribute;

pub trait LiAttribute: Attribute {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/li#attr-value>
#[derive(Debug, Attribute)]
#[attribute("lowercase", u8)]
pub struct Value(String);

impl LiAttribute for Value {}
