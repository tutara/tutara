use crate::compiler::*;
use crate::operation::*;
use inkwell::values::PointerValue;
use tutara_interpreter::{Error, Expression, Token, TokenType, Literal};

impl Compiler<'_> {
	pub fn get_variable(&self, name: &str) -> Result<PointerValue, Error> {
		let len = self.scope.len();
		for index in 0..len {
			if let Some(pointer) = self.scope[len - index - 1].variables.get(name) {
				return Ok(*pointer);
			}
		}

		Err(Error::new_compiler_error(
			"Variable not found in this scope".to_string(),
		))
	}

	pub fn set_variable(
		&self,
		name: &str,
		operator: Token,
		expression: Expression,
	) -> Result<Operation, Error> {
		use Operation::*;

		let value = if operator.r#type == TokenType::Assign {
			self.evaluate_expression(expression)?
		} else {
			return Err(Error::new_compiler_error(
				"Unsupported assignment operator".to_string(),
			));
		};

		match self.get_variable(&name) {
			Ok(pointer) => match value {
				FloatValue(value) => {
					self.builder.build_store(pointer, value);
					Ok(NoOp)
				}
				BoolValue(value) => {
					self.builder.build_store(pointer, value);
					Ok(NoOp)
				}
				_ => Err(Error::new_compiler_error(
					"Unsupported assignment operation".to_string(),
				)),
			},
			Err(_) => Err(Error::new_compiler_error(
				"Variable not found in this scope".to_string(),
			)),
		}
	}
}

impl Compiler<'_> {
	pub fn evaluate_declaration(&mut self, expression: Expression) -> Result<Operation, Error> {
		use self::Literal::*;
		use Expression::*;
		use Operation::*;

		match expression {
			Assignment(identifier, _operator, inner_expression) => match identifier.literal {
				Some(String(name)) => {
					let pointer;

					match self.evaluate_expression(*inner_expression)? {
						FloatValue(value) => {
							pointer = self.builder.build_alloca(self.context.f64_type(), &name);
							self.builder.build_store(pointer, value);
						}
						BoolValue(value) => {
							pointer = self.builder.build_alloca(self.context.bool_type(), &name);
							self.builder.build_store(pointer, value);
						}
						_ => {
							return Err(Error::new_compiler_error(
								"Unsupported assignment operation".to_string(),
							))
						}
					};
					let scope_index = self.scope.len() - 1;

					self.scope[scope_index]
						.variables
						.insert(name.to_string(), pointer);
					Ok(NoOp)
				}
				_ => Err(Error::new_compiler_error(
					"Unsupported expression".to_string(),
				)),
			},
			_ => Err(Error::new_compiler_error(
				"Unsupported expression".to_string(),
			)),
		}
	}
}
