use crate::ast::*;
use crate::Result;
use crate::Error;
use crate::ErrorType;
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
		match self.tokenizer.peek() {
			Some(Ok(_current)) => Some(self.statement()),
			Some(Err(_)) => Some(Err(self.tokenizer.next().unwrap().unwrap_err())),
			None => None,
		}
	}
}

// Error creation
impl Parser<'_> {
	fn create_token_syntax_error(&mut self, message: String, token: Token) -> Result<Token> {
		Err(Error::new_parser_error(message, token))
	}

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
			TokenType::Loop,
			TokenType::While,
			TokenType::For,
			TokenType::Break,
			TokenType::Continue,
			TokenType::If,
		]) {
			if let Ok(token) = token {
				match token.r#type {
					TokenType::Val | TokenType::Var => self.declaration(token),
					TokenType::Comment => Ok(Statement::Comment(token)),
					TokenType::Function => self.function(token),
					TokenType::Return => Ok(Statement::Return(self.expression_root().ok())),
					TokenType::Loop => self.r#loop(token),
					TokenType::While => self.r#while(token),
					TokenType::For => self.r#for(token),
					TokenType::Break => Ok(Statement::Break),
					TokenType::Continue => Ok(Statement::Continue),
					TokenType::If => self.r#if(token),
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
		match self.expression_root() {
			Ok(expression) => Ok(Statement::Expression(expression)),
			Err(error) => Err(error),
		}
	}

	fn declaration(&mut self, token: Token) -> Result<Statement> {
		let mut type_specification: Option<Token> = None;

		if let Some(next) = self.next_if_specifier() {
			type_specification = Some(next?)
		}

		if self.peek_token_type(TokenType::Identifier) {
			Ok(Statement::Declaration(
				token,
				type_specification,
				self.expression_root()?,
			))
		} else {
			self.create_statement_syntax_error("Expected variable name".to_string(), token)
		}
	}

	fn function(&mut self, token: Token) -> Result<Statement> {
		let mut type_specification: Option<Token> = None;

		if let Some(next) = self.next_if_specifier() {
			type_specification = Some(next?)
		}

		if let Some(Ok(identifier)) = self.next_if_token_type(TokenType::Identifier) {
			let mut parameters: Vec<(Token, Token)> = Vec::new();

			if let Some(Ok(open_parenthesis)) = self.next_if_token_type(TokenType::OpenParenthesis)
			{
				while let Some(Ok(token)) = self.tokenizer.peek() {
					if token.r#type == TokenType::CloseParenthesis {
						break;
					} else {
						parameters.push(self.parameter()?);
					}
				}

				match self.tokenizer.next() {
					Some(Ok(_close_parenthesis)) => {}
					_ => return self.create_statement_syntax_error(
						"Expected closing parenthesis".to_string(),
						open_parenthesis,
					)
				}
			}

			if let Some(Ok(open_curly_bracket)) =
				self.next_if_token_type(TokenType::OpenCurlyBracket)
			{
				match self.body(open_curly_bracket) {
					Ok(body) => Ok(Statement::Function(
						type_specification,
						identifier,
						parameters,
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
				self.tokenizer.next();
				return Ok(Statement::Body(statements));
			} else {
				match self.next() {
					Some(Ok(next)) => statements.push(next),
					Some(Err(err)) => return Err(err),
					None => {
						return self.create_statement_syntax_error(
							"Expected end of body".to_string(),
							open_curly_bracket,
						)
					}
				}
			}
		}

		self.create_statement_syntax_error("Expected end of body".to_string(), open_curly_bracket)
	}

	fn parameter(&mut self) -> Result<(Token, Token)> {
		if let Some(Ok(identifier)) = self.next_if_token_type(TokenType::Identifier) {
			let type_specification: Token;

			match self.next_if_specifier() {
				Some(next) => type_specification = next?,
				None => {
					return Err(Error::new_parser_error(
						"Expected type specification".to_string(),
						identifier,
					));
				}
			}

			if let Some(Ok(_)) = self.next_if_token_type(TokenType::Separator) {
				Ok((identifier, type_specification))
			} else if self.peek_token_type(TokenType::CloseParenthesis) {
				Ok((identifier, type_specification))
			} else {
				Err(Error::new_parser_error("Expected seperator".to_string(), identifier))
			}
		} else {
			let token = self.tokenizer.next().unwrap().unwrap();
			Err(Error::new_parser_error("Expected identifier".to_string(), token))
		}
	}

	fn r#loop(&mut self, token: Token) -> Result<Statement> {
		if let Some(Ok(open_curly_bracket)) = self.next_if_token_type(TokenType::OpenCurlyBracket) {
			match self.body(open_curly_bracket) {
				Ok(body) => Ok(Statement::Loop(Box::new(body))),
				Err(error) => Err(error),
			}
		} else {
			self.create_statement_syntax_error("Expected loop body".to_string(), token)
		}
	}

	fn r#while(&mut self, token: Token) -> Result<Statement> {
		if let Some(Ok(_)) = self.next_if_token_type(TokenType::OpenParenthesis) {
			let condition = self.expression_root()?;
			if let Some(Ok(_)) = self.next_if_token_type(TokenType::CloseParenthesis) {
				if let Some(Ok(open_curly_bracket)) =
					self.next_if_token_type(TokenType::OpenCurlyBracket)
				{
					match self.body(open_curly_bracket) {
						Ok(body) => Ok(Statement::While(condition, Box::new(body))),
						Err(error) => Err(error),
					}
				} else {
					self.create_statement_syntax_error("Expected loop body".to_string(), token)
				}
			} else {
				self.create_statement_syntax_error("Expected close parenthesis".to_string(), token)
			}
		} else {
			self.create_statement_syntax_error("Expected open parenthesis".to_string(), token)
		}
	}

	fn r#for(&mut self, token: Token) -> Result<Statement> {
		if let Some(Ok(_)) = self.next_if_token_type(TokenType::OpenParenthesis) {
			if let Some(Ok(_)) = self.next_if_token_type(TokenType::In) {
				if let Some(Ok(_)) = self.next_if_token_type(TokenType::CloseParenthesis) {
					if let Some(Ok(open_curly_bracket)) =
						self.next_if_token_type(TokenType::OpenCurlyBracket)
					{
						match self.body(open_curly_bracket) {
							Ok(body) => Ok(Statement::For(
								self.terms()?,
								self.expression_root()?,
								Box::new(body),
							)),
							Err(error) => Err(error),
						}
					} else {
						self.create_statement_syntax_error("Expected loop body".to_string(), token)
					}
				} else {
					self.create_statement_syntax_error(
						"Expected close parenthesis".to_string(),
						token,
					)
				}
			} else {
				self.create_statement_syntax_error("Expected in".to_string(), token)
			}
		} else {
			self.create_statement_syntax_error("Expected open parenthesis".to_string(), token)
		}
	}

	fn r#if(&mut self, token: Token) -> Result<Statement> {
		if let Some(Ok(_)) = self.next_if_token_type(TokenType::OpenParenthesis) {
			let expression = self.expression_root()?;

			if let Some(Ok(_)) = self.next_if_token_type(TokenType::CloseParenthesis) {
				if let Some(Ok(open_curly_bracket)) =
					self.next_if_token_type(TokenType::OpenCurlyBracket)
				{
					let body = self.body(open_curly_bracket);

					if let Some(Ok(next_else)) = self.next_if_token_type(TokenType::Else) {
						match self.r#else(next_else) {
							Ok(statement) => Ok(Statement::If(
								expression,
								Box::new(body?),
								Some(Box::new(statement)),
							)),
							Err(error) => Err(error),
						}
					} else {
						Ok(Statement::If(expression, Box::new(body?), None))
					}
				} else {
					self.create_statement_syntax_error("Expected body".to_string(), token)
				}
			} else {
				self.create_statement_syntax_error("Expected close parenthesis".to_string(), token)
			}
		} else {
			self.create_statement_syntax_error("Expected open parenthesis".to_string(), token)
		}
	}

	fn r#else(&mut self, token: Token) -> Result<Statement> {
		if let Some(Ok(open_curly_bracket)) = self.next_if_token_type(TokenType::OpenCurlyBracket) {
			self.body(open_curly_bracket)
		} else {
			self.create_statement_syntax_error("Expected body".to_string(), token)
		}
	}
}

