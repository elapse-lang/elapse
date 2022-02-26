use crate::ast::Span;

pub trait Expression {
    fn display(&self) -> String;
}

pub struct Str {
    pub span: Span,
    pub string: String
}

impl Expression for Str {
    fn display(&self) -> String {
        String::from(self.string.to_string())
    }
}

pub struct Integer {
    pub span: Span,
    pub value: String
}

impl Expression for Integer {
    fn display(&self) -> String {
        String::from(self.value.to_string())
    }
}

pub struct Identifier {
    pub span: Span,
    pub name: String
}

impl Expression for Identifier {
    fn display(&self) -> String {
        format!("{}", self.name)
    }
}

pub struct ScopeResolution {
    pub span: Span,
    pub scope: Vec<Identifier>
}

impl Expression for ScopeResolution {
    fn display(&self) -> String {
        let mut scope_name: String = "".to_owned();
        for ident in &self.scope {
            if scope_name == "" {
                scope_name = format!("{}", &ident.name);
            } else {
                scope_name = format!("{}::{}", scope_name, &ident.name);
            }
        }
        scope_name
    }
}
