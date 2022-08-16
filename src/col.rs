use crate::Attribute;

pub trait ColAttribute: Attribute {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/col#attr-span>
#[derive(Debug)]
pub struct Span {
    val: String,
}

impl Span {
    pub fn new(val: u8) -> Self {
        Self {
            val: val.to_string(),
        }
    }
}

impl Attribute for Span {
    fn get_val(&self) -> Option<&str> {
        Some(self.val.as_ref())
    }

    fn get_key(&self) -> &str {
        "span"
    }
}
