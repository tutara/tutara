use tutara_interpreter::{parser::Parser, Error, Tokenizer};
use tutara_compiler_llvm::Evaluator;

fn resolve(src: &str) -> Result<f64, Error>{
	let tokenizer = Tokenizer::new(&src);
	let parser = Parser::new(tokenizer.peekable());

	return Evaluator::evaluate(parser);
}

fn resolve_panic(src: &str){
	// This unwrap should cause a panic
	 resolve(src).unwrap();
}

#[test]
#[should_panic]
fn test_unexpected_token() {
	resolve_panic("@ return 1");
}

#[test]
#[should_panic]
fn test_no_return() {
	resolve_panic("1 + 1");
}

#[test]
#[should_panic]
fn test_invalid_function_type() {
	resolve_panic("fun: TYPE add(a: Int, b: Int){return a + b} return 1");
}

#[test]
#[should_panic]
fn test_invalid_paramater_type() {
	resolve_panic("fun: Int add(a: TYPE, b: TYPE){return a + b} return 1");
}

#[test]
#[should_panic]
fn test_invalid_unexpected_paramater_type() {
	resolve_panic("fun: Int add(a: 1, b: 1){return a + b} return 1");
}

#[test]
#[should_panic]
fn test_invalid_unexpected_function_type() {
	resolve_panic("fun: 1 add(a: Int, b: Int){return a + b} return 1");
}

#[test]
#[should_panic]
fn test_invalid_unexpected_function_name() {
	resolve_panic("fun: Int 1(a: Int, b: Int){return a + b} return 1");
}

#[test]
#[should_panic]
fn test_invalid_unexpected_paramater_name() {
	resolve_panic("fun: Int add(1: Int, 2: Int){return 1 + 2} return 1");
}

#[test]
fn test_return_1() {
	let result = resolve("return 1");
	assert_eq!(1.0, result.unwrap())
}

#[test]
fn test_return_1_plus_1() {
	let result = resolve("return 1 + 1");
	assert_eq!(2.0, result.unwrap())
}

#[test]
#[should_panic]
fn test_return_1_plus_true() {
	resolve_panic("return 1 + true");
}

#[test]
#[should_panic]
fn test_return_false_minus_true() {
	resolve_panic("return false - true");
}

#[test]
fn test_return_1_divide_2() {
	let result = resolve("return 1 / 2");
	assert_eq!(0.5, result.unwrap())
}

#[test]
fn test_return_2_minus_1() {
	let result = resolve("return 2 - 1");
	assert_eq!(1.0, result.unwrap())
}

#[test]
fn test_return_2_times_3() {
	let result = resolve("return 2 * 3");
	assert_eq!(6.0, result.unwrap())
}

#[test]
fn test_return_6_over_2() {
	let result = resolve("return 6 / 2");
	assert_eq!(3.0, result.unwrap())
}

#[test]
fn test_return_7_rest_2() {
	let result = resolve("return 7 % 2");
	assert_eq!(1.0, result.unwrap())
}

#[test]
fn test_return_2_pow_4() {
	let result = resolve("return 2 ** 4");
	assert_eq!(16.0, result.unwrap())
}

#[test]
fn test_return_minus_3_plus_8() {
	let result = resolve("return -3 + 8");
	assert_eq!(5.0, result.unwrap())
}

#[test]
fn test_bool_equals_operator() {
	let result = resolve("val a = 0 if(true == false){ a = 1 } return a");
	assert_eq!(0.0, result.unwrap())
}

#[test]
fn test_bool_not_equals_operator() {
	let result = resolve("val a = 0 if(true != false){ a = 1 } return a");
	assert_eq!(1.0, result.unwrap())
}

#[test]
fn test_bool_not_operator() {
	let result = resolve("val a = 0 if(true == !false){ a = 1 } return a");
	assert_eq!(1.0, result.unwrap())
}

#[test]
fn test_int_equals_operator() {
	let result = resolve("val a = 0 if(12 == 24){ a = 1 } return a");
	assert_eq!(0.0, result.unwrap())
}

#[test]
fn test_int_not_equals_operator() {
	let result = resolve("val a = 0 if(12 != 24){ a = 1 } return a");
	assert_eq!(1.0, result.unwrap())
}

#[test]
fn test_int_not_operator() {
	let result = resolve("val a = 0 if(!(12 == 24)){ a = 1 } return a");
	assert_eq!(1.0, result.unwrap())
}

#[test]
fn test_int_lesser_operator() {
	let result = resolve("val a = 0 if(4 < 5){ a = 1 } return a");
	assert_eq!(1.0, result.unwrap())
}

#[test]
fn test_int_greater_operator() {
	let result = resolve("val a = 0 if(4 > 5){ a = 1 } return a");
	assert_eq!(0.0, result.unwrap())
}

#[test]
fn test_int_lesser_or_equal_operator_lower() {
	let result = resolve("val a = 0 if(4 <= 5){ a = 1 } return a");
	assert_eq!(1.0, result.unwrap())
}

#[test]
fn test_int_lesser_or_equal_operator_equal() {
	let result = resolve("val a = 0 if(5 <= 5){ a = 1 } return a");
	assert_eq!(1.0, result.unwrap())
}

#[test]
fn test_int_greater_or_equal_operator_lower() {
	let result = resolve("val a = 0 if(5 >= 4){ a = 1 } return a");
	assert_eq!(1.0, result.unwrap())
}

#[test]
fn test_int_greater_or_equal_operator_equal() {
	let result = resolve("val a = 0 if(5 >= 5){ a = 1 } return a");
	assert_eq!(1.0, result.unwrap())
}

#[test]
#[should_panic]
fn test_unexpected_continue() {
	resolve_panic("continue return 1");
}

#[test]
#[should_panic]
fn test_unexpected_break() {
	resolve_panic("break return 1");
}

#[test]
#[should_panic]
fn test_invalid_while_condition() {
	resolve_panic("while(1){} return 1");
}

#[test]
#[should_panic]
fn test_invalid_if_condition() {
	resolve_panic("if(1){} return 1");
}

#[test]
#[should_panic]
fn test_invalid_declaration() {
	resolve_panic("val a = if");
}

#[test]
#[should_panic]
fn test_invalid_call() {
	resolve_panic("1()");
}
