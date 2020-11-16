
use crate::Token;
use std::fmt::{self, Debug};
use std::result;
use serde::{Serialize, Deserialize};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, Serialize, Deserialize)]
pub enum ErrorType {
	Lexical(u32, u32, u32), // Line, column, length
	Parser(Token),
	Compiler
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
	pub r#type: ErrorType,
	pub message: String,
}

impl Error {
	pub fn new(r#type: ErrorType, message: String) -> Error {
		Error { r#type, message }
	}

	pub fn new_lexical_error(message: String, line: u32, column: u32, length: u32) -> Error {
		Error {
			r#type: ErrorType::Lexical(line, column, length),
			message,
		}
	}

	pub fn new_parser_error(message: String, token: Token) -> Error {
		Error {
			r#type: ErrorType::Parser(token),
			message,
		}
	}

	pub fn new_compiler_error(message: String) -> Error {
		Error {
			r#type: ErrorType::Compiler,
			message,
		}
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match &self.r#type {
			ErrorType::Lexical(line, column, _length) => write!(
				f,
				"Error at line {} on column {}: {}",
				line, column, self.message
			),
			ErrorType::Parser(token) => write!(
				f,
				"Syntax error on {}: at line: {} on column: {}, message: {}",
				token.r#type, token.line, token.column, self.message
			),
		    ErrorType::Compiler => write!(
				f,
				"Compiler error: {}",
				self.message
			)
		}
	}
}
