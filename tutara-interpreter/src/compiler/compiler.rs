use inkwell::{
	builder::Builder, context::Context, module::Module, values::FloatValue, values::FunctionValue,
};

use crate::{Error, Expression, Literal, Parser, Statement};

pub struct Compiler<'a> {
	pub(super) context: &'a Context,
	pub(super) module: Module<'a>,
	pub(super) builder: Builder<'a>,
}

impl Compiler<'_> {
	pub fn compile<'b>(&self, parser: Parser<'b>) -> Result<FunctionValue, Error> {
		let fun_type = self.context.f64_type().fn_type(&[], false);
		let fun = self.module.add_function("entry", fun_type, None);
		let body = self.context.append_basic_block(fun, "body");
		self.builder.position_at_end(body);

		for result in parser {
			match result {
				Ok(statement) => {
					match self.evaluate_statement(statement) {
						Ok(value) => self.builder.build_return(Some(&value)),
						Err(err) => return Err(err),
					}
					
				}
				Err(error) => return Err(error),
			};
		}

		return Ok(fun);
	}

	pub fn evaluate_statement(&self, statement: Statement) -> Result<FloatValue, Error> {
		match statement {
			Statement::Expression(expression) => self.evaluate_expression(expression),
			_ => Err(Error::new_compiler_error("Unsupported statement".to_string())),
		}
	}

	pub fn evaluate_expression(&self, expression: Expression) -> Result<FloatValue, Error> {
		match expression {
			Expression::Literal(token) => match token.literal {
				Some(Literal::Number(number)) => {
					let r#type = self.context.f64_type();
					let literal = r#type.const_float(f64::from(number));
					return Ok(literal);
				}
				_ => Err(Error::new_compiler_error("Unsupported literal".to_string())),
			},
			Expression::Binary(left, operator, right) => {
				let lhs = self.evaluate_expression(*left)?;
				let rhs = self.evaluate_expression(*right)?;
				let builder = self.context.create_builder();

				return match operator.r#type {
					crate::TokenType::Plus => Ok(builder.build_float_add(lhs, rhs, "tmpadd")),
					crate::TokenType::Minus => Ok(builder.build_float_sub(lhs, rhs, "tmpsub")),
					crate::TokenType::Multiply => Ok(builder.build_float_mul(lhs, rhs, "tmpmul")),
					crate::TokenType::Division => Ok(builder.build_float_div(lhs, rhs, "tmpdiv")),
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
					crate::TokenType::Modulo => Ok(builder.build_float_rem(lhs, rhs, "tmprem")),
					_ => Err(Error::new_compiler_error("Unexpected token".to_string())),
				};
			}
			_ => Err(Error::new_compiler_error("Unsupported expression".to_string())),
		}
	}
}
