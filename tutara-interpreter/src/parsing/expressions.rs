use crate::ast::*;
use crate::parser::Parser;
use crate::Error;
use crate::ErrorType;
use crate::Result;

impl Parser<'_> {
	pub(crate) fn expression(&mut self) -> Result<Statement> {
		match self.expression_root() {
			Ok(expression) => Ok(Statement::Expression(expression)),
			Err(error) => Err(error),
		}
	}

	pub(crate) fn expression_root(&mut self) -> Result<Expression> {
		self.assignment()
	}

	pub(super) fn assignment(&mut self) -> Result<Expression> {
		use TokenType::*;

		let expression: Expression = self.or()?;

		if let Some(Ok(token)) = self.next_if_in_token_types(&[
			Assign,
			AssignPlus,
			AssignMinus,
			AssignMultiply,
			AssignDivision,
			AssignExponentiation,
			AssignModulo,
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

	pub(super) fn or(&mut self) -> Result<Expression> {
		let mut expression: Expression = self.and()?;

		while let Some(Ok(token)) = self.next_if_token_type(TokenType::Or) {
			expression = Expression::Binary(Box::new(expression), token, Box::new(self.and()?));
		}

		Ok(expression)
	}

	pub(super) fn and(&mut self) -> Result<Expression> {
		let mut expression: Expression = self.comparison()?;

		while let Some(Ok(token)) = self.next_if_token_type(TokenType::And) {
			expression =
				Expression::Binary(Box::new(expression), token, Box::new(self.comparison()?));
		}

		Ok(expression)
	}

	pub(super) fn comparison(&mut self) -> Result<Expression> {
		use TokenType::*;

		let mut expression: Expression = self.addition_and_subtraction()?;

		while let Some(Ok(token)) = self.next_if_in_token_types(&[
			Equal,
			NotEqual,
			Greater,
			GreaterOrEqual,
			Lesser,
			LesserOrEqual,
		]) {
			expression = Expression::Binary(
				Box::new(expression),
				token,
				Box::new(self.addition_and_subtraction()?),
			);
		}

		Ok(expression)
	}

	pub(super) fn addition_and_subtraction(&mut self) -> Result<Expression> {
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

	pub(super) fn multiplication_division_modulo(&mut self) -> Result<Expression> {
		use TokenType::*;

		let mut expression = self.involution()?;

		while let Some(Ok(token)) = self.next_if_in_token_types(&[Multiply, Division, Modulo]) {
			expression =
				Expression::Binary(Box::new(expression), token, Box::new(self.involution()?));
		}
		Ok(expression)
	}

	pub(super) fn involution(&mut self) -> Result<Expression> {
		let mut expression = self.unary()?;

		while let Some(Ok(token)) = self.next_if_token_type(TokenType::Exponentiation) {
			expression = Expression::Binary(Box::new(expression), token, Box::new(self.unary()?));
		}
		Ok(expression)
	}

	pub(super) fn unary(&mut self) -> Result<Expression> {
		if let Some(Ok(token)) =
			self.next_if_in_token_types(&[TokenType::Minus, TokenType::Plus, TokenType::Not])
		{
			return Ok(Expression::Unary(token, Box::new(self.unary()?)));
		}

		self.get()
	}

	pub(super) fn get(&mut self) -> Result<Expression> {
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

	pub(super) fn call(&mut self, function: Expression, open_parenthesis: Token) -> Result<Expression> {
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

	pub(crate) fn terms(&mut self) -> Result<Expression> {
		use TokenType::*;

		if let Some(Ok(token)) = self.next_if_token_type(Identifier) {
			return Ok(Expression::Identifier(token));
		}

		if let Some(Ok(token)) = self.next_if_in_token_types(&[String, Integer, Boolean]) {
			return Ok(Expression::Literal(token));
		}

		if let Some(Ok(_token)) = self.next_if_token_type(OpenParenthesis) {
			let expression = self.assignment()?;

			if let Some(Ok(_next)) = self.next_if_token_type(CloseParenthesis) {
				return Ok(Expression::Grouping(Box::new(expression)));
			}
		}

		match self.tokenizer.next() {
			Some(Ok(next)) => {
				self.create_expression_syntax_error("Unexpected token".to_string(), next)
			}
			Some(Err(err)) => Err(err),
			None => Err(Error::new(
				ErrorType::Eof,
				"Unexpected end of file".to_string(),
			)),
		}
	}
}
