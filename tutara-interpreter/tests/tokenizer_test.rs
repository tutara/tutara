use std::fs;
use std::path::PathBuf;
use tutara_interpreter::{Token, Tokenizer};

fn test_tokens(name: &str) {
	let mut script_path: PathBuf = ["tests", "scripts", name, name].iter().collect();
	script_path.set_extension("ttr");

	let mut tokens_path: PathBuf = ["tests", "scripts", name, name].iter().collect();
	tokens_path.set_extension("tokens.json");

	let script = fs::read_to_string(script_path).expect("Could not read test script");
	let source = fs::read_to_string(tokens_path).expect("Could not read tokens");
	let tokens = serde_json::from_str::<Vec<Token>>(&source).expect("Could not parse tokens");

	let tokenizer = Tokenizer::new(&script);

	for (index, current) in tokenizer.enumerate() {
		match current {
			Ok(current) => match tokens.get(index) {
				Some(expected) => assert_eq!(*expected, current), 
				None => panic!("Could not find token for index {}", index)
			}
			Err(err) => panic!("{:?}", err),
		}
	}
}

#[test]
fn test_vat_function_tokens() {
	test_tokens("vat-function")
}

#[test]
fn test_primitives_tokens() {
	test_tokens("primitives")
}

#[test]
fn test_variables_tokens() {
	test_tokens("variables")
}

#[test]
fn test_operators_tokens() {
	test_tokens("operators")
}

#[test]
fn test_logic_tokens() {
	test_tokens("logic")
}

#[test]
fn test_assignment_tokens() {
	test_tokens("assignment")
}

#[test]
fn test_functions_tokens() {
	test_tokens("functions")
}
