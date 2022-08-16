pub trait ColGroupAttribute: Attribute {}
use crate::{col::Span, Attribute};

impl ColGroupAttribute for Span {}
