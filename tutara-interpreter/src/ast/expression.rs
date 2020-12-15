use crate::Token;

use std::fmt::{self, Debug};
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum Expression {
	Literal(Token),											// Token (With literal value)
	Identifier(Token),
	Binary(Box<Expression>, Token, Box<Expression>),		// Binding two expressions with an operator
	Unary(Token, Box<Expression>),							// Binding a expression with an operator
	Grouping(Box<Expression>),								// Group of an expression between ( )
	Assignment(Token, Token, Box<Expression>),  			// Identifier, Assignment Operator, Expression
	Get(Box<Expression>, Token),							// Called on, Called item
	Call(Box<Expression>, Token, Vec<Expression>, Token),	// Identifier | Get, (, Literal | identifier ,)
}

impl fmt::Display for Expression {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:#?}", self)
	}
}
