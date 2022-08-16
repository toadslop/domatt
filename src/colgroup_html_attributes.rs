pub trait ColGroupAttribute: Attribute {}

use crate::Attribute;
pub use crate::Span;
impl ColGroupAttribute for Span {}
