use super::literal::Literal;
use super::token_type::TokenType;

use std::fmt::{self, Debug};
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
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

impl fmt::Display for Token {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:#?}", self)
	}
}
