use tutara_interpreter::{Token, Tokenizer};

#[test]
fn test_create_tokenizer() {
	let tokenizer = Tokenizer::new("script");
	let tokens: Result<Vec<Token>, tutara_interpreter::Error> = tokenizer.collect();
	assert_eq!(1, tokens.unwrap().len())
}
