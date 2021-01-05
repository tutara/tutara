use std::fs;
use std::path::PathBuf;
use tutara_compiler_llvm::Evaluator;
use tutara_interpreter::{parser::Parser, Tokenizer};

fn test_compiler(name: &str, result: f64) {
	let mut script_path: PathBuf = ["tests", "scripts", name].iter().collect();
	script_path.set_extension("ttr");

	let script = fs::read_to_string(script_path).expect("Could not read test script");

	let tokenizer = Tokenizer::new(&script);
	let parser = Parser::new(tokenizer.peekable());
	let evaluation = Evaluator::evaluate(parser);

	assert_eq!(result, evaluation.unwrap())
}

#[test]
fn test_math_function() {
	test_compiler("math_function", 3.0);
}

#[test]
fn test_continue() {
	test_compiler("continue", 5.0);
}

#[test]
fn test_break() {
	test_compiler("break", 9.0);
}

#[test]
fn test_if_else() {
	test_compiler("if_else", 10.0);
}

#[test]
fn test_assignment_operations() {
	test_compiler("assignment_operations", 3137.0);
}

#[test]
fn test_logic_function() {
	test_compiler("logic_function", 1.0);
}
