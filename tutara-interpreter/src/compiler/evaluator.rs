use std::path::Path;

use inkwell::context::Context;

use crate::{Compiler, Error, Parser};

pub struct Evaluator {
	
}

impl Evaluator {
	pub fn evaluate<'a, 'b>(parser: Parser<'b>) -> Result<f64, Error> {
		let context = Context::create();
		let module = context.create_module("init");
		let builder = context.create_builder();

		let compiler = Compiler {
			context: &context,
			module,
			builder,
		};

		match compiler.compile(parser) {
			Ok(fun) => {
				let engine = compiler.module.create_jit_execution_engine(inkwell::OptimizationLevel::None).unwrap();
				
				return unsafe {
					Ok(engine.run_function(fun, &[]).as_float(&context.f64_type()))
				}
			}
			Err(err) => Err(err),
		}
	}

	pub fn save<'a, 'b>(parser: Parser<'b>, path: &Path) -> Option<Error> {
		let context = Context::create();
		let module = context.create_module("init");
		let builder = context.create_builder();

		let compiler = Compiler {
			context: &context,
			module,
			builder,
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
