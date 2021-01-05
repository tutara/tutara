use crate::compiler::Compiler;
use crate::operation::Operation;
use inkwell::{FloatPredicate, IntPredicate};
use tutara_interpreter::{Error, Expression, Token, TokenType};

impl Compiler<'_> {
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
				Equal => Ok(BoolValue(self.builder.build_int_compare(
					IntPredicate::EQ,
					lhs,
					rhs,
					"Equal",
				))),
				NotEqual => Ok(BoolValue(self.builder.build_int_compare(
					IntPredicate::NE,
					lhs,
					rhs,
					"NotEqual",
				))),
				_ => Err(Error::new_compiler_error("Unexpected token".to_string())),
			}
		} else {
			Err(Error::new_compiler_error("Unexpected token".to_string()))
		}
	}
}
