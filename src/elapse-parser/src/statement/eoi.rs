use crate::ast::Statement;

pub struct Eoi {
    line: i32
}

impl Statement for Eoi {
    fn display(&self) -> String {
        String::from("eoi")
    }
}