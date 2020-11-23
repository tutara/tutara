use std::collections::HashMap;

use inkwell::{builder::Builder, context::Context, module::Module, values::FloatValue, values::InstructionValue, values::{BasicValueEnum, FunctionValue, PointerValue}};

use tutara_interpreter::{Error, Expression, Literal, Parser, Statement, Token, TokenType};

pub struct Compiler<'a> {
	pub(super) context: &'a Context,
	pub(super) module: Module<'a>,
	pub(super) builder: Builder<'a>,
	
	pub(super) variables: HashMap<String, PointerValue<'a>>,
}
pub enum Operation<'a> {
	FloatValue(FloatValue<'a>),
	Return(InstructionValue<'a>),
	NoOp(),
}

impl Compiler<'_> {
	pub fn compile<'b>(&mut self, parser: Parser<'b>) -> Result<FunctionValue, Error> {
		let fun_type = self.context.f64_type().fn_type(&[], false);
		let fun = self.module.add_function("main", fun_type, None);
		let body = self.context.append_basic_block(fun, "entry");
		self.builder.position_at_end(body);

		for result in parser {
			match result {
				Ok(statement) => if let Operation::Return(_) = self.evaluate_statement(statement)? {
					match self.module.verify() {
						Ok(_) => return Ok(fun),
						Err(err) => return Err(Error::new_compiler_error(err.to_string())),
					}
					
				},
				Err(error) => return Err(error),
			};
		}

		Err(Error::new_compiler_error("No return statement found in script".to_string()))
	}

	pub fn evaluate_statement(&mut self, statement: Statement) -> Result<Operation, Error> {
		match statement {
			Statement::Expression(expression) => self.evaluate_expression(expression),
			Statement::Declaration(_mutability, _type_specification, expression) => {
				self.evaluate_declaration(expression)?;
				Ok(Operation::NoOp())
			},
			Statement::Return(expression) => self.evaluate_return(expression),
			Statement::Comment(_) => Ok(Operation::NoOp()),
			_ => Err(Error::new_compiler_error("Unsupported statement".to_string())),
		}
	}

	pub fn evaluate_return(&self, right: Option<Expression>) -> Result<Operation, Error> {
		match right {
		    Some(expression) => match self.evaluate_expression(expression) {
		        Ok(Operation::FloatValue(result)) => Ok(Operation::Return(self.builder.build_return(Some(&result)))),
				Err(err) => Err(err),
				_ => Err(Error::new_compiler_error("Unsupported return operation".to_string()))
		    },
		    None => Ok(Operation::Return(self.builder.build_return(None)))
		}
	}

	pub fn evaluate_declaration(&mut self, expression: Expression) -> Result<Operation, Error> {
		match expression {
			Expression::Assignment(identifier, _operator, inner_expression) => {
				match identifier.literal {
					Some(Literal::String(name)) => {
						let pointer = self.builder.build_alloca(self.context.f64_type(), &name);
						self.variables.insert(name.to_string(), pointer);

						match self.evaluate_expression(*inner_expression)? {
							Operation::FloatValue(value) => {
								self.builder.build_store(pointer, value);
								Ok(Operation::NoOp())
							},
							_ => Err(Error::new_compiler_error("Unsupported assignment operation".to_string())),
						}
					},
					_ => Err(Error::new_compiler_error("Unsupported expression".to_string())),
				}
			}
			_ => Err(Error::new_compiler_error("Unsupported expression".to_string())),
		}
	}

	pub fn evaluate_operator(&self, left: Expression, right: Expression, operator: Token) -> Result<Operation, Error> {
		let lhs = match self.evaluate_expression(left) {
			Ok(Operation::FloatValue(value)) => value,
			Err(err) => return Err(err),
			_ => return Err(Error::new_compiler_error("Unsupported left hand expression".to_string()))
		};
		let rhs = match self.evaluate_expression(right) {
			Ok(Operation::FloatValue(value)) => value,
			Err(err) => return Err(err),
			_ => return Err(Error::new_compiler_error("Unsupported right hand expression".to_string()))
		};

		match operator.r#type {
			TokenType::Plus => Ok(Operation::FloatValue(self.builder.build_float_add(lhs, rhs, "tmpadd"))),
			TokenType::Minus => Ok(Operation::FloatValue(self.builder.build_float_sub(lhs, rhs, "tmpsub"))),
			TokenType::Multiply => Ok(Operation::FloatValue(self.builder.build_float_mul(lhs, rhs, "tmpmul"))),
			TokenType::Division => Ok(Operation::FloatValue(self.builder.build_float_div(lhs, rhs, "tmpdiv"))),
			TokenType::Exponentiation => {
				let f64_type = self.context.f64_type();
				let pow_fun = self.module.add_function(
					"llvm.pow.f64",
					f64_type.fn_type(&[f64_type.into(), f64_type.into()], false),
					None,
				);

				Ok(Operation::FloatValue(
					self.builder
						.build_call(pow_fun, &[lhs.into(), rhs.into()], "tmppow")
						.try_as_basic_value()
						.left()
						.unwrap()
						.into_float_value(),
				))
			}
			TokenType::Modulo => Ok(Operation::FloatValue(self.builder.build_float_rem(lhs, rhs, "tmprem"))),
			_ => Err(Error::new_compiler_error("Unexpected token".to_string())),
		}
	}

	pub fn evaluate_expression(&self, expression: Expression) -> Result<Operation, Error> {
		match expression {
			Expression::Literal(token) => match token.literal {
				Some(Literal::Number(number)) => {
					let r#type = self.context.f64_type();
					let literal = r#type.const_float(f64::from(number));

					Ok(Operation::FloatValue(literal))
				}
				_ => Err(Error::new_compiler_error("Unsupported literal".to_string())),
			},
			Expression::Identifier(identifier) => match identifier.literal {
				Some(Literal::String(name)) => {
					match self.variables.get(&name) {
						Some(pointer) => {
							let value = self.builder.build_load(*pointer, &name);

							Ok(Operation::FloatValue(value.into_float_value()))
						},
						None => Err(Error::new_compiler_error(format!("Unknown variable {}", name))),
					}
				},
				_ => Err(Error::new_compiler_error("Unsupported identifier".to_string())),
			}
			Expression::Assignment(identifier, operator, expression) => {
				match identifier.literal {
					Some(Literal::String(name)) => {
						let value = if operator.r#type == TokenType::Assign {
							self.evaluate_expression(*expression)?
						} else {
							let expr = Expression::Identifier(Token::new(TokenType::Identifier, Some(Literal::String(name.clone())), identifier.line, identifier.column, identifier.length));
							self.evaluate_operator(expr, *expression, match operator.r#type {
								TokenType::AssignPlus => Token::new(TokenType::Plus, None, operator.line, operator.column, operator.length),
								TokenType::AssignMinus => Token::new(TokenType::Minus, None, operator.line, operator.column, operator.length),
								TokenType::AssignMultiply => Token::new(TokenType::Multiply, None, operator.line, operator.column, operator.length),
								TokenType::AssignDivision => Token::new(TokenType::Division, None, operator.line, operator.column, operator.length),
								TokenType::AssignExponentiation => Token::new(TokenType::Exponentiation, None, operator.line, operator.column, operator.length),
								TokenType::AssignModulo => Token::new(TokenType::Modulo, None, operator.line, operator.column, operator.length),
								_ => return Err(Error::new_compiler_error("Unsupported assignment operator".to_string())),
							})?
						};
						let pointer = self.variables.get(&name).unwrap();
						
						match value {
							Operation::FloatValue(value) => {
								self.builder.build_store(*pointer, value);
								Ok(Operation::NoOp())
							},
							_ => Err(Error::new_compiler_error("Unsupported assignment operation".to_string())),
						}
					},
					_ => Err(Error::new_compiler_error("Unsupported identifier".to_string())),
				}
			},
			Expression::Binary(left, operator, right) => self.evaluate_operator(*left, *right, operator),
			Expression::Call(function, _, parameters, _) => {
				let name = match *function {
					Expression::Identifier(identifier) => match identifier.literal {
						Some(Literal::String(name)) => name,
						_ => return Err(Error::new_compiler_error("Unsupported call".to_string()))
					},
					_ => return Err(Error::new_compiler_error("Unsupported call".to_string()))
				};

				let fun = match self.module.get_function(&name) {
					Some(fun) => fun,
					None => return Err(Error::new_compiler_error(format!("Unknown function {}", name))),
				};
				let mut args: Vec<_> = Vec::new();
				for expression in parameters.into_iter() {
					match self.evaluate_expression(expression)? {
					    Operation::FloatValue(value) => args.push(value.into()),
						_ => return Err(Error::new_compiler_error("Unsupported return operation".to_string()))
					}
				}
				
				let result = self.builder.build_call(fun, &args, &name)
					.try_as_basic_value()
					.left()
					.unwrap();
				
				match result {
					BasicValueEnum::FloatValue(value) => Ok(Operation::FloatValue(value)),
					_ => Err(Error::new_compiler_error("Unsupported result".to_string())),
				}
			},
			_ => Err(Error::new_compiler_error("Unsupported expression".to_string())),
		}
	}
}
