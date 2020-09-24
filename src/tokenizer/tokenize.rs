use super::Literal;
use super::token::Token;
use super::token_type::TokenType;

pub struct Tokenizer<'a> {
	chars: std::iter::Peekable<std::str::Chars<'a>>,
	pub tokens: Vec<Token>,
	// Cursor position
	line: u32,
	column: u32,
	// Current block length
	length: u32,
}

impl Tokenizer<'_> {
	pub fn new(source: &str) -> Tokenizer {
		Tokenizer {
			chars: source.chars().peekable(),
			tokens: Vec::new(),
			line: 1,
			column: 0,
			length: 0,
		}
	}

	fn create_token(&mut self, r#type: TokenType) -> Token {
		return self.create_literal_token(r#type, None);
	}

	fn create_literal_token(&mut self, r#type: TokenType, literal: Option<Literal>) -> Token {
		return Token::new(r#type, literal, self.line, self.column, self.length);
	}

	fn read(&mut self) {
		if let Some(current) = self.chars.next() {
			self.length = 1;
			// https://doc.rust-lang.org/reference/whitespace.html
			if current.is_whitespace() {
				if current == '\r' {
					self.line += 1;
					self.column = 0;
				}
			} else if current.is_digit(10) {
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

				let token = self.create_literal_token(
					TokenType::Integer,
					Some(Literal::Number(value.parse::<u32>().unwrap())),
				);
				self.tokens.push(token);
			} else if current.is_alphabetic() {
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
					let token = self.create_token(r#type);
					self.tokens.push(token);
				} else {
					let token = self
						.create_literal_token(TokenType::Identifier, Some(Literal::String(value)));
					self.tokens.push(token);
				}
			} else if let Some(r#type) = TokenType::get_reserved_token(&current.to_string()) {
				let token = self.create_token(r#type);
				self.tokens.push(token);
			} else {
				println!("Crash at {}", current);

				unimplemented!();
			}

			self.column += self.length;
		} else {
			unreachable!();
		}
	}

	fn eof(&mut self) {
		self.length = 0;
		let token = self.create_token(TokenType::Eof);
		self.tokens.push(token);
	}

	pub fn tokenize(&mut self) {
		loop {
			let next = self.chars.peek();

			match next {
				None => {
					self.eof();
					break;
				}
				_ => self.read(),
			}
		}
	}
}
