use crate::ast::*;
use crate::Result;
use crate::parser::Parser;

impl Parser<'_>{
	pub(crate) fn declaration(&mut self, token: Token) -> Result<Statement> {
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
}
