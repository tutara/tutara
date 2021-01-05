use crate::compiler::Compiler;
use crate::operation::Operation;
use crate::scope::ScopeContext;
use crate::Scope;
use tutara_interpreter::{Error, Expression, Statement};

impl Compiler<'_> {
	pub fn evaluate_if(
		&mut self,
		condition: Expression,
		true_branch: Box<Statement>,
		false_branch: Option<Box<Statement>>,
	) -> Result<Operation, Error> {
		match self.evaluate_expression(condition)? {
			Operation::BoolValue(value) => {
				let parent_block = self.builder.get_insert_block().unwrap();
				let true_block = self
					.context
					.insert_basic_block_after(parent_block, "if_true_block");
				let false_block = self
					.context
					.insert_basic_block_after(true_block, "if_false_block");
				let continuation_block = self
					.context
					.insert_basic_block_after(false_block, "if_continuation_block");

				// If
				self.builder
					.build_conditional_branch(value, true_block, false_block);

				// True
				self.scope
					.push(Scope::new(ScopeContext::If(true_block, continuation_block)));
				self.builder.position_at_end(true_block);
				self.evaluate_statement(*true_branch)?;
				self.builder.build_unconditional_branch(continuation_block);
				self.scope.pop();

				// False
				self.builder.position_at_end(false_block);
				if let Some(false_branch) = false_branch {
					self.scope.push(Scope::new(ScopeContext::If(
						false_block,
						continuation_block,
					)));
					self.evaluate_statement(*false_branch)?;
					self.scope.pop();
				}
				self.builder.build_unconditional_branch(continuation_block);

				// Continue
				self.builder.position_at_end(continuation_block);

				Ok(Operation::NoOp)
			}
			_ => Err(Error::new_compiler_error(
				"Unsupported type in condition".to_string(),
			)),
		}
	}
}
