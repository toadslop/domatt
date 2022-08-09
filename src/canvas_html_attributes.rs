use strum::{AsRefStr, Display};

use crate::Attribute;

/// An enum defining the different canvas-element-specific attribute keys. Each variant takes either tuple
/// that represents the valid values for the attributes or nothing to represent a boolean
/// attribute.
#[derive(Debug, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum CanvasHtmlAttributes {
    /// Canvas height takes just a number of pixels with no unit specified. Since the largest screens
    /// available today are well within the max value of a u-16 and because negative height doesn't make
    /// sense, we use u16 to reduce unneccessary memory usage.
    Height(u16),
    /// Canvas width takes just a number of pixels with no unit specified. Since the largest screens
    /// available today are well within the max value of a u-16 and because negative height doesn't make
    /// sense, we use u16 to reduce unneccessary memory usage.
    Width(u16),
}

impl<'a> Attribute<'a> for CanvasHtmlAttributes {
    fn get_key(&self) -> &str {
        self.as_ref()
    }

    fn get_val(&self) -> Option<&str> {
        match &self {
            CanvasHtmlAttributes::Height(val) => Some(val.to_string().as_ref()),
            CanvasHtmlAttributes::Width(val) => Some(val.to_string().as_ref()),
        }
    }
}
