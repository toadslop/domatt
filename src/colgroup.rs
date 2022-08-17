use crate::{col::Span, Attribute};

pub trait ColGroupAttribute: Attribute {}
impl ColGroupAttribute for Span {}
