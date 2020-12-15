use crate::Error;
use crate::Expression;
use crate::Result;
use crate::Statement;
use crate::Token;
use crate::TokenType;

pub struct Analyzer {}

impl Analyzer {
	pub fn new() -> Analyzer {
		Analyzer {}
	}
}

impl Default for Analyzer {
    fn default() -> Analyzer {
        Analyzer::new()
    }
}

impl Analyzer {
	pub fn analyze(&mut self, statement: Statement) -> Result<Statement> {
		use Statement::*;
		
		match statement {
			Expression(_) => self.analyze_statement(&statement),
			Loop(_) => self.analyze_statement(&statement),
			_ => Ok(statement),
		}
	}

	pub fn analyze_statement(&mut self, statement: &Statement) -> Result<Statement> {
		use self::Expression::*;
		use crate::Literal::*;
		use Statement::*;

		match statement {
			Expression(expression) => {
				Ok(Statement::Expression(self.analyze_expression(expression)?))
			}
			Loop(statement) => Ok(While(
				Literal(Token::new(TokenType::Boolean, Some(Boolean(true)), 0, 0, 0)),
				Box::new(self.analyze_statement(&**statement)?),
			)),
			_ => unreachable!(),
		}
	}

	fn analyze_expression(&mut self, expression: &Expression) -> Result<Expression> {
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
				_ => Ok(Assignment(
					identifier.clone(),
					assignment.clone(),
					expression.clone(),
				)),
			},
			_ => Ok(expression.clone()),
		}
	}
}

impl Analyzer {
	fn operation_assignment(
		&mut self,
		identifier: &Token,
		assignment: &Token,
		expression: &Expression,
		token_type: TokenType,
	) -> Result<Expression> {
		use crate::Literal::*;
		use Expression::*;

		match &identifier.literal {
			Some(String(name)) => Ok(Assignment(
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
						Some(String(name.clone())),
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
					Box::new(expression.clone()),
				)),
			)),
			_ => Err(Error::new_compiler_error(
				"Unsupported identifier".to_string(),
			)),
		}
	}
}
