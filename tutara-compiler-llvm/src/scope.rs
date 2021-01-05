use inkwell::{basic_block::BasicBlock, values::PointerValue};
use std::collections::HashMap;

pub struct Scope<'a> {
	pub(crate) scope_context: ScopeContext<'a>,
	pub(crate) variables: HashMap<String, PointerValue<'a>>,
}

impl<'a> Scope<'_> {
	pub fn new(scope_context: ScopeContext<'a>) -> Scope<'a> {
		Scope {
			scope_context,
			variables: HashMap::new(),
		}
	}
}

pub enum ScopeContext<'a> {
	While(BasicBlock<'a>, BasicBlock<'a>, BasicBlock<'a>), // Body , Evaluation , Continuation
	If(BasicBlock<'a>, BasicBlock<'a>),                    // Body , Continuation
	Fun,
	Main,
}
