use crate::ast::Statement;

pub struct Import {
    pub path: String
}

impl Statement for Import {
    fn display(&self) -> String {
        format!("import {};", self.path)
    }
}