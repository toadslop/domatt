use crate::Attribute;
pub trait CanvasAttribute: Attribute {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/canvas#attr-height>
#[derive(Debug, Attribute)]
#[attribute("lowercase", u16)]
pub struct Height(String);
impl CanvasAttribute for Height {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/canvas#attr-width>
#[derive(Debug, Attribute)]
#[attribute("lowercase", u16)]
pub struct Width(String);
impl CanvasAttribute for Width {}
