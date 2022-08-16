pub trait BaseAttribute: Attribute {}

pub use crate::html::anchor::Href;
use crate::Attribute;
impl BaseAttribute for Href {}

pub use crate::html::anchor::Target;
impl BaseAttribute for Target {}
