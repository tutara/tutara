use wasm_bindgen::prelude::*;

use tutara_interpreter::Token;
use tutara_interpreter::Tokenizer;
use tutara_interpreter::{Parser, Statement};

#[wasm_bindgen]
pub struct Source {
    text: String,
    tokens: Option<Vec<Token>>,
    statements: Option<Vec<Statement>>,
}

#[wasm_bindgen]
impl Source {
    #[wasm_bindgen]
    pub fn get_tokens(&mut self) -> JsValue {
        if self.tokens.is_none() {
            let tokenizer = Tokenizer::new(&self.text);
            self.tokens = Some(
                tokenizer
                    .filter(|token| token.is_ok())
                    .map(|token| token.unwrap())
                    .collect(),
            );
        }

        JsValue::from_serde(&self.tokens).unwrap()
    }

    #[wasm_bindgen]
    pub fn get_statements(&mut self) -> JsValue {
        if self.statements.is_none() {
            let parser = Parser::new(Tokenizer::new(&self.text).peekable());
            self.statements = Some(
                parser
                    .filter(|statement| statement.is_ok())
                    .map(|statement| statement.unwrap())
                    .collect(),
            );
        }

        JsValue::from_serde(&self.statements).unwrap()
    }
}

#[wasm_bindgen]
pub fn from_source(source: &str) -> Source {
    Source {
        text: source.to_string(),
        tokens: None,
        statements: None,
    }
}
