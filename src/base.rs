pub trait BaseAttribute: Attribute {}

pub use crate::anchor::Href;
use crate::Attribute;
impl BaseAttribute for Href {}

pub use crate::anchor::Target;
impl BaseAttribute for Target {}
