use std::collections::HashMap;

use inkwell::{builder::Builder, context::Context, values::InstructionValue, module::Module, values::FloatValue, values::{FunctionValue, PointerValue}};

use crate::{Error, Expression, Literal, Parser, Statement, Token};

pub struct Compiler<'a> {
	pub(super) context: &'a Context,
	pub(super) module: Module<'a>,
	pub(super) builder: Builder<'a>,
	
	pub(super) variables: HashMap<String, PointerValue<'a>>,
}
pub enum Operation<'a> {
	Expression(FloatValue<'a>),
	Declaration(),
	Return(InstructionValue<'a>),
}

impl Compiler<'_> {
	pub fn compile<'b>(&mut self, parser: Parser<'b>) -> Result<FunctionValue, Error> {
		let fun_type = self.context.f64_type().fn_type(&[], false);
		let fun = self.module.add_function("entry", fun_type, None);
		let body = self.context.append_basic_block(fun, "body");
		self.builder.position_at_end(body);

		for result in parser {
			match result {
				Ok(statement) => {
					match self.evaluate_statement(statement) {
						Ok(value) => {},
						Err(err) => return Err(err),
					}
					
				}
				Err(error) => return Err(error),
			};
		}

		return Ok(fun);
	}

	pub fn evaluate_statement(&mut self, statement: Statement) -> Result<Operation, Error> {
		match statement {
			Statement::Expression(expression) => Ok(Operation::Expression(self.evaluate_expression(expression)?)),
			Statement::Declaration(mutability, type_specification, expression) => {
				self.evaluate_declaration(expression)?;
				Ok(Operation::Declaration())
			},
			Statement::Return(_, expression) => Ok(Operation::Return(self.evaluate_return(expression)?)),
			_ => Err(Error::new_compiler_error("Unsupported statement".to_string())),
		}
	}

	pub fn evaluate_return(&self, right: Option<Expression>) -> Result<InstructionValue, Error> {
		match right {
		    Some(expression) => match self.evaluate_expression(expression) {
		        Ok(result) => Ok(self.builder.build_return(Some(&result))),
		        Err(err) => Err(err),
		    },
		    None => Ok(self.builder.build_return(None))
		}
	}

	pub fn evaluate_declaration(&mut self, expression: Expression) -> Result<FloatValue, Error> {
		match expression {
			Expression::Assignment(identifier, operator, inner_expression) => {
				match identifier.literal {
					Some(Literal::String(name)) => {
						let pointer = self.builder.build_alloca(self.context.f64_type(), &name);
						self.variables.insert(name.to_string(), pointer);
						let value = self.evaluate_expression(*inner_expression)?;

						self.builder.build_store(pointer, value);

						Ok(value)
					},
					_ => Err(Error::new_compiler_error("Unsupported expression".to_string())),
				}
			}
			_ => Err(Error::new_compiler_error("Unsupported expression".to_string())),
		}
	}

	pub fn evaluate_operator(&self, left: Expression, right: Expression, operator: Token) -> Result<FloatValue, Error> {
		let lhs = self.evaluate_expression(left)?;
		let rhs = self.evaluate_expression(right)?;

		return match operator.r#type {
			crate::TokenType::Plus => Ok(self.builder.build_float_add(lhs, rhs, "tmpadd")),
			crate::TokenType::Minus => Ok(self.builder.build_float_sub(lhs, rhs, "tmpsub")),
			crate::TokenType::Multiply => Ok(self.builder.build_float_mul(lhs, rhs, "tmpmul")),
			crate::TokenType::Division => Ok(self.builder.build_float_div(lhs, rhs, "tmpdiv")),
			crate::TokenType::Pow => {
				let f64_type = self.context.f64_type();
				let pow_fun = self.module.add_function(
					"llvm.pow.f64",
					f64_type.fn_type(&[f64_type.into(), f64_type.into()], false),
					None,
				);

				Ok(
					self.builder
						.build_call(pow_fun, &[lhs.into(), rhs.into()], "tmppow")
						.try_as_basic_value()
						.left()
						.unwrap()
						.into_float_value(),
				)
			}
			crate::TokenType::Modulo => Ok(self.builder.build_float_rem(lhs, rhs, "tmprem")),
			_ => Err(Error::new_compiler_error("Unexpected token".to_string())),
		};
	}

	pub fn evaluate_expression<'ctx>(&self, expression: Expression) -> Result<FloatValue, Error> {
		match expression {
			Expression::Literal(token) => match token.literal {
				Some(Literal::Number(number)) => {
					let r#type = self.context.f64_type();
					let literal = r#type.const_float(f64::from(number));

					Ok(literal)
				}
				_ => Err(Error::new_compiler_error("Unsupported literal".to_string())),
			},
			Expression::Identifier(identifier) => match identifier.literal {
				Some(Literal::String(name)) => {
					let pointer = self.variables.get(&name).unwrap();
					let value = self.builder.build_load(*pointer, &name);

					Ok(value.into_float_value())
				},
				_ => Err(Error::new_compiler_error("Unsupported identifier".to_string())),
			}
			// Expression::Assignment(identifier, operator, expression) => {
			// 	match identifier.literal {
			// 		Some(Literal::String(name)) => {
			// 			let pointer = self.variables.get(&name).unwrap();
			// 			let value = self.builder.build_load(*pointer, &name);

			// 			let value_new = self.evaluate_operator(value, *expression, operator);

			// 			let rhs = self.evaluate_expression(*expression)?;
			// 			self.builder.build_store(ptr, name)
			// 			let lhz = self.builder.build_extract_value(agg, index, name)
			// 		},
			// 		_ => Err(Error::new_compiler_error("Unsupported identifier".to_string())),
			// 	}
			// },
			Expression::Binary(left, operator, right) => self.evaluate_operator(*left, *right, operator),
			_ => Err(Error::new_compiler_error("Unsupported expression".to_string())),
		}
	}
}
