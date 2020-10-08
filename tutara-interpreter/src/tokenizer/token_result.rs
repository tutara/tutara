//1: raw -> tokens - LexicalError
//2: tokens -> optimized tokens - ??Error
//3: optimized tokens -> uitvoer - RuntimeError
use std::result;
use super::token::Token;

pub type TokenResult = result::Result<Token, Error>;

#[derive(Debug)]
pub enum ErrorType {
	Lexical,
}

#[derive(Debug)]
pub struct Error {
	pub r#type: ErrorType,
	pub message: String,
	pub line: u32,
	pub column: u32,
	pub length: u32,
}

impl Error {
	pub fn new(
		r#type: ErrorType,
		message: String,
		line: u32,
		column: u32,
		length: u32,
		
	) -> Error {
		Error {
			r#type,
			message,
			line,
			column,
			length,
		}
	}
}
