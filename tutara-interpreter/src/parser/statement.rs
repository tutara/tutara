use crate::{Expression, Token};

use serde::{Deserialize, Serialize};
use std::fmt::{self, Debug};

#[derive(Debug, Serialize, Deserialize, Eq)]
pub enum Statement {
	Expression(Expression),
	Declaration(Token, Option<Box<Statement>>, Expression), // var | val, Option<TypeSpecification>, Assignment | Identifier
	TypeSpecification(Token, Token),                        // Specifier, Identifier
	Comment(Token),
	Block(Vec<Statement>),
	Body(Token, Vec<Statement>, Token), // OpenCurlyBracket, Statements, CloseCurlyBracket
	Parameters(Token, Vec<Statement>, Token), // OpenParenthesis, Vec<Parameter>, CloseParenthesis
	Parameter(Token, Box<Statement>, Option<Token>), // Identifier, TypeSpecification, Option<Seperator>
	Function(
		Token,                  // Function
		Option<Box<Statement>>, // Option<TypeSpecification>
		Token,                  // Identifier
		Option<Box<Statement>>, // Option<Parameters>
		Box<Statement>,         // Body
	),
	Loop(Token, Box<Statement>),                            // Loop, Body
	While(Token, Token, Expression, Token, Box<Statement>), // While, (, condition ), Body
	For(
		Token,          // For
		Token,          // (
		Expression,     // Identifier
		Token,          // in
		Expression,     // iterable
		Token,          // )
		Box<Statement>, // Body
	),
	Break(Token),
	Return(Token, Option<Expression>), // Return, Option<Expression>
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
			TypeSpecification(ref a_specifier, ref a_identifier) => match *other {
				TypeSpecification(ref b_specifier, ref b_identifier) => {
					a_specifier.eq(b_specifier) && a_identifier.eq(b_identifier)
				}
				_ => false,
			},
			Comment(ref a_comment) => match *other {
				Comment(ref b_comment) => a_comment.eq(b_comment),
				_ => false,
			},
			Block(ref a_block) => match *other {
				Block(ref b_block) => a_block.eq(b_block),
				_ => false,
			},
			Body(ref a_open, ref a_body, ref a_close) => match *other {
				Body(ref b_open, ref b_body, ref b_close) => {
					a_open.eq(b_open) && a_body.eq(b_body) && a_close.eq(b_close)
				}
				_ => false,
			},
			Parameters(ref a_open, ref a_parameters, ref a_close) => match *other {
				Parameters(ref b_open, ref b_parameters, ref b_close) => {
					a_open.eq(b_open) && a_parameters.eq(b_parameters) && a_close.eq(b_close)
				}
				_ => false,
			},
			Parameter(ref a_identifier, ref a_type_specification, ref a_comma) => match *other {
				Parameter(ref b_identifier, ref b_type_specification, ref b_comma) => {
					a_identifier.eq(b_identifier)
						&& a_type_specification.eq(b_type_specification)
						&& a_comma.eq(b_comma)
				}
				_ => false,
			},
			Function(
				ref a_fun,
				ref a_type_specification,
				ref a_identifier,
				ref a_parameters,
				ref a_body,
			) => match *other {
				Function(
					ref b_fun,
					ref b_type_specification,
					ref b_identifier,
					ref b_parameters,
					ref b_body,
				) => {
					a_fun.eq(b_fun)
						&& a_type_specification.eq(b_type_specification)
						&& a_identifier.eq(b_identifier)
						&& a_parameters.eq(b_parameters)
						&& a_body.eq(b_body)
				}
				_ => false,
			},
			Loop(ref a_loop, ref a_body) => match *other {
				Loop(ref b_loop, ref b_body) => a_loop.eq(b_loop) && a_body.eq(b_body),
				_ => false,
			},
			While(ref a_while, ref a_open, ref a_condition, ref a_close, ref a_body) => {
				match *other {
					While(ref b_while, ref b_open, ref b_condition, ref b_close, ref b_body) => {
						a_while.eq(b_while)
							&& a_open.eq(b_open) && a_condition.eq(b_condition)
							&& a_close.eq(b_close) && a_body.eq(b_body)
					}
					_ => false,
				}
			}
			For(
				ref a_for,
				ref a_open,
				ref a_identifier,
				ref a_in,
				ref a_iterable,
				ref a_close,
				ref a_body,
			) => match *other {
				For(
					ref b_for,
					ref b_open,
					ref b_identifier,
					ref b_in,
					ref b_iterable,
					ref b_close,
					ref b_body,
				) => {
					a_for.eq(b_for)
						&& a_open.eq(b_open) && a_identifier.eq(b_identifier)
						&& a_in.eq(b_in) && a_iterable.eq(b_iterable)
						&& a_close.eq(b_close) && a_body.eq(b_body)
				}
				_ => false,
			},
			Break(ref a_break) => match *other {
				Break(ref b_break) => a_break.eq(b_break),
				_ => false,
			},
			Return(ref a_return, ref a_expression) => match *other {
				Return(ref b_return, ref b_expression) => {
					a_return.eq(b_return) && a_expression.eq(b_expression)
				}
				_ => false,
			},
		}
	}
}
