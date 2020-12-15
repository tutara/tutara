use tutara_interpreter::{Expression, Literal, Parser, Statement, Token, TokenType, Tokenizer};

use Expression::*;
use Statement::*;

#[test]
fn test_create_parser() {
	let tokenizer = Tokenizer::new("script");
	let parser = Parser::new(tokenizer.peekable());
	let statements: Result<Vec<Statement>, tutara_interpreter::Error> = parser.collect();

	assert_eq!(1, statements.unwrap().len())
}

fn create_parser_test(input: &str, expected_statement: Statement) {
	let tokenizer = Tokenizer::new(input);
	let mut parser = Parser::new(tokenizer.peekable());
	let given_statement = parser.next().unwrap().unwrap();
	assert_eq!(expected_statement, given_statement);
}

fn create_fail_statement_test(input: &str) {
	let tokenizer = Tokenizer::new(input);
	let mut parser = Parser::new(tokenizer.peekable());
	let given_statement = parser.next().unwrap();
	assert!(given_statement.is_err());
}

#[test]
fn test_create_expression_literal_number() {
	create_parser_test(
		"1000",
		Expression(Literal(Token::new(
			TokenType::Integer,
			Some(Literal::Number(1000)),
			1,
			0,
			4,
		))),
	)
}

#[test]
fn test_create_invalid_expression_literal_number() {
	create_fail_statement_test("1000000000000000000000")
}

#[test]
fn test_create_invalid_token() {
	create_fail_statement_test("@");
	create_fail_statement_test("`");
	create_fail_statement_test("~");
}

#[test]
fn test_create_expression_literal_string() {
	create_parser_test(
		"'string'",
		Expression(Literal(Token::new(
			TokenType::String,
			Some(Literal::String("string".to_string())),
			1,
			0,
			8,
		))),
	)
}

#[test]
fn test_create_expression_identifier() {
	create_parser_test(
		"foo",
		Expression(Identifier(Token::new(
			TokenType::Identifier,
			Some(Literal::String("foo".to_string())),
			1,
			0,
			3,
		))),
	)
}

#[test]
fn test_create_declaration_integer() {
	create_parser_test(
		"val foo = 12",
		Declaration(
			Token::new(TokenType::Val, None, 1, 0, 3),
			None,
			Assignment(
				Token::new(
					TokenType::Identifier,
					Some(Literal::String("foo".to_string())),
					1,
					4,
					3,
				),
				Token::new(TokenType::Assign, None, 1, 8, 1),
				Box::new(Literal(Token::new(
					TokenType::Integer,
					Some(Literal::Number(12)),
					1,
					10,
					2,
				))),
			),
		),
	)
}

#[test]
fn test_create_declaration_boolean() {
	create_parser_test(
		"var foo = true",
		Declaration(
			Token::new(TokenType::Var, None, 1, 0, 3),
			None,
			Assignment(
				Token::new(
					TokenType::Identifier,
					Some(Literal::String("foo".to_string())),
					1,
					4,
					3,
				),
				Token::new(TokenType::Assign, None, 1, 8, 1),
				Box::new(Literal(Token::new(
					TokenType::Boolean,
					Some(Literal::Boolean(true)),
					1,
					10,
					4,
				))),
			),
		),
	)
}

#[test]
fn test_create_declaration_string() {
	create_parser_test(
		"val foo = 'string'",
		Declaration(
			Token::new(TokenType::Val, None, 1, 0, 3),
			None,
			Assignment(
				Token::new(
					TokenType::Identifier,
					Some(Literal::String("foo".to_string())),
					1,
					4,
					3,
				),
				Token::new(TokenType::Assign, None, 1, 8, 1),
				Box::new(Literal(Token::new(
					TokenType::String,
					Some(Literal::String("string".to_string())),
					1,
					10,
					8,
				))),
			),
		),
	)
}

#[test]
fn test_create_typed_declaration_number() {
	create_parser_test(
		"var: Integer foo = 200",
		Declaration(
			Token::new(TokenType::Var, None, 1, 0, 3),
			Some(Token::new(
				TokenType::Identifier,
				Some(Literal::String("Integer".to_string())),
				1,
				5,
				7,
			)),
			Assignment(
				Token::new(
					TokenType::Identifier,
					Some(Literal::String("foo".to_string())),
					1,
					13,
					3,
				),
				Token::new(TokenType::Assign, None, 1, 17, 1),
				Box::new(Literal(Token::new(
					TokenType::Integer,
					Some(Literal::Number(200)),
					1,
					19,
					3,
				))),
			),
		),
	)
}

