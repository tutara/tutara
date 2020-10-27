use super::token::Token;
use super::token_result::Error;
use super::token_result::ErrorType;
use super::token_result::TokenResult;
use super::token_type::TokenType;
use super::Literal;

pub struct Tokenizer<'a> {
	chars: std::iter::Peekable<std::str::Chars<'a>>,
	// Cursor position
	line: u32,
	column: u32,
	// Current block length
	length: u32,
}

impl Iterator for Tokenizer<'_> {
	type Item = TokenResult;

	fn next(&mut self) -> Option<Self::Item> {
		let _next = self.chars.peek();

		loop {
			if let Some(current) = self.chars.next() {
				self.length = 1;
				let mut token = None;

				// https://doc.rust-lang.org/reference/whitespace.html
				if current.is_whitespace() {
					if current == '\n' {
						self.line += 1;
						self.column = 0;
						self.length = 0;
					}
				} else if current.is_digit(10) {
					token = Some(self.number(current));
				} else if current.is_alphabetic() {
					token = Some(self.identifier(current));
				} else if current == *&'\'' {
					token = Some(self.string());
				} else if current == *&'/' && self.chars.peek() == Some(&'/') {
					self.chars.next();

					token = Some(self.comment());
				} else if let Some(r#type) = TokenType::get_reserved_token(&current.to_string()) {
					token = Some(self.create_token(r#type));
				} else {
					token = Some(self.create_error(
						ErrorType::Lexical,
						format!("Unexpected token at {} ({}:{})", current, self.line, self.column),
					))
				}

				self.column += self.length;

				if token.is_some() {
					return token;
				}
			} else {
				return None;
			}
		}
	}
}

// Constructor
impl Tokenizer<'_> {
	pub fn new(source: &str) -> Tokenizer {
		Tokenizer {
			chars: source.chars().peekable(),
			line: 1,
			column: 0,
			length: 0,
		}
	}
}

// Helper functions
impl Tokenizer<'_> {
	fn create_token(&mut self, r#type: TokenType) -> TokenResult {
		return self.create_literal_token(r#type, None);
	}

	fn create_literal_token(&mut self, r#type: TokenType, literal: Option<Literal>) -> TokenResult {
		return Ok(Token::new(
			r#type,
			literal,
			self.line,
			self.column,
			self.length,
		));
	}

	fn create_error(&mut self, r#type: ErrorType, message: String) -> TokenResult {
		return Err(Error::new(
			r#type,
			message,
			self.line,
			self.column,
			self.length,
		));
	}
}

// Token functions
impl Tokenizer<'_> {
	fn number(&mut self, current: char) -> TokenResult {
		let mut value = current.to_string();

		loop {
			if let Some(next) = self.chars.peek() {
				if next.is_digit(10) {
					value.push(*next);
					self.chars.next();
					self.length += 1;
				} else {
					break;
				}
			} else {
				break;
			}
		}
		let literal = value.parse::<u32>();

		if literal.is_err() {
			return self.create_error(ErrorType::Lexical, "Invalid number".to_string());
		} else {
			return self
				.create_literal_token(TokenType::Integer, Some(Literal::Number(literal.unwrap())));
		}
	}

	fn identifier(&mut self, current: char) -> TokenResult {
		let mut value = current.to_string();

		loop {
			if let Some(next) = self.chars.peek() {
				if next.is_alphabetic() {
					value.push(*next);
					self.chars.next();
					self.length += 1;
				} else {
					break;
				}
			} else {
				break;
			}
		}

		if let Some(r#type) = TokenType::get_reserved_token(&value) {
			return self.create_token(r#type);
		} else {
			return self.create_literal_token(TokenType::Identifier, Some(Literal::String(value)));
		}
	}

	fn string(&mut self) -> TokenResult {
		let mut value = String::new();

		loop {
			if let Some(next) = self.chars.peek() {
				if next == &*&'\\' {
					// Escape sequences
					self.chars.next();
					self.length += 1;
					if let Some(escaped) = self.escape() {
						value += &*escaped;
					} else {
						value.push('\\');
					}
				} else if *next == '\'' {
					// string end
					self.chars.next();
					self.length += 1;
					break;
				} else if *next == '\n' {
					return self.create_error(
						ErrorType::Lexical,
						"Unexpected new line, expected end of string.".to_string(),
					);
				} else {
					// other characters
					value.push(*next);
					self.chars.next();
					self.length += 1;
				}
			} else {
				break;
			}
		}

		return self.create_literal_token(TokenType::String, Some(Literal::String(value)));
	}

	fn comment(&mut self) -> TokenResult {
		let mut value = String::new();

		loop {
			if self.chars.peek() == Some(&'\n') {
				break;
			} else {
				let next = self.chars.next();

				if next.is_some() {
					value.push(next.unwrap());
				} else {
					break;
				}

				self.length += 1;
			}
		}

		return self.create_literal_token(TokenType::Comment, Some(Literal::String(value)));
	}

	fn escape(&mut self) -> Option<&str> {
		return match self.chars.peek() {
			Some('n') => {
				self.chars.next();
				Some("\n")
			}
			Some('r') => {
				self.chars.next();
				Some("\r")
			}
			Some('t') => {
				self.chars.next();
				Some("\t")
			}
			Some('\\') => {
				self.chars.next();
				Some("\\")
			}
			Some('\'') => {
				self.chars.next();
				Some("'")
			}
			_ => None,
		};
	}
}
