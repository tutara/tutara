use super::result::Error;
use super::result::ErrorType;
use super::result::Result;
use super::token::Token;
use super::token_type::TokenType;
use super::Literal;

use std::iter::Peekable;
use std::str::Chars;

pub struct Tokenizer<'a> {
	chars: Peekable<Chars<'a>>,
	// Cursor position
	line: u32,
	column: u32,
	// Current block length
	length: u32,
}

impl Iterator for Tokenizer<'_> {
	type Item = Result<Token>;

	fn next(&mut self) -> Option<Self::Item> {
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
				} else if current == '\'' {
					token = Some(self.string());
				} else if current == '/' && self.chars.peek() == Some(&'/') {
					self.chars.next();
					self.length += 1;

					token = Some(self.comment());
				} else if current == '&' {
					token = Some(self.token_if_char('&', TokenType::And, "expected &"))
				} else if current == '|' {
					token = Some(self.token_if_char('|', TokenType::Or, "expected |"))
				} else if let Some(r#type) = TokenType::get_reserved_token(&current.to_string()) {
					token = Some(self.create_token(r#type));
					token = Some(self.assignment_operation(token.unwrap().unwrap()));
					token = Some(self.comparison(token.unwrap().unwrap()))
				} else {
					token = Some(self.create_error(
						ErrorType::Lexical(self.line, self.column, self.length),
						format!(
							"Unexpected token at {} ({}:{})",
							current, self.line, self.column
						),
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
	fn create_token(&mut self, r#type: TokenType) -> Result<Token> {
		self.create_literal_token(r#type, None)
	}

	fn create_literal_token(
		&mut self,
		r#type: TokenType,
		literal: Option<Literal>,
	) -> Result<Token> {
		Ok(Token::new(
			r#type,
			literal,
			self.line,
			self.column,
			self.length,
		))
	}
	fn create_error(&mut self, r#type: ErrorType, message: String) -> Result<Token> {
		Err(Error::new(r#type, message))
	}
}

// Token functions
impl Tokenizer<'_> {
	fn number(&mut self, current: char) -> Result<Token> {
		let mut value = current.to_string();

		while let Some(next) = self.chars.peek() {
			if next.is_digit(10) {
				value.push(*next);
				self.chars.next();
				self.length += 1;
			} else {
				break;
			}
		}
		let literal = value.parse::<u32>();

		if let Err(_literal) = literal {
			self.create_error(
				ErrorType::Lexical(self.line, self.column, self.length),
				"Invalid number".to_string(),
			)
		} else {
			self.create_literal_token(TokenType::Integer, Some(Literal::Number(literal.unwrap())))
		}
	}

	fn identifier(&mut self, current: char) -> Result<Token> {
		let mut value = current.to_string();

		while let Some(next) = self.chars.peek() {
			if next.is_alphanumeric() {
				value.push(*next);
				self.chars.next();
				self.length += 1;
			} else {
				break;
			}
		}

		if let Some(r#type) = TokenType::get_reserved_token(&value) {
			self.create_token(r#type)
		} else {
			self.create_literal_token(TokenType::Identifier, Some(Literal::String(value)))
		}
	}

	fn string(&mut self) -> Result<Token> {
		let mut value = String::new();

		while let Some(next) = self.chars.peek() {
			if next == &'\\' {
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
					ErrorType::Lexical(self.line, self.column, self.length),
					"Unexpected new line, expected end of string.".to_string(),
				);
			} else {
				// other characters
				value.push(*next);
				self.chars.next();
				self.length += 1;
			}
		}

		self.create_literal_token(TokenType::String, Some(Literal::String(value)))
	}

	fn comment(&mut self) -> Result<Token> {
		let mut value = String::new();

		loop {
			if self.chars.peek() == Some(&'\n') {
				break;
			} else {
				let next = self.chars.next();

				if let Some(next) = next {
					value.push(next);
				} else {
					break;
				}

				self.length += 1;
			}
		}

		self.create_literal_token(TokenType::Comment, Some(Literal::String(value)))
	}

	fn escape(&mut self) -> Option<&str> {
		match self.chars.peek() {
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
		}
	}

	fn assignment_operation(&mut self, token: Token) -> Result<Token> {
		let r#type = &token.r#type;

		if TokenType::is_operation(&r#type) {
			if let Some(_next) = self.next_if_char('=') {
				self.chars.next();
				self.length += 1;
				return match r#type {
					TokenType::Plus => self.create_token(TokenType::AssignPlus),
					TokenType::Minus => self.create_token(TokenType::AssignMinus),
					TokenType::Multiply => self.create_token(TokenType::AssignMultiply),
					TokenType::Division => self.create_token(TokenType::AssignDivision),
					TokenType::Pow => self.create_token(TokenType::AssignPow),
					TokenType::Modulo => self.create_token(TokenType::AssignModulo),
					_ => self.create_error(
						ErrorType::Lexical(self.line, self.column, self.length),
						"Invalid assignment operation".to_string(),
					),
				};
			}
		}

		Ok(token)
	}

	fn comparison(&mut self, token: Token) -> Result<Token> {
		if vec![
			TokenType::Not,
			TokenType::Assign,
			TokenType::Greater,
			TokenType::Lesser,
		]
		.contains(&token.r#type)
		{
			if let Some(_next) = self.next_if_char('=') {
				self.chars.next();
				self.length += 1;
				return match token.r#type {
					TokenType::Not => self.create_token(TokenType::NotEqual),
					TokenType::Assign => self.create_token(TokenType::Equal),
					TokenType::Greater => self.create_token(TokenType::GreaterOrEqual),
					TokenType::Lesser => self.create_token(TokenType::LesserOrEqual),
					_ => self.create_error(
						ErrorType::Lexical(self.line, self.column, self.length),
						"Invalid assignment operation".to_string(),
					),
				};
			}
		}

		Ok(token)
	}
}

impl Tokenizer<'_> {
	fn peek_char(&mut self, next: char) -> bool {
		match self.chars.peek() {
			Some(peek) => *peek == next,
			_ => false,
		}
	}

	fn next_if_char(&mut self, next: char) -> Option<char> {
		if self.peek_char(next) {
			self.chars.next()
		} else {
			None
		}
	}

	fn token_if_char(
		&mut self,
		next: char,
		token_type: TokenType,
		error_message: &str,
	) -> Result<Token> {
		if let Some(_next) = self.next_if_char(next) {
			self.chars.next();
			self.length += 1;
			return self.create_token(token_type);
		}

		self.create_error(
			ErrorType::Lexical(self.line, self.column, self.length),
			error_message.to_string(),
		)
	}
}
