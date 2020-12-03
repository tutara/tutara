use criterion::{criterion_group, criterion_main, Criterion};
use std::fs;
use std::path::PathBuf;
use tutara_interpreter::{Error, Parser, Statement, Token, Tokenizer};

fn tokenizer_benchmark(criterion: &mut Criterion) {
	criterion.bench_function("tokenizer_benchmark", |bencher| {
		bencher.iter(|| {
			let mut script_path: PathBuf = ["benches", "benchie"].iter().collect();
			script_path.set_extension("ttr");
			let script = fs::read_to_string(script_path).expect("Could not read test script");
			let tokenizer = Tokenizer::new(&script);
			let _tokens: Result<Vec<Token>, Error> = tokenizer.collect();
		})
	});
}

fn parser_benchmark(criterion: &mut Criterion) {
	criterion.bench_function("parser_benchmark", |bencher| {
		bencher.iter(|| {
			let mut script_path: PathBuf = ["benches", "benchie"].iter().collect();
			script_path.set_extension("ttr");
			let script = fs::read_to_string(script_path).expect("Could not read test script");
			let parser = Parser::new(Tokenizer::new(&script).peekable());
			let _tokens: Result<Vec<Statement>, Error> = parser.collect();
		})
	});
}

criterion_group!(benches, tokenizer_benchmark, parser_benchmark,);
criterion_main!(benches);
