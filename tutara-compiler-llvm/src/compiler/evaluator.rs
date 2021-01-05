use std::{path::Path};
use inkwell::context::Context;
use tutara_interpreter::{parser::Parser, Error, Analyzer};
use crate::Compiler;

pub struct Evaluator {
	
}

impl Evaluator {
	pub fn evaluate(parser: Parser<'_>) -> Result<f64, Error> {
		let context = Context::create();
		let module = context.create_module("init");
		let builder = context.create_builder();
		let analyzer = Analyzer::default();

		let mut compiler = Compiler {
			context: &context,
			module,
			builder,
			analyzer,
			scope: Vec::new()
		};

		let engine = compiler.module.create_jit_execution_engine(inkwell::OptimizationLevel::None).unwrap();
		match compiler.compile(parser) {
			Ok(fun) => unsafe {
				Ok(engine.run_function(fun, &[]).as_float(&context.f64_type()))
			},
			Err(err) => Err(err),
		}
	}

	pub fn save(parser: Parser<'_>, path: &Path) -> Option<Error> {
		let context = Context::create();
		let module = context.create_module("init");
		let builder = context.create_builder();
		let analyzer = Analyzer::default();

		let mut compiler = Compiler {
			context: &context,
			module,
			builder,
			analyzer,
			scope: Vec::new()
		};

		match compiler.compile(parser) {
			Ok(_) => {
				compiler.module.write_bitcode_to_path(path);

				None
			},
			Err(err) => Some(err),
		}
	}
}
