use crate::compiler::*;
use crate::operation::*;
use crate::scope::*;
use tutara_interpreter::{Error, Expression, Statement};

impl Compiler<'_> {
	pub fn evaluate_while(
		&mut self,
		condition: Expression,
		body: Box<Statement>,
	) -> Result<Operation, Error> {
		let parent_block = self.builder.get_insert_block().unwrap();
		let body_block = self
			.context
			.insert_basic_block_after(parent_block, "while_body_block");
		let evaluation_block = self
			.context
			.insert_basic_block_after(body_block, "while_evaluation_block");
		let continuation_block = self
			.context
			.insert_basic_block_after(evaluation_block, "while_continuation_block");
		self.builder.build_unconditional_branch(evaluation_block);

		// Body
		self.scope.push(Scope::new(ScopeContext::While(
			body_block,
			evaluation_block,
			continuation_block,
		)));
		self.builder.position_at_end(body_block);
		self.evaluate_statement(*body)?;
		self.builder.build_unconditional_branch(evaluation_block);
		self.scope.pop();

		// Evaluation
		self.builder.position_at_end(evaluation_block);
		match self.evaluate_expression(condition)? {
			Operation::BoolValue(value) => {
				self.builder
					.build_conditional_branch(value, body_block, continuation_block);
			}
			_ => {
				return Err(Error::new_compiler_error(
					"Unsupported type in condition".to_string(),
				))
			}
		}

		// Continue
		self.builder.position_at_end(continuation_block);

		Ok(Operation::NoOp)
	}

	pub fn evaluate_continue(&mut self) -> Result<Operation, Error> {
		let len = self.scope.len();
		for index in 0..len {
			if let ScopeContext::While(body, evaluation, _continuation) =
				self.scope[len - index - 1].scope_context
			{
				self.builder.build_unconditional_branch(evaluation);
				let gc = self.context.insert_basic_block_after(body, "gc");
				self.builder.position_at_end(gc);
				return Ok(Operation::NoOp);
			}
		}

		Err(Error::new_compiler_error(
			"Unable to continue in current scope".to_string(),
		))
	}

	pub fn evaluate_break(&mut self) -> Result<Operation, Error> {
		let len = self.scope.len();
		for index in 0..len {
			if let ScopeContext::While(body, _evaluation, continuation) =
				self.scope[len - index - 1].scope_context
			{
				self.builder.build_unconditional_branch(continuation);
				let gc = self.context.insert_basic_block_after(body, "gc");
				self.builder.position_at_end(gc);
				return Ok(Operation::NoOp);
			}
		}

		Err(Error::new_compiler_error(
			"Unable to break from current scope".to_string(),
		))
	}
}
