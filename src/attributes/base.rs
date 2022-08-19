pub trait BaseAttribute: Attribute {}

pub use super::anchor::Href;
use super::Attribute;
impl BaseAttribute for Href {}

pub use super::anchor::Target;
impl BaseAttribute for Target {}
