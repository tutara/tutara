use crate::ast::*;
use crate::parser::Parser;
use crate::Result;

impl Parser<'_> {
	pub(crate) fn r#if(&mut self, token: Token) -> Result<Statement> {
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

	pub(crate) fn r#else(&mut self, token: Token) -> Result<Statement> {
		if let Some(Ok(open_curly_bracket)) = self.next_if_token_type(TokenType::OpenCurlyBracket) {
			self.body(open_curly_bracket)
		} else {
			self.create_statement_syntax_error("Expected body".to_string(), token)
		}
	}
}
