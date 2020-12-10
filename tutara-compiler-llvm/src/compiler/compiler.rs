use inkwell::{
	basic_block::BasicBlock,
	builder::Builder,
	context::Context,
	module::Module,
	values::FloatValue,
	values::InstructionValue,
	values::{BasicValueEnum, FunctionValue, IntValue, PointerValue},
	FloatPredicate,
};
use std::collections::HashMap;
use tutara_interpreter::{Analyzer, Error, Expression, Literal, Statement, Token, TokenType};

pub struct Compiler<'a> {
	pub(super) context: &'a Context,
	pub(super) module: Module<'a>,
	pub(super) builder: Builder<'a>,

	pub(super) scope: Vec<Scope<'a>>,
	pub(super) variables: HashMap<String, PointerValue<'a>>,
}

pub enum Scope<'a> {
	While(BasicBlock<'a>, BasicBlock<'a>, BasicBlock<'a>), // Body , Evaluation , Continuation
	If(BasicBlock<'a>, BasicBlock<'a>),                    // Body , Continuation
}

pub enum Operation<'a> {
	FloatValue(FloatValue<'a>),
	BoolValue(IntValue<'a>),
	Return(InstructionValue<'a>),
	NoOp,
}

impl Compiler<'_> {
	pub fn compile<'b>(&mut self, analyzer: Analyzer<'b>) -> Result<FunctionValue, Error> {
		let fun_type = self.context.f64_type().fn_type(&[], false);
		let fun = self.module.add_function("main", fun_type, None);
		let body = self.context.append_basic_block(fun, "entry");
		self.builder.position_at_end(body);

		for result in analyzer {
			match result {
				Ok(statement) => {
					if let Operation::Return(_) = self.evaluate_statement(statement)? {
						match self.module.verify() {
							Ok(_) => return Ok(fun),
							Err(err) => return Err(Error::new_compiler_error(err.to_string())),
						}
					}
				}
				Err(error) => return Err(error),
			};
		}

		Err(Error::new_compiler_error(
			"No return statement found in script".to_string(),
		))
	}

	pub fn evaluate_statement(&mut self, statement: Statement) -> Result<Operation, Error> {
		use Statement::*;

		match statement {
			Break => self.evaluate_break(),
			Continue => self.evaluate_continue(),
			While(condition, body) => self.evaluate_while(condition, body),
			If(condition, true_branch, false_branch) => {
				self.evaluate_if(condition, true_branch, false_branch)
			}
			Expression(expression) => self.evaluate_expression(expression),
			Declaration(_mutability, _type_specification, expression) => {
				self.evaluate_declaration(expression)?;
				Ok(Operation::NoOp)
			}
			Body(statements) => self.evaluate_body(statements),
			Return(expression) => self.evaluate_return(expression),
			Comment(_) => Ok(Operation::NoOp),
			_ => Err(Error::new_compiler_error(
				"Unsupported statement".to_string(),
			)),
		}
	}

	pub fn evaluate_continue(&mut self) -> Result<Operation, Error> {
		let len = self.scope.len();
		for index in 0..len {
			if let Scope::While(body, evaluation, _continuation) = self.scope[len - index - 1] {
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
			if let Scope::While(body, _evaluation, continuation) = self.scope[len - index - 1] {
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

	pub fn evaluate_body(&mut self, statements: Vec<Statement>) -> Result<Operation, Error> {
		for statement in statements {
			self.evaluate_statement(statement)?;
		}

		Ok(Operation::NoOp)
	}

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
		self.scope.push(Scope::While(
			body_block,
			evaluation_block,
			continuation_block,
		));
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
				self.scope.push(Scope::If(true_block, continuation_block));
				self.builder.position_at_end(true_block);
				self.evaluate_statement(*true_branch)?;
				self.builder.build_unconditional_branch(continuation_block);
				self.scope.pop();

				// False
				self.builder.position_at_end(false_block);
				if let Some(false_branch) = false_branch {
					self.scope.push(Scope::If(false_block, continuation_block));
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

	pub fn evaluate_return(&self, right: Option<Expression>) -> Result<Operation, Error> {
		use Operation::*;

		match right {
			Some(expression) => match self.evaluate_expression(expression) {
				Ok(FloatValue(result)) => Ok(Return(self.builder.build_return(Some(&result)))),
				Ok(BoolValue(result)) => Ok(Return(self.builder.build_return(Some(&result)))),
				Err(err) => Err(err),
				_ => Err(Error::new_compiler_error(
					"Unsupported return operation".to_string(),
				)),
			},
			None => Ok(Return(self.builder.build_return(None))),
		}
	}

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

					self.variables.insert(name.to_string(), pointer);
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

	pub fn evaluate_operator(
		&self,
		left: Expression,
		right: Expression,
		operator: Token,
	) -> Result<Operation, Error> {
		use FloatPredicate::*;
		use Operation::*;
		use TokenType::*;

		let operations = (
			self.evaluate_expression(left)?,
			self.evaluate_expression(right)?,
		);

		if let (FloatValue(lhs), FloatValue(rhs)) = operations {
			match operator.r#type {
				Plus => Ok(FloatValue(self.builder.build_float_add(lhs, rhs, "tmpadd"))),
				Minus => Ok(FloatValue(self.builder.build_float_sub(lhs, rhs, "tmpsub"))),
				Multiply => Ok(FloatValue(self.builder.build_float_mul(lhs, rhs, "tmpmul"))),
				Division => Ok(FloatValue(self.builder.build_float_div(lhs, rhs, "tmpdiv"))),
				Exponentiation => {
					let f64_type = self.context.f64_type();
					let pow_fun = self.module.add_function(
						"llvm.pow.f64",
						f64_type.fn_type(&[f64_type.into(), f64_type.into()], false),
						None,
					);
					Ok(FloatValue(
						self.builder
							.build_call(pow_fun, &[lhs.into(), rhs.into()], "tmppow")
							.try_as_basic_value()
							.left()
							.unwrap()
							.into_float_value(),
					))
				}
				Modulo => Ok(FloatValue(self.builder.build_float_rem(lhs, rhs, "tmprem"))),
				Equal => Ok(BoolValue(
					self.builder.build_float_compare(OEQ, lhs, rhs, "Equal"),
				)),
				NotEqual => Ok(BoolValue(
					self.builder.build_float_compare(ONE, lhs, rhs, "NotEqual"),
				)),
				GreaterOrEqual => Ok(BoolValue(self.builder.build_float_compare(
					OGE,
					lhs,
					rhs,
					"GreaterOrEqual",
				))),
				LesserOrEqual => Ok(BoolValue(self.builder.build_float_compare(
					OLE,
					lhs,
					rhs,
					"LesserOrEqual",
				))),
				Greater => Ok(BoolValue(
					self.builder.build_float_compare(OGT, lhs, rhs, "Greater"),
				)),
				Lesser => Ok(BoolValue(
					self.builder.build_float_compare(OLT, lhs, rhs, "Lesser"),
				)),
				_ => Err(Error::new_compiler_error("Unexpected token".to_string())),
			}
		} else if let (BoolValue(lhs), BoolValue(rhs)) = operations {
			match operator.r#type {
				TokenType::And => Ok(BoolValue(self.builder.build_and(lhs, rhs, "And"))),
				TokenType::Or => Ok(BoolValue(self.builder.build_or(lhs, rhs, "Or"))),
				_ => Err(Error::new_compiler_error("Unexpected token".to_string())),
			}
		} else {
			Err(Error::new_compiler_error("Unexpected token".to_string()))
		}
	}

	pub fn evaluate_expression(&self, expression: Expression) -> Result<Operation, Error> {
		use Expression::*;
		use Operation::*;
		use self::Literal::*;

		match expression {
			Literal(token) => match token.literal {
				Some(Number(number)) => {
					let r#type = self.context.f64_type();
					let literal = r#type.const_float(f64::from(number));

					Ok(FloatValue(literal))
				}
				Some(Boolean(bool)) => {
					let r#type = self.context.bool_type();
					let literal = if bool {
						r#type.const_all_ones()
					} else {
						r#type.const_zero()
					};

					Ok(BoolValue(literal))
				}
				_ => Err(Error::new_compiler_error("Unsupported literal".to_string())),
			},
			Identifier(identifier) => match identifier.literal {
				Some(String(name)) => match self.variables.get(&name) {
					Some(pointer) => {
						let value = self.builder.build_load(*pointer, &name);

						match value {
							BasicValueEnum::FloatValue(value) => Ok(FloatValue(value)),
							BasicValueEnum::IntValue(value) => {
								if value.get_type().get_bit_width() == 1 {
									Ok(Operation::BoolValue(value))
								} else {
									Err(Error::new_compiler_error(
										"Unsupported bit width".to_string(),
									))
								}
							}
							_ => Err(Error::new_compiler_error(
								"Unsupported type for operation".to_string(),
							)),
						}
					}
					None => Err(Error::new_compiler_error(format!(
						"Unknown variable {}",
						name
					))),
				},
				_ => Err(Error::new_compiler_error(
					"Unsupported identifier".to_string(),
				)),
			},
			Assignment(identifier, operator, expression) => match identifier.literal {
				Some(String(name)) => {
					let value = if operator.r#type == TokenType::Assign {
						self.evaluate_expression(*expression)?
					} else {
						return Err(Error::new_compiler_error(
							"Unsupported assignment operator".to_string(),
						));
					};
					let pointer = self.variables.get(&name).unwrap();

					match value {
						FloatValue(value) => {
							self.builder.build_store(*pointer, value);
							Ok(NoOp)
						}
						BoolValue(value) => {
							self.builder.build_store(*pointer, value);
							Ok(NoOp)
						}
						_ => Err(Error::new_compiler_error(
							"Unsupported assignment operation".to_string(),
						)),
					}
				}
				_ => Err(Error::new_compiler_error(
					"Unsupported identifier".to_string(),
				)),
			},
			Unary(_, expression) => {
				let value = self.evaluate_expression(*expression)?;
				match value {
					BoolValue(value) => {
						Ok(BoolValue(self.builder.build_not(value, "not")))
					}
					_ => Err(Error::new_compiler_error(
						"Unsupported type for operation".to_string(),
					)),
				}
			}
			Binary(left, operator, right) => {
				self.evaluate_operator(*left, *right, operator)
			}
			Grouping(expression) => self.evaluate_expression(*expression),
			Call(function, _, parameters, _) => {
				let name = match *function {
					Expression::Identifier(identifier) => match identifier.literal {
						Some(String(name)) => name,
						_ => return Err(Error::new_compiler_error("Unsupported call".to_string())),
					},
					_ => return Err(Error::new_compiler_error("Unsupported call".to_string())),
				};

				let fun = match self.module.get_function(&name) {
					Some(fun) => fun,
					None => {
						return Err(Error::new_compiler_error(format!(
							"Unknown function {}",
							name
						)))
					}
				};
				let mut args: Vec<_> = Vec::new();
				for expression in parameters.into_iter() {
					match self.evaluate_expression(expression)? {
						FloatValue(value) => args.push(value.into()),
						BoolValue(value) => args.push(value.into()),
						_ => {
							return Err(Error::new_compiler_error(
								"Unsupported return operation".to_string(),
							))
						}
					}
				}

				let result = self
					.builder
					.build_call(fun, &args, &name)
					.try_as_basic_value()
					.left()
					.unwrap();

				match result {
					BasicValueEnum::FloatValue(value) => Ok(FloatValue(value)),
					BasicValueEnum::IntValue(value) => {
						if value.get_type().get_bit_width() == 1 {
							Ok(BoolValue(value))
						} else {
							Err(Error::new_compiler_error(
								"Unsupported bit width".to_string(),
							))
						}
					}
					_ => Err(Error::new_compiler_error("Unsupported result".to_string())),
				}
			}
			_ => Err(Error::new_compiler_error(
				"Unsupported expression".to_string(),
			)),
		}
	}
}
