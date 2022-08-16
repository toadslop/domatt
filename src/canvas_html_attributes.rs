use crate::Attribute;
pub trait CanvasAttribute: Attribute {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/canvas#attr-height>
#[derive(Debug, Clone, PartialEq)]
pub struct Height {
    val: String,
}

impl Height {
    pub fn new(val: u16) -> Self {
        Self {
            val: val.to_string(),
        }
    }
}

impl Attribute for Height {
    fn get_val(&self) -> Option<&str> {
        Some(self.val.as_ref())
    }

    fn get_key(&self) -> &str {
        "height"
    }
}

impl CanvasAttribute for Height {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/canvas#attr-width>
#[derive(Debug, Clone, PartialEq)]
pub struct Width {
    val: String,
}

impl Width {
    pub fn new(val: u16) -> Self {
        Self {
            val: val.to_string(),
        }
    }
}
// Note: unsigned because we can't have a negative colcount and 8-bit because you
// could never render more than 255 columns on a screen, so we do this for optimization

impl Attribute for Width {
    fn get_val(&self) -> Option<&str> {
        Some(self.val.as_ref())
    }

    fn get_key(&self) -> &str {
        "width"
    }
}

impl CanvasAttribute for Width {}
