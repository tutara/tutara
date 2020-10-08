use std::fmt::{self, Debug};

#[derive(Debug)]
pub enum Literal {
	Number(u32),
	String(String),
}

impl fmt::Display for Literal {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}

impl Literal {
	fn to_string(&self) -> String {
		match self {
			Literal::Number(n) => n.to_string(),
			Literal::String(s) => s.clone()
		}
	}
}
