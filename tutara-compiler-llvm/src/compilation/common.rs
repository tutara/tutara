use crate::compiler::Compiler;
use crate::operation::Operation;
use tutara_interpreter::{Error, Statement};

impl Compiler<'_> {
	pub fn evaluate_body(&mut self, statements: Vec<Statement>) -> Result<Operation, Error> {
		for statement in statements {
			self.evaluate_statement(statement)?;
		}

		Ok(Operation::NoOp)
	}
}
