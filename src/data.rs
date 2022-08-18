use crate::Attribute;

pub trait DataAttribute {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/data#attr-value>
#[derive(Debug, Attribute)]
#[attribute("lowercase", String)]
pub struct Value(String);
impl DataAttribute for Value {}
