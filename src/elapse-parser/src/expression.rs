
pub trait Expression {
    fn display(&self) -> String;
}

pub struct Identifier {
    pub name: String
}

impl Expression for Identifier {
    fn display(&self) -> String {
        format!("{}", self.name)
    }
}

pub struct ScopeResolution {
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
