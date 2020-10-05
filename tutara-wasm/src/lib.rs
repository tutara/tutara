use wasm_bindgen::prelude::*;

use tutara_interpreter::Tokenizer;
use tutara_interpreter::Token;

#[wasm_bindgen]
pub struct LocalTokenizer {
	tokenizer: &'static mut Tokenizer<'static>
}

#[wasm_bindgen]
pub struct LocalToken {
	// pub r#type: TokenType,
	// pub literal: Option<Literal>,
	pub line: u32,
	pub column: u32,
	pub length: u32,
}

impl LocalToken {
	pub fn from_token(token: Token) -> LocalToken {
		return LocalToken {
			line: token.line,
			column: token.column,
			length: token.length,
		}
	}
}

#[wasm_bindgen]
pub fn create_tokenizer(source: &str) -> LocalTokenizer {
	return LocalTokenizer {
		tokenizer: Box::leak(Box::new(Tokenizer::new(source)))
	};
}

#[wasm_bindgen]
pub fn next_token(localTokenizer: LocalTokenizer) -> Option<LocalToken> {
	let tokenizer = localTokenizer.tokenizer;
	let result = tokenizer.next();

	if result.is_none() {
		return None;
	}

	let token = result.unwrap();

	// if token.is_err() {
	// 	return None;
	// }

	return Some(LocalToken::from_token(token.unwrap()));
}
