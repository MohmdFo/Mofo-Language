use crate::parser::ASTNode;

pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Self
    }

    pub fn execute(&self, ast: ASTNode) -> Result<(), String> {
        match ast {
            ASTNode::PrintStatement(value) => {
                println!("{}", value);
                Ok(())
            }
        }
    }
}
