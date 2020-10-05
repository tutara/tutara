use std::fmt::{self, Debug};

#[derive(Debug)]
pub enum TokenType {
	// Primitives
	Integer, // 12
	String, // "foo"
	// Booleans
	True,  // true
	False, // false
	// Variables
	Val, // Immutable
	Var, // Mutable
	// Reference
	Identifier, // NAME
	// Arithmetic Operations
	Plus,      // +
	Minus,     // -
	Multiply,  // *
	Division,    // /
	Pow,       // ^
	Modulo,    // %
	// Uncategorized
	Assign,    // =
	Specifier, // :
	// System
	Comment,
}

impl TokenType {
	pub fn get_reserved_token(ident: &str) -> Option<TokenType> {
		use TokenType::*;

		match ident {
			"true" => Some(True),
			"false" => Some(False),

			"val" => Some(Val),
			"var" => Some(Var),
			
			"+" => Some(Plus),
			"-" => Some(Minus),
			"*" => Some(Multiply),
			"/" => Some(Division),
			"^" => Some(Pow),
			"%" => Some(Modulo),
			
			"=" => Some(Assign),
			":" => Some(Specifier),
			_ => None,
		}
	}
}

impl fmt::Display for TokenType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}
