use std::env;
use std::fs;
use std::io;

mod tokenizer;
use tokenizer::Tokenizer;

fn main() {
	// Read command line arguments
	let args: Vec<String> = env::args().collect();

	let source: String;

	match args.get(1) {
		// Read from argument
		Some(arg) => {
			if arg == "--file" {
				let file = args.get(2);
				if file.is_none() {
					println!("ERROR: Please specify a file: --file [path]");
					return;
				}

				let file_path = file.unwrap();
				println!("Reading from file {}", file_path);
				source = fs::read_to_string(file_path)
					.expect("Could not read file")
					.to_string();
			} else {
				println!("Reading from argument");
				source = arg.to_string();
			}
		}
		// Read from stdin
		None => {
			println!("Reading from stdin");
			let mut input = String::new();
			print!("> ");

			io::stdin().read_line(&mut input).expect("Failed to read");
			source = input
		}
	};

	println!("# Input");
	println!("{}", source);
	println!();

	// Tokenize
	let mut tokenizer = Tokenizer::new(&source);
	tokenizer.tokenize();

	println!("# Tokens");
	println!();

	let mut last_line = 1;
	for token in tokenizer.tokens.iter() {
		let mut literal_val = "".to_string();

		if token.literal.is_some() {
			literal_val =
				String::new() + "literal=" + &token.literal.as_ref().unwrap().to_string() + " ";
		}

		if token.line != last_line {
			println!();
			last_line = token.line;
		}

		println!(
			"{:<11}{:<30}line={:<4} column={:<4} length={:<4}",
			format!("{:?}", token.r#type),
			literal_val,
			token.line,
			token.column,
			token.length
		);
	}
}
