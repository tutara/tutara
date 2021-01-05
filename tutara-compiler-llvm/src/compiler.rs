use super::operation::Operation;
use super::scope::ScopeContext;
use crate::Scope;
use inkwell::{builder::Builder, context::Context, module::Module, values::FunctionValue};
use tutara_interpreter::{parser::Parser, Analyzer, Error, Statement};

pub struct Compiler<'a> {
	pub(super) context: &'a Context,
	pub(super) module: Module<'a>,
	pub(super) builder: Builder<'a>,
	pub(super) analyzer: Analyzer,
	pub(super) scope: Vec<Scope<'a>>,
}

impl Compiler<'_> {
	pub fn compile<'b>(&mut self, parser: Parser<'b>) -> Result<FunctionValue, Error> {
		let fun_type = self.context.f64_type().fn_type(&[], false);
		let fun = self.module.add_function("main", fun_type, None);
		let body = self.context.append_basic_block(fun, "entry");
		self.builder.position_at_end(body);
		self.scope.push(Scope::new(ScopeContext::Main));

		for result in parser {
			match result {
				Ok(statement) => {
					if let Operation::Return(_) = self.evaluate_statement(statement)? {
						match self.module.verify() {
							Ok(_) => return Ok(fun),
							Err(err) => return Err(Error::new_compiler_error(err.to_string())),
						}
					}
				}
				Err(error) => return Err(error),
			};
		}

		Err(Error::new_compiler_error(
			"No return statement found in script".to_string(),
		))
	}

	pub fn evaluate_statement(&mut self, statement: Statement) -> Result<Operation, Error> {
		use Statement::*;

		let analyzed_statement = self.analyzer.analyze(statement)?;

		match analyzed_statement {
			Break => self.evaluate_break(),
			Continue => self.evaluate_continue(),
			While(condition, body) => self.evaluate_while(condition, body),
			If(condition, true_branch, false_branch) => {
				self.evaluate_if(condition, true_branch, false_branch)
			}
			Expression(expression) => self.evaluate_expression(expression),
			Declaration(_mutability, _type_specification, expression) => {
				self.evaluate_declaration(expression)?;
				Ok(Operation::NoOp)
			}
			Body(statements) => self.evaluate_body(statements),
			Return(expression) => self.evaluate_return(expression),
			Comment(_) => Ok(Operation::NoOp),
			Function(r#type, identifier, parameters, body) => {
				self.evaluate_function(identifier, r#type, parameters, body)
			}
			For(_identifier, _iterable, _body) => Err(Error::new_compiler_error(
				"Unsupported statement: for".to_string(),
			)),
			// Statements converted by analyzer
			Loop(_body) => Err(Error::new_compiler_error(
				"Unexpected statement: loop".to_string(),
			)),
		}
	}
}
