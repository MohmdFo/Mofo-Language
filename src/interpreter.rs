use crate::parser::{ASTNode, Expression};
use std::collections::HashMap;

pub struct Interpreter {
    symbol_table: HashMap<String, (String, String)>, // Store variable names, types, and values
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
            ASTNode::Assignment(name, var_type, value) => {
                if var_type != "str" {
                    return Err(format!(
                        "Unsupported type '{}' for variable '{}'.",
                        var_type, name
                    ));
                }
                self.symbol_table.insert(name, (var_type, value));
                Ok(())
            }
        }
    }

    fn evaluate_expression(&self, expression: Expression) -> Result<String, String> {
        match expression {
            Expression::Variable(name) => self
                .symbol_table
                .get(&name)
                .map(|(_, value)| value.clone())
                .ok_or_else(|| format!("Undefined variable: {}", name)),
            Expression::StringLiteral(value) => Ok(value),
        }
    }
}
