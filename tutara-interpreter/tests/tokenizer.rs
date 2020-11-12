use std::fs;
use tutara_interpreter::{Token, Tokenizer};

fn test_tokens(name: &str) {
	let base = "tests\\scripts\\";

	let source = fs::read_to_string(format!("{}{}\\{}.tokens.json", base, name, name))
		.expect("Could not read tokens");
	let tokens = serde_json::from_str::<Vec<Token>>(&source).expect("Could not parse tokens");
	let script = fs::read_to_string(format!("{}{}\\{}.ttr", base, name, name))
		.expect("Could not read test script");

	let tokenizer = Tokenizer::new(&script);
	let mut count = 0;
	for result in tokenizer {
		if let Ok(result) = result {
			assert_eq!(tokens[count], result);
		} else {
			panic!("Tokenizer found an error")
		}

		count += 1;
	}
}

#[test]
fn test_vat_function() {
	test_tokens("vat-function")
}

#[test]
fn test_primitives() {
	test_tokens("primitives")
}

#[test]
fn test_variables() {
	test_tokens("variables")
}
