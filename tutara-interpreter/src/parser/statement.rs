use crate::{Expression, Token};

use std::fmt::{self, Debug};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Statement {
	Expression(Expression),
	Declaration(Token, Option<Box<Statement>>, Expression),	// var | val, Option<TypeSpecification>, Assignment | Identifier
	TypeSpecification(Token, Token), 						// Specifier, Identifier
	Comment(Token),
	Block(Vec<Statement>),
	Body(Token, Vec<Statement>, Token),						// OpenCurlyBracket, Statements, CloseCurlyBracket
	Parameters(Token, Vec<Statement>, Token),				// OpenParenthesis, Vec<Parameter>, CloseParenthesis 
	Parameter(Token, Box<Statement>, Option<Token>),		// Identifier, TypeSpecification, Option<Seperator>
	Function(
		Token,												// Function
		Option<Box<Statement>>,								// Option<TypeSpecification>
		Token,												// Identifier
		Option<Box<Statement>>,								// Option<Parameters>
		Box<Statement>,										// Body
	),
	Return(Token, Option<Expression>),						// Return, Option<Expression>
}

impl fmt::Display for Statement {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:#?}", self)
	}
}
