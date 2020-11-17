use std::path::PathBuf;
use std::fs;
use tutara_interpreter::{Statement, Tokenizer, Parser};

fn test_statements(name: &str) {
	let mut script_path: PathBuf = ["tests", "scripts", name, name].iter().collect();
	script_path.set_extension("ttr");

	let mut statements_path: PathBuf = ["tests", "scripts", name, name].iter().collect();
	statements_path.set_extension("statements.json");

	let script = fs::read_to_string(script_path).expect("Could not read test script");
	let source = fs::read_to_string(statements_path).expect("Could not read statements");
	let statements = serde_json::from_str::<Vec<Statement>>(&source).expect("Could not parse statements");

	let parser = Parser::new(Tokenizer::new(&script).peekable());

	for (index, current) in parser.enumerate() {
		match current {
			Ok(current) => match statements.get(index) {
				Some(expected) => assert_eq!(*expected, current), 
				None => panic!("Could not find statement for index {}", index)
			}
			Err(err) => panic!("{:?}", err),
		}
	}
}

#[test]
fn test_vat_function_statements() {
	test_statements("vat-function")
}

#[test]
fn test_primitives_statements() {
	test_statements("primitives")
}

#[test]
fn test_variables_statements() {
	test_statements("variables")
}

#[test]
fn test_operators_statements() {
	test_statements("operators")
}

#[test]
fn test_logic_statements() {
	test_statements("logic")
}

#[test]
fn test_assignment_statements() {
	test_statements("assignment")
}

#[test]
fn test_functions_statements() {
	test_statements("functions")
}

#[test]
fn test_choice_statements() {
	test_statements("choice")
}
