use crate::ast::*;
use crate::Error;
use crate::Result;
use crate::Tokenizer;

use core::iter::Peekable;

pub struct Parser<'a> {
	pub(super) tokenizer: Peekable<Tokenizer<'a>>,
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

// Statement parsing
impl Parser<'_> {
	fn statement(&mut self) -> Result<Statement> {
		use TokenType::*;
	
		if let Some(token) = self.next_if_in_token_types(&[
			Var, Val, Comment, Function, Return, Loop, While, For, Break, Continue, If,
		]) {
			if let Ok(token) = token {
				match token.r#type {
					Val | Var => self.declaration(token),
					Comment => Ok(Statement::Comment(token)),
					Function => self.function(token),
					Return => Ok(Statement::Return(self.expression_root().ok())),
					Loop => self.r#loop(token),
					While => self.r#while(token),
					For => self.r#for(token),
					Break => Ok(Statement::Break),
					Continue => Ok(Statement::Continue),
					If => self.r#if(token),
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
}

// Helper functions for iterating trough tokens
impl Parser<'_> {
	pub(super) fn peek_in_token_types(&mut self, types: &[TokenType]) -> bool {
		match self.tokenizer.peek() {
			Some(&Ok(ref token)) => types.contains(&token.r#type),
			_ => false,
		}
	}

	pub(super) fn next_if_in_token_types(&mut self, types: &[TokenType]) -> Option<Result<Token>> {
		if self.peek_in_token_types(types) {
			self.tokenizer.next()
		} else {
			None
		}
	}

	pub(super) fn peek_token_type(&mut self, token_type: TokenType) -> bool {
		match self.tokenizer.peek() {
			Some(&Ok(ref token)) => token.r#type == token_type,
			_ => false,
		}
	}

	pub(super) fn next_if_token_type(&mut self, token_type: TokenType) -> Option<Result<Token>> {
		if self.peek_token_type(token_type) {
			self.tokenizer.next()
		} else {
			None
		}
	}

	pub(super) fn next_if_specifier(&mut self) -> Option<Result<Token>> {
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

// Error creation
impl Parser<'_> {
	pub(super) fn create_token_syntax_error(
		&mut self,
		message: String,
		token: Token,
	) -> Result<Token> {
		Err(Error::new_parser_error(message, token))
	}

	pub(super) fn create_expression_syntax_error(
		&mut self,
		message: String,
		token: Token,
	) -> Result<Expression> {
		Err(Error::new_parser_error(message, token))
	}

	pub(super) fn create_statement_syntax_error(
		&mut self,
		message: String,
		token: Token,
	) -> Result<Statement> {
		Err(Error::new_parser_error(message, token))
	}
}
