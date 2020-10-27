extern crate tutara_interpreter;

use std::env;
use std::fs;
use std::io;
use std::io::Write;

use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use tutara_interpreter::{Token, Tokenizer};

fn color_for_token(token: &Token) -> Option<Color> {
	// Colors based on Nord color palette
	match token.r#type {
	    tutara_interpreter::TokenType::Integer => Some(Color::Rgb(94, 129, 172)),
	    tutara_interpreter::TokenType::String => Some(Color::Rgb(163, 190, 140)),
	    tutara_interpreter::TokenType::True => Some(Color::Rgb(208, 135, 1)),
	    tutara_interpreter::TokenType::False => Some(Color::Rgb(208, 135, 1)),
	    tutara_interpreter::TokenType::Val => Some(Color::Rgb(208, 135, 1)),
	    tutara_interpreter::TokenType::Var => Some(Color::Rgb(208, 135, 1)),
	    tutara_interpreter::TokenType::Identifier => Some(Color::Rgb(235, 203, 1)),
		tutara_interpreter::TokenType::Plus => Some(Color::Rgb(180, 142, 173)),
	    tutara_interpreter::TokenType::Minus => Some(Color::Rgb(180, 142, 173)),
	    tutara_interpreter::TokenType::Multiply => Some(Color::Rgb(180, 142, 173)),
	    tutara_interpreter::TokenType::Division => Some(Color::Rgb(180, 142, 173)),
	    tutara_interpreter::TokenType::Pow => Some(Color::Rgb(180, 142, 173)),
	    tutara_interpreter::TokenType::Modulo => Some(Color::Rgb(180, 142, 173)),
	    tutara_interpreter::TokenType::Function => Some(Color::Rgb(208, 135, 1)),
	    tutara_interpreter::TokenType::Return => Some(Color::Rgb(208, 135, 1)),
	    tutara_interpreter::TokenType::Separator => Some(Color::Rgb(236, 239, 244)),
	    tutara_interpreter::TokenType::OpenParenthesis => Some(Color::Rgb(143, 188, 187)),
	    tutara_interpreter::TokenType::CloseParenthesis => Some(Color::Rgb(143, 188, 187)),
	    tutara_interpreter::TokenType::OpenCurlyBracket => Some(Color::Rgb(143, 188, 187)),
	    tutara_interpreter::TokenType::CloseCurlyBracket => Some(Color::Rgb(143, 188, 187)),
	    tutara_interpreter::TokenType::Assign => Some(Color::Rgb(236, 239, 244)),
	    tutara_interpreter::TokenType::Specifier => Some(Color::Rgb(236, 239, 244)),
	    tutara_interpreter::TokenType::Comment => Some(Color::Rgb(216, 222, 233)),
	}
}

fn run(src: String) {
	let mut stdout = StandardStream::stdout(ColorChoice::Auto);
		
	let tokenizer = Tokenizer::new(&src);
	
	let mut line_index = 1;
	let mut column_index = 0;

	let mut lines = src.lines();
	let mut line = lines.next().unwrap();

	for result in tokenizer {
		if let Err(err) = result {
			writeln!(&mut stdout, "Error at line {} on column {}: {}", err.line, err.column, err.message);
			break;
		} else if let Ok(token) = result {
			stdout.set_color(ColorSpec::new().set_fg(color_for_token(&token)));

			while token.line > line_index {
				line_index += 1;
				column_index = 0;
				line = lines.next().unwrap();
				writeln!(&mut stdout);
			}

			if (column_index < token.column) {
				let diff = token.column - column_index;
				write!(&mut stdout, "{}", " ".repeat(diff as usize));
				column_index += diff;
			}
			
			write!(&mut stdout, "{}", line.chars().skip(token.column as usize).take(token.length as usize).collect::<String>());
			column_index += token.length;
		}
	}

	// Reset colors
	stdout.set_color(&ColorSpec::new());
	writeln!(&mut stdout);
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
			run(input);
		}
	}
}

fn run_file(path: &String) {
	println!("Reading from file {}", path);
	
	let source = fs::read_to_string(path).expect("Could not read file");

	run(source);
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
				run(arg.to_string());
			}
		}
		// Read from stdin
		None => {
			interactive_mode();
		}
	};
}
