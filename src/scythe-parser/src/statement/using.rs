use crate::ast::Statement;
use crate::expression::{Expression, ScopeResolution};

pub struct Use {
    pub scope: ScopeResolution
}

impl Statement for Use {
    fn display(&self) -> String {
        format!("use {};", self.scope.display())
    }
}