#[test]
fn test_create_expression_binary_number_addition() {
	create_parser_test(
		"2 + 7",
		Expression(Binary(
			Box::new(Literal(Token::new(
				TokenType::Integer,
				Some(Literal::Number(2)),
				1,
				0,
				1,
			))),
			Token::new(TokenType::Plus, None, 1, 2, 1),
			Box::new(Literal(Token::new(
				TokenType::Integer,
				Some(Literal::Number(7)),
				1,
				4,
				1,
			))),
		)),
	)
}

#[test]
fn test_create_nested_expression_binary_number_addition() {
	create_parser_test(
		"2 + 7 + 8",
		Expression(Binary(
			Box::new(Binary(
				Box::new(Literal(Token::new(
					TokenType::Integer,
					Some(Literal::Number(2)),
					1,
					0,
					1,
				))),
				Token::new(TokenType::Plus, None, 1, 2, 1),
				Box::new(Literal(Token::new(
					TokenType::Integer,
					Some(Literal::Number(7)),
					1,
					4,
					1,
				))),
			)),
			Token::new(TokenType::Plus, None, 1, 6, 1),
			Box::new(Literal(Token::new(
				TokenType::Integer,
				Some(Literal::Number(8)),
				1,
				8,
				1,
			))),
		)),
	)
}

#[test]
fn test_create_nested_expression_binary_number_addition_and_multiplication() {
	create_parser_test(
		"2 + 7 * 8",
		Expression(Binary(
			Box::new(Literal(Token::new(
				TokenType::Integer,
				Some(Literal::Number(2)),
				1,
				0,
				1,
			))),
			Token::new(TokenType::Plus, None, 1, 2, 1),
			Box::new(Binary(
				Box::new(Literal(Token::new(
					TokenType::Integer,
					Some(Literal::Number(7)),
					1,
					4,
					1,
				))),
				Token::new(TokenType::Multiply, None, 1, 6, 1),
				Box::new(Literal(Token::new(
					TokenType::Integer,
					Some(Literal::Number(8)),
					1,
					8,
					1,
				))),
			)),
		)),
	)
}

#[test]
fn test_create_nested_expression_binary_identifier_addition() {
	create_parser_test(
		"foo + bar + baz",
		Expression(Binary(
			Box::new(Binary(
				Box::new(Identifier(Token::new(
					TokenType::Identifier,
					Some(Literal::String("foo".to_string())),
					1,
					0,
					3,
				))),
				Token::new(TokenType::Plus, None, 1, 4, 1),
				Box::new(Identifier(Token::new(
					TokenType::Identifier,
					Some(Literal::String("bar".to_string())),
					1,
					6,
					3,
				))),
			)),
			Token::new(TokenType::Plus, None, 1, 10, 1),
			Box::new(Identifier(Token::new(
				TokenType::Identifier,
				Some(Literal::String("baz".to_string())),
				1,
				12,
				3,
			))),
		)),
	)
}

#[test]
fn test_create_expression_unary_number() {
	create_parser_test(
		"-1",
		Expression(Unary(
			Token::new(TokenType::Minus, None, 1, 0, 1),
			Box::new(Literal(Token::new(
				TokenType::Integer,
				Some(Literal::Number(1)),
				1,
				1,
				1,
			))),
		)),
	)
}

#[test]
fn test_create_expression_unary_boolean() {
	create_parser_test(
		"!true",
		Expression(Unary(
			Token::new(TokenType::Not, None, 1, 0, 1),
			Box::new(Literal(Token::new(
				TokenType::Boolean,
				Some(Literal::Boolean(true)),
				1,
				1,
				4,
			))),
		)),
	)
}

#[test]
fn test_create_expression_unary_in_binary_numbers() {
	create_parser_test(
		"2 + 7 + -8",
		Expression(Binary(
			Box::new(Binary(
				Box::new(Literal(Token::new(
					TokenType::Integer,
					Some(Literal::Number(2)),
					1,
					0,
					1,
				))),
				Token::new(TokenType::Plus, None, 1, 2, 1),
				Box::new(Literal(Token::new(
					TokenType::Integer,
					Some(Literal::Number(7)),
					1,
					4,
					1,
				))),
			)),
			Token::new(TokenType::Plus, None, 1, 6, 1),
			Box::new(Unary(
				Token::new(TokenType::Minus, None, 1, 8, 1),
				Box::new(Literal(Token::new(
					TokenType::Integer,
					Some(Literal::Number(8)),
					1,
					9,
					1,
				))),
			)),
		)),
	)
}

#[test]
fn test_create_failed_expressions() {
	create_fail_statement_test("2=7 + -8");
	create_fail_statement_test("2 + () 8");
	create_fail_statement_test("2 - ) + -8");
	create_fail_statement_test("2 + (7 + -8");
	create_fail_statement_test("2 *** 8");
}
