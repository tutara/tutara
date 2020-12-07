use std::{collections::HashMap, path::Path};
use inkwell::context::Context;
use tutara_interpreter::{Error, Analyzer};
use crate::Compiler;

pub struct Evaluator {
	
}

impl Evaluator {
	pub fn evaluate(analyser: Analyzer<'_>) -> Result<f64, Error> {
		let context = Context::create();
		let module = context.create_module("init");
		let builder = context.create_builder();

		let mut compiler = Compiler {
			context: &context,
			module,
			builder,
			variables: HashMap::new()
		};

		let engine = compiler.module.create_jit_execution_engine(inkwell::OptimizationLevel::None).unwrap();
		match compiler.compile(analyser) {
			Ok(fun) => unsafe {
				Ok(engine.run_function(fun, &[]).as_float(&context.f64_type()))
			},
			Err(err) => Err(err),
		}
	}

	pub fn save(analyser: Analyzer<'_>, path: &Path) -> Option<Error> {
		let context = Context::create();
		let module = context.create_module("init");
		let builder = context.create_builder();

		let mut compiler = Compiler {
			context: &context,
			module,
			builder,
			variables: HashMap::new()
		};

		match compiler.compile(analyser) {
			Ok(_) => {
				compiler.module.write_bitcode_to_path(path);

				None
			},
			Err(err) => Some(err),
		}
	}
}
