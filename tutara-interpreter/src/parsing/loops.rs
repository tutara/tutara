use crate::ast::*;
use crate::parser::Parser;
use crate::Result;

impl Parser<'_> {
	pub(crate) fn r#loop(&mut self, token: Token) -> Result<Statement> {
		if let Some(Ok(open_curly_bracket)) = self.next_if_token_type(TokenType::OpenCurlyBracket) {
			match self.body(open_curly_bracket) {
				Ok(body) => Ok(Statement::Loop(Box::new(body))),
				Err(error) => Err(error),
			}
		} else {
			self.create_statement_syntax_error("Expected loop body".to_string(), token)
		}
	}

	pub(crate) fn r#while(&mut self, token: Token) -> Result<Statement> {
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

	pub(crate) fn r#for(&mut self, token: Token) -> Result<Statement> {
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
}
