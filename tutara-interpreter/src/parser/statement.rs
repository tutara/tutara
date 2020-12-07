use crate::{Expression, Token};

use serde::{Deserialize, Serialize};
use std::fmt::{self, Debug};

#[derive(Debug, Serialize, Deserialize, Eq)]
pub enum Statement {
	Expression(Expression),
	Declaration(Token, Option<Token>, Expression), // var | val , Type , Assignment | Identifier
	Comment(Token),
	Body(Vec<Statement>),       // Statements
	Function(
		Option<Token>,          // Type
		Token,                  // Identifier
		Vec<(Token, Token)>, // Vec<Parameter(Identifier, Type)>
		Box<Statement>,         // Body
	),
	Loop(Box<Statement>),              // Body
	While(Expression, Box<Statement>), // Condition , Body
	For(
		Expression,     // Identifier
		Expression,     // Iterable
		Box<Statement>, // Body
	),
	Break,
	Return(Option<Expression>), // Option<Expression>
	If(Expression, Box<Statement>, Option<Box<Statement>>), // Expression , Body A , Body B
}

impl fmt::Display for Statement {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:#?}", self)
	}
}

impl PartialEq for Statement {
	fn eq(&self, other: &Statement) -> bool {
		use Statement::*;
		match *self {
			Expression(ref a_expression) => match *other {
				Expression(ref b_expression) => a_expression.eq(b_expression),
				_ => false,
			},
			Declaration(ref a_token, ref a_statement, ref a_expression) => match *other {
				Declaration(ref b_token, ref b_statement, ref b_expression) => {
					a_token.eq(b_token)
						&& a_statement.eq(b_statement)
						&& a_expression.eq(b_expression)
				}
				_ => false,
			},
			Comment(ref a_comment) => match *other {
				Comment(ref b_comment) => a_comment.eq(b_comment),
				_ => false,
			},
			Body(ref a_body) => match *other {
				Body(ref b_body) => a_body.eq(b_body),
				_ => false,
			},
			Function(ref a_type_specification, ref a_identifier, ref a_parameters, ref a_body) => {
				match *other {
					Function(
						ref b_type_specification,
						ref b_identifier,
						ref b_parameters,
						ref b_body,
					) => {
						a_type_specification.eq(b_type_specification)
							&& a_identifier.eq(b_identifier)
							&& a_parameters.eq(b_parameters)
							&& a_body.eq(b_body)
					}
					_ => false,
				}
			}
			Loop(ref a_body) => match *other {
				Loop(ref b_body) => a_body.eq(b_body),
				_ => false,
			},
			While(ref a_condition, ref a_body) => match *other {
				While(ref b_condition, ref b_body) => {
					a_condition.eq(b_condition) && a_body.eq(b_body)
				}
				_ => false,
			},
			For(ref a_identifier, ref a_iterable, ref a_body) => match *other {
				For(ref b_identifier, ref b_iterable, ref b_body) => {
					a_identifier.eq(b_identifier) && a_iterable.eq(b_iterable) && a_body.eq(b_body)
				}
				_ => false,
			},
			Break => matches!(*other, Break),
			Return(ref a_expression) => match *other {
				Return(ref b_expression) => a_expression.eq(b_expression),
				_ => false,
			},
			If(ref a_expression, ref a_body, ref a_else) => match *other {
				If(ref b_expression, ref b_body, ref b_else) => {
					a_expression.eq(b_expression) && a_body.eq(b_body) && a_else.eq(b_else)
				}
				_ => false,
			}
		}
	}
}
