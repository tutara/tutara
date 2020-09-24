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
