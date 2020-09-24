use super::token_type::TokenType;
use super::literal::Literal;

pub struct Token {
	pub r#type: TokenType,
	pub literal: Option<Literal>,
	pub line: u32,
	pub column: u32,
	pub length: u32,
}

impl Token {
	pub fn new(
		r#type: TokenType,
		literal: Option<Literal>,
		line: u32,
		column: u32,
		length: u32,
	) -> Token {
		Token {
			r#type,
			literal,
			line,
			column,
			length,
		}
	}
}
