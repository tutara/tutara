extern crate tutara_interpreter;

use std::env;
use std::fs;
use std::io;
use std::io::Write;

use tutara_interpreter::Tokenizer;

fn run(src: String, print_input: bool) {
	if print_input {
		println!("# Input");
		println!("{}", src);
		println!();
	}
	
	let tokenizer = Tokenizer::new(&src);

	println!("# Tokens");
	println!();
	
	let mut last_line = 1;
	for result in tokenizer {
		if let Err(err) = result {
			println!("Error at line {} on column {}: {}", err.line, err.column, err.message);
		} else if let Ok(token) = result {
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
}

fn interactive_mode() {
	println!("Initialized Tutara interactive mode. Use \".file [path]\" to read files or \".exit\" to leave.");
	println!();
	
	loop {
		let mut input = String::new();
		print!("> ");
		io::stdout().flush().expect("Failed to write");
		io::stdin().read_line(&mut input).expect("Failed to read");
	
		if input.starts_with(".exit") {
			println!("Exiting interactive mode");
			break;
		} else if input.starts_with(".file") {
			let parts = input.split_whitespace().nth(1);
			
			if let Some(path) = parts {
				run_file(&path.to_string());
			} else {
				println!("Invalid path. Syntax: .file [path]");	
			}
		} else {
			run(input, false);
		}
	}
}

fn run_file(path: &String) {
	println!("Reading from file {}", path);
	
	let source = fs::read_to_string(path).expect("Could not read file").to_string();

	run(source, true);
}

fn main() {
	// Read command line arguments
	let args: Vec<String> = env::args().collect();

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
				run_file(file_path);
			} else {
				println!("Reading from argument");
				run(arg.to_string(), true);
			}
		}
		// Read from stdin
		None => {
			interactive_mode();
		}
	};
}
