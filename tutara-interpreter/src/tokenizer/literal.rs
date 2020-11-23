use std::string::String;
use std::fmt::{self, Debug};

use super::Literal::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Eq, Serialize, Deserialize)]
pub enum Literal {
	Number(u32),
	String(String),
	Boolean(bool)
}

impl fmt::Display for Literal {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}

impl PartialEq for Literal {
	fn eq(&self, other: &Literal) -> bool {
		match *self {
			Number(ref a) => match *other {
				Number(ref b) => a.eq(b),
				_ => false,
			},
			String(ref a) => match *other {
				String(ref b) => a.eq(b),
				_ => false,
			},
			Boolean(ref a) => match *other {
				Boolean(ref b) => a.eq(b),
				_ => false,
			},
		}
	}
}
