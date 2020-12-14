use serde::{Deserialize, Serialize};
use std::fmt::{self, Debug};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum TokenType {
	// Primitives
	Integer, // 12
	String,  // "foo"
	Boolean, // true / false
	// Variables
	Val, // Immutable
	Var, // Mutable
	// Reference
	Identifier, // NAME
	// Operations
	Plus,     			// +
	Minus,    			// -
	Multiply, 			// *
	Division, 			// /
	Exponentiation ,    // **
	Modulo,   			// %
	// Logic
	Not,
	And,
	Or,
	Equal,
	NotEqual,
	GreaterOrEqual,
	LesserOrEqual,
	Greater,
	Lesser,
	// Assignment operator
	Assign,         		// =
	AssignPlus,     		// +=
	AssignMinus,    		// -=
	AssignMultiply, 		// *=
	AssignDivision, 		// /=
	AssignExponentiation,   // **=
	AssignModulo,   		// %=
	// Function
	Function,  // fun
	Return,    // return
	Separator, // ,
	// Choice
	If,
	Else,
	Match,
	// Loops
	Break,
	Continue,
	While,
	Loop,
	For,
	In,
	// Braces
	OpenParenthesis,   	// (
	CloseParenthesis,  	// )
	OpenCurlyBracket,  	// {
	CloseCurlyBracket, 	// }
	// Uncategorized
	Specifier, 			// :
	Dot,
	Arrow,				// ->
	// System
	Comment,
}

impl TokenType {
	pub fn is_operation(token: &TokenType) -> bool {
		vec![
			TokenType::Plus,
			TokenType::Minus,
			TokenType::Multiply,
			TokenType::Division,
			TokenType::Exponentiation,
			TokenType::Modulo,
		]
		.contains(&token)
	}

	pub fn get_reserved_token(ident: &str) -> Option<TokenType> {
		use TokenType::*;

		match ident {
			"val" => Some(Val),
			"var" => Some(Var),

			"!" => Some(Not),
			"<" => Some(Lesser),
			">" => Some(Greater),

			"+" => Some(Plus),
			"-" => Some(Minus),
			"*" => Some(Multiply),
			"/" => Some(Division),
			"**" => Some(Exponentiation),
			"%" => Some(Modulo),

			"fun" => Some(Function),
			"return" => Some(Return),
			"," => Some(Separator),

			"if" => Some(If),
			"else" => Some(Else),
			"match" => Some(Match),

			"break" => Some(Break),
			"continue" => Some(Continue),
			"while" => Some(While),
			"loop" => Some(Loop),
			"for" => Some(For),
			"in" => Some(In),

			"(" => Some(OpenParenthesis),
			")" => Some(CloseParenthesis),
			"{" => Some(OpenCurlyBracket),
			"}" => Some(CloseCurlyBracket),

			"=" => Some(Assign),
			":" => Some(Specifier),
			"." => Some(Dot),
			_ => None,
		}
	}
}

impl fmt::Display for TokenType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}
