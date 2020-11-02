use wasm_bindgen::prelude::*;

use tutara_interpreter::Error;
use tutara_interpreter::Token;
use tutara_interpreter::Tokenizer;
use tutara_interpreter::{Parser, Statement};

#[wasm_bindgen]
pub struct Source {
    text: String,
    tokens: Option<Result<Vec<Token>, Error>>,
    statements: Option<Result<Vec<Statement>, Error>>,
}

#[wasm_bindgen]
impl Source {
    #[wasm_bindgen(catch)]
    pub fn get_tokens(&mut self) -> Result<JsValue, JsValue> {
        if self.tokens.is_none() {
            let tokenizer = Tokenizer::new(&self.text);
            self.tokens = Some(tokenizer.collect());
        }

        match &self.tokens {
            Some(Ok(tokens)) => Ok(JsValue::from_serde(&tokens).unwrap()),
            Some(Err(err)) => Err(JsValue::from_serde(&err).unwrap()),
            None => unreachable!(),
        }
    }

    #[wasm_bindgen(catch)]
    pub fn get_statements(&mut self) -> Result<JsValue, JsValue> {
        if self.statements.is_none() {
			let parser = Parser::new(Tokenizer::new(&self.text).peekable());
            self.statements = Some(parser.collect());
        }

        match &self.statements {
            Some(Ok(statements)) => Ok(JsValue::from_serde(&statements).unwrap()),
            Some(Err(err)) => Err(JsValue::from_serde(&err).unwrap()),
            None => unreachable!(),
        }
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
