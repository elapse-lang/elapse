use crate::grammar::{ScytheParser, Rule};
use crate::expression::*;

use crate::statement::*;

use pest_consume::Error;
use pest_consume::match_nodes;
use pest_consume::Parser;

type Result<T> = std::result::Result<T, Error<Rule>>;
type Node<'i> = pest_consume::Node<'i, Rule, ()>;

#[pest_consume::parser]
impl ScytheParser {
    fn EOI(_input: Node) -> Result<()> {
        Ok(())
    }
    fn identifier(input: Node) -> Result<Identifier> {
        Ok(Identifier{ name: input.as_str().to_owned() })
    }
    fn scope_resolution(input: Node) -> Result<ScopeResolution> {
        Ok(match_nodes!(input.into_children();
            [identifier(ident)..] => {
                let identifiers = ident.collect();
                ScopeResolution{ scope: identifiers }
            }
        ))
    }
    fn integer_decimal(input: Node) -> Result<i128> {
        input.as_str()
            .parse::<i128>()
            .map_err(|e| input.error(e))
    }
    fn string(input: Node) -> Result<String> {
        Ok(input.as_str().to_owned())
    }
    // fn infix(input: Node) -> Result<Infix> {

    // }
    fn import_statement(input: Node) -> Result<impl Statement> {
        Ok(match_nodes!(input.into_children();
            [string(path)] => {
                import::Import {
                    path
                }
            }
        ))
    }
    fn use_statement(input: Node) -> Result<impl Statement> {
        Ok(match_nodes!(input.into_children();
            [scope_resolution(scope)] => {
                using::Use {
                    scope
                }
            }
        ))
    }
    fn program(input: Node) -> Result<Vec<Box<dyn Statement>>> {
        let nodes = { input.into_children() };
        let mut statements: Vec<Box<dyn Statement>> = Vec::new();
        for node in nodes {
            let rule = node.as_rule();
            match rule {
                Rule::EOI => println!("end of input"),
                Rule::import_statement => statements.push(Box::new(Self::import_statement(node)?)),
                Rule::use_statement => statements.push(Box::new(Self::use_statement(node)?)),
                _ => {}
            }
        }

        for statement in &statements {
            println!("{}", statement.display());
        }
        Ok(statements)
    }
}

pub trait Statement {
    fn display(&self) -> String;
}

pub fn parse_scythe(input_str: &str) -> Result<Vec<Box<dyn Statement>>> {
    let inputs = ScytheParser::parse_with_userdata(Rule::program, input_str, ())?;
    let input = inputs.single()?;
    ScytheParser::program(input)
}