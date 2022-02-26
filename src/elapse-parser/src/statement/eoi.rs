use crate::ast::{Span, Statement};

pub struct Eoi {
    span: Span
}

impl Statement for Eoi {
    fn display(&self) -> String {
        String::from("Eoi")
    }
}