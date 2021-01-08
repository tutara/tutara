use crate::ast::*;
use crate::Result;
use crate::parser::Parser;

impl Parser<'_>{
	pub(crate) fn body(&mut self, open_curly_bracket: Token) -> Result<Statement> {
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
}
