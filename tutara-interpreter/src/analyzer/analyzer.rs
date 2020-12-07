use crate::Error;
use crate::Expression;
use crate::Parser;
use crate::Result;
use crate::Statement;
use crate::Token;
use crate::TokenType;

pub struct Analyzer<'a> {
	parser: Parser<'a>,
}

impl<'a> Analyzer<'_> {
	pub fn new(parser: Parser<'a>) -> Analyzer<'a> {
		Analyzer { parser }
	}
}

impl Iterator for Analyzer<'_> {
	type Item = Result<Statement>;

	fn next(&mut self) -> Option<Self::Item> {
		match self.parser.next() {
			Some(Ok(current)) => Some(self.analyze(current)),
			Some(Err(err)) => Some(Err(err)),
			None => None,
		}
	}
}

impl Analyzer<'_> {
	fn analyze(&mut self, statement: Statement) -> Result<Statement> {
		use crate::Literal::*;

		match statement {
			Statement::Expression(expression) => self.expression(expression),
			Statement::Loop(statement) => Ok(Statement::While(
				Expression::Literal(Token::new(
					TokenType::Boolean,
					Some(Boolean(true)),
					0,
					0,
					0,
				)),
				statement,
			)),
			_ => Ok(statement),
		}
	}
}

impl Analyzer<'_> {
	fn expression(&mut self, expression: Expression) -> Result<Statement> {
		use Expression::*;
		use TokenType::*;

		match expression {
			Assignment(identifier, assignment, expression) => match &assignment.r#type {
				AssignPlus => self.operation_assignment(identifier, assignment, expression, Plus),
				AssignMinus => self.operation_assignment(identifier, assignment, expression, Minus),
				AssignMultiply => {
					self.operation_assignment(identifier, assignment, expression, Multiply)
				}
				AssignDivision => {
					self.operation_assignment(identifier, assignment, expression, Division)
				}
				AssignExponentiation => {
					self.operation_assignment(identifier, assignment, expression, Exponentiation)
				}
				AssignModulo => {
					self.operation_assignment(identifier, assignment, expression, Modulo)
				}
				_ => Ok(Statement::Expression(Assignment(
					identifier, assignment, expression,
				))),
			},
			_ => Ok(Statement::Expression(expression)),
		}
	}

	fn operation_assignment(
		&mut self,
		identifier: Token,
		assignment: Token,
		expression: Box<Expression>,
		token_type: TokenType,
	) -> Result<Statement> {
		use crate::Literal::*;
		use Expression::*;

		match identifier.literal {
			Some(String(name)) => {
				Ok(Statement::Expression(Assignment(
					Token::new(
						TokenType::Identifier,
						Some(String(name.clone())),
						identifier.line,
						identifier.column,
						identifier.length,
					),
					Token::new(
						TokenType::Assign,
						None,
						assignment.line,
						assignment.column,
						assignment.length,
					),
					Box::new(Binary(
						Box::new(Identifier(Token::new(
							TokenType::Identifier,
							Some(String(name)),
							identifier.line,
							identifier.column,
							identifier.length,
						))),
						Token::new(
							token_type,
							None,
							assignment.line,
							assignment.column,
							assignment.length,
						),
						expression,
					)),
				)))
			}
			_ => Err(Error::new_compiler_error(
				"Unsupported identifier".to_string(),
			)),
		}
	}
}
