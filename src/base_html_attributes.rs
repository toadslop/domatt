pub trait BaseAttribute: Attribute {}

pub use crate::anchor_html_attributes::Href;
use crate::Attribute;
impl BaseAttribute for Href {}

pub use crate::anchor_html_attributes::Target;
impl BaseAttribute for Target {}
