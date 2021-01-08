use crate::ast::*;
use crate::parser::Parser;
use crate::Error;
use crate::Result;

impl Parser<'_> {
	pub(crate) fn function(&mut self, token: Token) -> Result<Statement> {
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
					_ => {
						return self.create_statement_syntax_error(
							"Expected closing parenthesis".to_string(),
							open_parenthesis,
						)
					}
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
				Err(Error::new_parser_error(
					"Expected seperator".to_string(),
					identifier,
				))
			}
		} else {
			let token = self.tokenizer.next().unwrap().unwrap();
			Err(Error::new_parser_error(
				"Expected identifier".to_string(),
				token,
			))
		}
	}
}
