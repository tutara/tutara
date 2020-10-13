use crate::Error;
use crate::Expression;
use crate::Result;
use crate::Statement;
use crate::Token;
use crate::TokenType;
use crate::Tokenizer;

use core::iter::Peekable;

pub struct Parser<'a> {
	tokenizer: Peekable<Tokenizer<'a>>,
}

impl<'a> Parser<'_> {
	pub fn new(tokenizer: Peekable<Tokenizer<'a>>) -> Parser<'a> {
		Parser { tokenizer }
	}
}

impl Iterator for Parser<'_> {
	type Item = Result<Statement>;

	fn next(&mut self) -> Option<Self::Item> {
		if let Some(Ok(_current)) = self.tokenizer.peek() {
			return Some(self.statement());
		}

		None
	}
}

// Error creation
impl Parser<'_> {
	fn create_expression_syntax_error(
		&mut self,
		message: String,
		token: Token,
	) -> Result<Expression> {
		Err(Error::new_parser_error(message, token))
	}

	fn create_statement_syntax_error(
		&mut self,
		message: String,
		token: Token,
	) -> Result<Statement> {
		Err(Error::new_parser_error(message, token))
	}
}

// Statement parsing
impl Parser<'_> {
	fn statement(&mut self) -> Result<Statement> {
		if let Some(token) = self.next_if_in_token_types(&[
			TokenType::Var,
			TokenType::Val,
			TokenType::Comment,
			TokenType::Function,
			TokenType::Return,
		]) {
			if let Ok(token) = token {
				match token.r#type {
					TokenType::Val | TokenType::Var => self.declaration(token),
					TokenType::Comment => Ok(Statement::Comment(token)),
					TokenType::Function => self.function(token),
					TokenType::Return => self.r#return(token),
					_ => self.create_statement_syntax_error(
						"statement not implemented please report issue".to_string(),
						token,
					),
				}
			} else {
				self.create_statement_syntax_error(
					"Invalid token found".to_string(),
					token.unwrap(),
				)
			}
		} else {
			self.expression()
		}
	}

	fn expression(&mut self) -> Result<Statement> {
		if let Ok(expression) = self.expression_root() {
			return Ok(Statement::Expression(expression));
		}
		let token = self.tokenizer.next().unwrap().unwrap();
		self.create_statement_syntax_error("Invalid expression".to_string(), token)
	}

	fn declaration(&mut self, token: Token) -> Result<Statement> {
		let mut type_specification: Option<Box<Statement>> = None;

		if let Some(Ok(specifier)) = self.next_if_in_token_types(&[TokenType::Specifier]) {
			if let Some(Ok(r#type)) = self.next_if_in_token_types(&[TokenType::Identifier]) {
				type_specification =
					Some(Box::new(Statement::TypeSpecification(specifier, r#type)));
			} else {
				return self.create_statement_syntax_error("Expected type".to_string(), specifier);
			}
		}

		if self.peek_in_token_types(&[TokenType::Identifier]) {
			let statement = self.expression_root();

			if let Ok(expression) = statement {
				Ok(Statement::Declaration(
					token,
					type_specification,
					expression,
				))
			} else {
				self.create_statement_syntax_error("Invalid expression".to_string(), token)
			}
		} else {
			self.create_statement_syntax_error("Expected variable name".to_string(), token)
		}
	}

	fn function(&mut self, token: Token) -> Result<Statement> {
		let mut type_specification: Option<Box<Statement>> = None;

		if let Some(Ok(specifier)) = self.next_if_in_token_types(&[TokenType::Specifier]) {
			if let Some(Ok(r#type)) = self.next_if_in_token_types(&[TokenType::Identifier]) {
				type_specification =
					Some(Box::new(Statement::TypeSpecification(specifier, r#type)));
			} else {
				return self.create_statement_syntax_error("Expected type".to_string(), specifier);
			}
		}

		if let Some(Ok(identifier)) = self.next_if_in_token_types(&[TokenType::Identifier]) {
			let mut parameters_statement: Option<Box<Statement>> = None;

			if let Some(Ok(open_parenthesis)) =
				self.next_if_in_token_types(&[TokenType::OpenParenthesis])
			{
				let mut parameters: Vec<Statement> = Vec::new();
				while let Some(Ok(token)) = self.tokenizer.peek() {
					if token.r#type == TokenType::CloseParenthesis {
						break;
					} else {
						match self.parameter() {
							Ok(parameter) => parameters.push(parameter),
							Err(error) => return Err(error),
						}
					}
				}

				if let Some(Ok(close_parenthesis)) = self.tokenizer.next() {
					parameters_statement = Some(Box::new(Statement::Parameters(
						open_parenthesis,
						parameters,
						close_parenthesis,
					)));
				} else {
					return self.create_statement_syntax_error(
						"Expected closing parenthesis".to_string(),
						open_parenthesis,
					);
				}
			}

			if let Some(Ok(open_curly_bracket)) =
				self.next_if_in_token_types(&[TokenType::OpenCurlyBracket])
			{
				match self.body(open_curly_bracket) {
					Ok(body) => Ok(Statement::Function(
						token,
						type_specification,
						identifier,
						parameters_statement,
						Box::new(body),
					)),
					Err(error) => Err(error),
				}
			} else {
				self.create_statement_syntax_error("Expected function body".to_string(), identifier)
			}
		} else {
			self.create_statement_syntax_error("Expected identifier".to_string(), token)
		}
	}

	fn body(&mut self, open_curly_bracket: Token) -> Result<Statement> {
		let mut statements: Vec<Statement> = Vec::new();

		while let Some(Ok(token)) = self.tokenizer.peek() {
			if token.r#type == TokenType::CloseCurlyBracket {
				return Ok(Statement::Body(
					open_curly_bracket,
					statements,
					self.tokenizer.next().unwrap().unwrap(),
				));
			} else {
				statements.push(self.statement()?);
			}
		}

		self.create_statement_syntax_error(
			"Expected end of function".to_string(),
			open_curly_bracket,
		)
	}

	fn parameter(&mut self) -> Result<Statement> {
		if let Some(Ok(identifier)) = self.next_if_in_token_types(&[TokenType::Identifier]) {
			if let Some(Ok(specifier)) = self.next_if_in_token_types(&[TokenType::Specifier]) {
				if let Some(Ok(r#type)) = self.next_if_in_token_types(&[TokenType::Identifier]) {
					if let Some(Ok(seperator)) =
						self.next_if_in_token_types(&[TokenType::Separator])
					{
						Ok(Statement::Parameter(
							identifier,
							Box::new(Statement::TypeSpecification(specifier, r#type)),
							Some(seperator),
						))
					} else if self.peek_in_token_types(&[TokenType::CloseParenthesis]) {
						Ok(Statement::Parameter(
							identifier,
							Box::new(Statement::TypeSpecification(specifier, r#type)),
							None,
						))
					} else {
						self.create_statement_syntax_error(
							"Expected seperator".to_string(),
							specifier,
						)
					}
				} else {
					self.create_statement_syntax_error("Expected type".to_string(), specifier)
				}
			} else {
				self.create_statement_syntax_error("Expected specifier".to_string(), identifier)
			}
		} else {
			let token = self.tokenizer.next().unwrap().unwrap();
			self.create_statement_syntax_error("Expected identifier".to_string(), token)
		}
	}

	fn r#return(&mut self, token: Token) -> Result<Statement> {
		Ok(Statement::Return(token, self.expression_root().ok()))
	}
}

// Expression parsing
impl Parser<'_> {
	fn expression_root(&mut self) -> Result<Expression> {
		self.assignment()
	}

	fn assignment(&mut self) -> Result<Expression> {
		let expression: Expression = self.addition_and_subtraction()?;

		if let Some(Ok(token)) = self.next_if_in_token_types(&[
			TokenType::Assign,
			TokenType::AssignPlus,
			TokenType::AssignMinus,
			TokenType::AssignMultiply,
			TokenType::AssignDivision,
			TokenType::AssignPow,
			TokenType::AssignModulo,
		]) {
			return match expression {
				Expression::Identifier(_token) => Ok(Expression::Assignment(
					_token,
					token,
					Box::new(self.assignment()?),
				)),
				_ => self.create_expression_syntax_error("Failed on assignment".to_string(), token),
			};
		}

		Ok(expression)
	}

	fn addition_and_subtraction(&mut self) -> Result<Expression> {
		let mut expression = self.multiplication_division_modulo()?;

		while let Some(Ok(token)) =
			self.next_if_in_token_types(&[TokenType::Minus, TokenType::Plus])
		{
			expression = Expression::Binary(
				Box::new(expression),
				token,
				Box::new(self.multiplication_division_modulo()?),
			);
		}
		Ok(expression)
	}

	fn multiplication_division_modulo(&mut self) -> Result<Expression> {
		let mut expression = self.involution()?;

		while let Some(Ok(token)) = self.next_if_in_token_types(&[
			TokenType::Multiply,
			TokenType::Division,
			TokenType::Modulo,
		]) {
			expression =
				Expression::Binary(Box::new(expression), token, Box::new(self.involution()?));
		}
		Ok(expression)
	}

	fn involution(&mut self) -> Result<Expression> {
		let mut expression = self.unary()?;

		while let Some(Ok(token)) = self.next_if_in_token_types(&[TokenType::Pow]) {
			expression = Expression::Binary(Box::new(expression), token, Box::new(self.unary()?));
		}
		Ok(expression)
	}

	fn unary(&mut self) -> Result<Expression> {
		if let Some(Ok(token)) = self.next_if_in_token_types(&[TokenType::Minus]) {
			return Ok(Expression::Unary(token, Box::new(self.unary()?)));
		}

		self.terms()
	}

	fn terms(&mut self) -> Result<Expression> {
		if let Some(Ok(token)) = self.next_if_in_token_types(&[TokenType::Identifier]) {
			return Ok(Expression::Identifier(token));
		}

		if let Some(Ok(token)) = self.next_if_in_token_types(&[
			TokenType::String,
			TokenType::Integer,
			TokenType::True,
			TokenType::False,
		]) {
			return Ok(Expression::Literal(token));
		}

		if let Some(Ok(_token)) = self.next_if_in_token_types(&[TokenType::OpenParenthesis]) {
			let expression = self.assignment()?;

			if let Some(Ok(_next)) = self.next_if_in_token_types(&[TokenType::CloseParenthesis]) {
				return Ok(Expression::Grouping(Box::new(expression)));
			}
		}

		let token = self.tokenizer.next().unwrap().unwrap();
		self.create_expression_syntax_error("Unexpected token".to_string(), token)
	}
}

// Helper functions for iterating trough tokens
impl Parser<'_> {
	fn peek_in_token_types(&mut self, types: &[TokenType]) -> bool {
		match self.tokenizer.peek() {
			Some(&Ok(ref token)) => types.contains(&token.r#type),
			_ => false,
		}
	}

	fn next_if_in_token_types(&mut self, types: &[TokenType]) -> Option<Result<Token>> {
		if self.peek_in_token_types(types) {
			self.tokenizer.next()
		} else {
			None
		}
	}
}