// Expression parsing
impl Parser<'_> {
	fn expression_root(&mut self) -> Result<Expression> {
		self.assignment()
	}

	fn assignment(&mut self) -> Result<Expression> {
		let expression: Expression = self.or()?;

		if let Some(Ok(token)) = self.next_if_in_token_types(&[
			TokenType::Assign,
			TokenType::AssignPlus,
			TokenType::AssignMinus,
			TokenType::AssignMultiply,
			TokenType::AssignDivision,
			TokenType::AssignExponentiation,
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

	fn or(&mut self) -> Result<Expression> {
		let mut expression: Expression = self.and()?;

		while let Some(Ok(token)) = self.next_if_token_type(TokenType::Or) {
			expression = Expression::Binary(Box::new(expression), token, Box::new(self.and()?));
		}

		Ok(expression)
	}

	fn and(&mut self) -> Result<Expression> {
		let mut expression: Expression = self.comparison()?;

		while let Some(Ok(token)) = self.next_if_token_type(TokenType::And) {
			expression =
				Expression::Binary(Box::new(expression), token, Box::new(self.comparison()?));
		}

		Ok(expression)
	}

	fn comparison(&mut self) -> Result<Expression> {
		let mut expression: Expression = self.addition_and_subtraction()?;

		while let Some(Ok(token)) = self.next_if_in_token_types(&[
			TokenType::Equal,
			TokenType::NotEqual,
			TokenType::Greater,
			TokenType::GreaterOrEqual,
			TokenType::Lesser,
			TokenType::LesserOrEqual,
		]) {
			expression = Expression::Binary(
				Box::new(expression),
				token,
				Box::new(self.addition_and_subtraction()?),
			);
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

		while let Some(Ok(token)) = self.next_if_token_type(TokenType::Exponentiation) {
			expression = Expression::Binary(Box::new(expression), token, Box::new(self.unary()?));
		}
		Ok(expression)
	}

	fn unary(&mut self) -> Result<Expression> {
		if let Some(Ok(token)) =
			self.next_if_in_token_types(&[TokenType::Minus, TokenType::Plus, TokenType::Not])
		{
			return Ok(Expression::Unary(token, Box::new(self.unary()?)));
		}

		self.get()
	}

	fn get(&mut self) -> Result<Expression> {
		let mut expression = self.terms()?;

		loop {
			expression = match self
				.next_if_in_token_types(&[TokenType::OpenParenthesis, TokenType::Dot])
			{
				Some(Err(error)) => return Err(error),
				Some(Ok(token)) => match token.r#type {
					TokenType::OpenParenthesis => self.call(expression, token)?,
					TokenType::Dot => {
						if let Some(Ok(identifier)) = self.next_if_token_type(TokenType::Identifier)
						{
							let get = Expression::Get(Box::new(expression), identifier);
							if let Some(Ok(open_parenthesis)) =
								self.next_if_token_type(TokenType::OpenParenthesis)
							{
								self.call(get, open_parenthesis)?
							} else {
								return Ok(get);
							}
						} else {
							return self.create_expression_syntax_error(
								"expected identifier".to_string(),
								token,
							);
						}
					}
					_ => unreachable!(),
				},
				None => break,
			};
		}

		Ok(expression)
	}

	fn call(&mut self, function: Expression, open_parenthesis: Token) -> Result<Expression> {
		let mut parameters: Vec<Expression> = Vec::new();

		while !self.peek_token_type(TokenType::CloseParenthesis) {
			parameters.push(self.expression_root()?);
			match self.next_if_token_type(TokenType::Separator) {
				Some(result) => match self.peek_token_type(TokenType::CloseParenthesis) {
					true => break,
					false => result?,
				},
				None => break,
			};
		}

		if let Some(Ok(close_parenthesis)) = self.next_if_token_type(TokenType::CloseParenthesis) {
			Ok(Expression::Call(
				Box::new(function),
				open_parenthesis,
				parameters,
				close_parenthesis,
			))
		} else {
			self.create_expression_syntax_error(
				"Incorrectly formatted parameters".to_string(),
				open_parenthesis,
			)
		}
	}

	fn terms(&mut self) -> Result<Expression> {
		if let Some(Ok(token)) = self.next_if_token_type(TokenType::Identifier) {
			return Ok(Expression::Identifier(token));
		}

		if let Some(Ok(token)) = self.next_if_in_token_types(&[
			TokenType::String,
			TokenType::Integer,
			TokenType::Boolean,
		]) {
			return Ok(Expression::Literal(token));
		}

		if let Some(Ok(_token)) = self.next_if_token_type(TokenType::OpenParenthesis) {
			let expression = self.assignment()?;

			if let Some(Ok(_next)) = self.next_if_token_type(TokenType::CloseParenthesis) {
				return Ok(Expression::Grouping(Box::new(expression)));
			}
		}

		match self.tokenizer.next() {
			Some(Ok(next)) => {
				self.create_expression_syntax_error("Unexpected token".to_string(), next)
			}
			Some(Err(err)) => Err(err),
			None => Err(Error::new(ErrorType::Eof, "Unexpected end of file".to_string())),
		}
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

	fn peek_token_type(&mut self, token_type: TokenType) -> bool {
		match self.tokenizer.peek() {
			Some(&Ok(ref token)) => token.r#type == token_type,
			_ => false,
		}
	}

	fn next_if_token_type(&mut self, token_type: TokenType) -> Option<Result<Token>> {
		if self.peek_token_type(token_type) {
			self.tokenizer.next()
		} else {
			None
		}
	}

	fn next_if_specifier(&mut self) -> Option<Result<Token>> {
		if let Some(Ok(specifier)) = self.next_if_token_type(TokenType::Specifier) {
			if let Some(Ok(r#type)) = self.next_if_token_type(TokenType::Identifier) {
				return Some(Ok(r#type));
			} else {
				return Some(
					self.create_token_syntax_error("Expected type".to_string(), specifier),
				);
			}
		}
		None
	}
}
