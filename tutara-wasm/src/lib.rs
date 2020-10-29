use wasm_bindgen::prelude::*;

use tutara_interpreter::Literal;
use tutara_interpreter::Token;
use tutara_interpreter::TokenType;
use tutara_interpreter::Tokenizer;

#[wasm_bindgen]
pub struct LocalToken {
	token_type: TokenType,
	literal: Option<Literal>,
	pub line: u32,
	pub column: u32,
	pub length: u32,
}

impl LocalToken {
	pub fn from_token(token: Token) -> LocalToken {
		LocalToken {
			token_type: token.r#type,
			literal: token.literal,
			line: token.line,
			column: token.column,
			length: token.length,
		}
	}
}

#[wasm_bindgen]
impl LocalToken {
	#[wasm_bindgen(getter)]
	pub fn token_type(&self) -> String {
		self.token_type.to_string()
	}
	#[wasm_bindgen(getter)]
	pub fn literal(&self) -> JsValue {
		if let Some(literal) = &self.literal {
			match literal {
				Literal::Number(n) => JsValue::from_f64(*n as f64),
				Literal::String(s) => JsValue::from_str(s),
			}
		} else {
			JsValue::null()
		}
	}
}

#[wasm_bindgen(module = "/token-set.js")]
extern "C" {
	pub type TokenSet;

	#[wasm_bindgen(constructor)]
	fn new() -> TokenSet;

	#[wasm_bindgen(method)]
	fn append(this: &TokenSet, token: LocalToken);
}

#[wasm_bindgen]
pub fn get_tokens(source: &str) -> TokenSet {
	let mut tokenizer = Tokenizer::new(source);
	let token_set = TokenSet::new();

	loop {
		let result = tokenizer.next();

		if result.is_none() {
			break;
		}

		let token = result.unwrap();
		token_set.append(LocalToken::from_token(token.unwrap()));
	}

	token_set
}
