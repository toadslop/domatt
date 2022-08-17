use crate::Attribute;

pub trait DetailsAttribute {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/details#attr-open>
#[derive(Debug, Attribute)]
#[attribute("lowercase")]
pub struct Open;
impl DetailsAttribute for Open {}
