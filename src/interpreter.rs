use crate::parser::{ASTNode, Expression};
use std::collections::HashMap;

pub struct Interpreter {
    symbol_table: HashMap<String, String>, // Store variable names and values
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            symbol_table: HashMap::new(),
        }
    }

    pub fn execute(&mut self, ast: ASTNode) -> Result<(), String> {
        match ast {
            ASTNode::PrintStatement(expression) => {
                let value = self.evaluate_expression(*expression)?;
                println!("{}", value);
                Ok(())
            }
            ASTNode::Assignment(name, value) => {
                self.symbol_table.insert(name, value);
                Ok(())
            }
        }
    }

    fn evaluate_expression(&self, expression: Expression) -> Result<String, String> {
        match expression {
            Expression::Variable(name) => self
                .symbol_table
                .get(&name)
                .cloned()
                .ok_or_else(|| format!("Undefined variable: {}", name)),
            Expression::StringLiteral(value) => Ok(value),
        }
    }
}
