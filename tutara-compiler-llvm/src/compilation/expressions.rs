use crate::compiler::Compiler;
use crate::operation::Operation;
use inkwell::values::BasicValueEnum;
use tutara_interpreter::{Error, Expression, Literal};

impl Compiler<'_> {
	pub fn evaluate_expression(&self, expression: Expression) -> Result<Operation, Error> {
		use self::Literal::*;
		use Expression::*;
		use Operation::*;

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
				Some(String(name)) => match self.get_variable(&name) {
					Ok(pointer) => {
						let value = self.builder.build_load(pointer, &name);

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
					Err(err) => Err(err),
				},
				_ => Err(Error::new_compiler_error(
					"Unsupported identifier".to_string(),
				)),
			},
			Assignment(identifier, operator, expression) => match identifier.literal {
				Some(String(name)) => self.set_variable(&name, operator, *expression),
				_ => Err(Error::new_compiler_error(
					"Unsupported identifier".to_string(),
				)),
			},
			Unary(_, expression) => {
				let value = self.evaluate_expression(*expression)?;
				match value {
					BoolValue(value) => Ok(BoolValue(self.builder.build_not(value, "not"))),
					FloatValue(value) => Ok(FloatValue(self.builder.build_float_neg(value, "neg"))),
					_ => Err(Error::new_compiler_error(
						"Unsupported type for operation".to_string(),
					)),
				}
			}
			Binary(left, operator, right) => self.evaluate_operator(*left, *right, operator),
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
			Get(_source, _target) => Err(Error::new_compiler_error(
				"Unsupported expression: Get".to_string(),
			)),
		}
	}
}
