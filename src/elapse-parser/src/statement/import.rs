use crate::ast::{Span, Statement};
use crate::expression::Str;

pub struct Import {
    pub span: Span,
    pub path: Str
}

impl Statement for Import {
    fn display(&self) -> String {
        format!("import {};", self.path.string)
    }
}