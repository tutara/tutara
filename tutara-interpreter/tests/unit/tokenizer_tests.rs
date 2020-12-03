use tutara_interpreter::{Literal, Token, TokenType, Tokenizer};

#[test]
fn test_create_tokenizer() {
	let tokenizer = Tokenizer::new("script");
	let tokens: Result<Vec<Token>, tutara_interpreter::Error> = tokenizer.collect();

	assert_eq!(1, tokens.unwrap().len())
}

fn create_token_test(input: &str, expected_token: Token) {
	let mut tokenizer = Tokenizer::new(input);
	let given_token = tokenizer.next().unwrap().unwrap();
	assert_eq!(expected_token, given_token);
}

fn create_fail_token_test(input: &str) {
	let mut tokenizer = Tokenizer::new(input);
	let given_token = tokenizer.next().unwrap();
	assert!(given_token.is_err());
}

#[test]
fn test_create_number() {
	create_token_test(
		"1000",
		Token::new(TokenType::Integer, Some(Literal::Number(1000)), 1, 0, 4),
	)
}

#[test]
fn test_create_invalid_number() {
	create_fail_token_test("1000000000000000000000")
}

#[test]
fn test_create_invalid_token() {
	create_fail_token_test("@");
	create_fail_token_test("`");
	create_fail_token_test("~");
}


#[test]
fn test_create_string() {
	create_token_test(
		"'string'",
		Token::new(
			TokenType::String,
			Some(Literal::String("string".to_string())),
			1,
			0,
			8,
		),
	)
}

#[test]
fn test_create_identifier() {
	create_token_test(
		"foo",
		Token::new(
			TokenType::Identifier,
			Some(Literal::String("foo".to_string())),
			1,
			0,
			3,
		),
	)
}
