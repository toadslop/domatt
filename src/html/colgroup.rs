use super::col::Span;
use crate::Attribute;

pub trait ColGroupAttribute: Attribute {}

impl ColGroupAttribute for Span {}
