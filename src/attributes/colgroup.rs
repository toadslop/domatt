use super::{col::Span, Attribute};

pub trait ColGroupAttribute: Attribute {}
impl ColGroupAttribute for Span {}
