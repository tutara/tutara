use crate::compiler::*;
use crate::operation::*;
use crate::scope::*;
use inkwell::{types::BasicTypeEnum, values::BasicValue};
use tutara_interpreter::{Error, Expression, Literal, Statement, Token, TokenType};

impl Compiler<'_> {
	pub fn evaluate_function(
		&mut self,
		identifier: Token,
		r#type: Option<Token>,
		parameters: Vec<(Token, Token)>,
		body: Box<Statement>,
	) -> Result<Operation, Error> {
		// Get parameter types
		let mut params: Vec<BasicTypeEnum> = Vec::new();

		for parameter in parameters.iter() {
			let r#type: BasicTypeEnum = match &parameter.1.literal {
				Some(Literal::String(literal)) => match literal.as_str() {
					"Int" => self.context.f64_type().into(),
					"Bool" => self.context.bool_type().into(),
					_ => {
						return Err(Error::new_compiler_error(
							"Invalid token/literal".to_string(),
						))
					}
				},
				_ => {
					return Err(Error::new_compiler_error(
						"Invalid token/literal".to_string(),
					))
				}
			};

			params.push(r#type)
		}

		// Get function return type
		let fun_type = match r#type {
			None => self.context.void_type().fn_type(&params, false),
			Some(token) => match token.r#type {
				TokenType::Identifier => match token.literal {
					Some(Literal::String(literal)) => match literal.as_str() {
						"Int" => self.context.f64_type().fn_type(&params, false),
						"Bool" => self.context.bool_type().fn_type(&params, false),
						_ => {
							return Err(Error::new_compiler_error(
								"Unknown return type".to_string(),
							))
						}
					},
					_ => return Err(Error::new_compiler_error("Unexpected literal".to_string())),
				},
				_ => return Err(Error::new_compiler_error("Unexpected token".to_string())),
			},
		};

		// Get function name
		let fun_name = match identifier.literal {
			Some(Literal::String(str)) => str,
			_ => {
				return Err(Error::new_compiler_error(
					"Invalid token/literal".to_string(),
				))
			}
		};

		// Create function
		let fun = self.module.add_function(fun_name.as_str(), fun_type, None);
		let body_block = self
			.context
			.append_basic_block(fun, format!("{}_entry", fun_name).as_str());

		self.scope.push(Scope::new(ScopeContext::Fun));
		let current = self.builder.get_insert_block();
		self.builder.position_at_end(body_block);

		// Set parameters in function body
		for (i, parameter) in fun.get_param_iter().enumerate() {
			let parameter_name = match &parameters[i].0.literal {
				Some(Literal::String(str)) => str,
				_ => {
					return Err(Error::new_compiler_error(
						"Invalid token/literal".to_string(),
					))
				}
			};

			parameter.set_name(parameter_name.as_str());

			let alloca = self
				.builder
				.build_alloca(parameter.get_type(), &parameter_name);
			self.builder.build_store(alloca, parameter);
			let scope_index = self.scope.len() - 1;
			self.scope[scope_index]
				.variables
				.insert(parameter_name.to_string(), alloca);
		}

		self.evaluate_statement(*body)?;
		self.scope.pop();
		self.builder.position_at_end(current.unwrap());

		Ok(Operation::NoOp)
	}

	pub fn evaluate_return(&self, right: Option<Expression>) -> Result<Operation, Error> {
		use Operation::*;

		let len = self.scope.len();
		for index in 0..len {
			if let ScopeContext::Fun = self.scope[len - index - 1].scope_context {
				if let Some(expression) = right {
					match self.evaluate_expression(expression) {
						Ok(FloatValue(result)) => self.builder.build_return(Some(&result)),
						Ok(BoolValue(result)) => self.builder.build_return(Some(&result)),
						Err(err) => return Err(err),
						_ => {
							return Err(Error::new_compiler_error(
								"Unsupported return operation".to_string(),
							))
						}
					};
				} else {
					self.builder.build_return(None);
				}

				return Ok(Operation::NoOp);
			}
		}

		// Return on top-level program - should be removed when top-level program statement is added.
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
}
