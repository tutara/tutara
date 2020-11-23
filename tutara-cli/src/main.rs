extern crate tutara_interpreter;

use std::{env, fs};
use std::io;
use std::io::Result;
use std::io::Write;
use tutara_interpreter::{Parser, Token, TokenType, Tokenizer};
use tutara_compiler_llvm::Evaluator;

use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};


fn color_for_token(token: &Token) -> Option<Color> {
	// Colors based on Nord color palette
	match token.r#type {
		TokenType::Integer => Some(Color::Rgb(94, 129, 172)),
		TokenType::String => Some(Color::Rgb(163, 190, 140)),
		TokenType::Boolean => Some(Color::Rgb(208, 135, 1)),
		TokenType::Val => Some(Color::Rgb(208, 135, 1)),
		TokenType::Var => Some(Color::Rgb(208, 135, 1)),
		TokenType::Identifier => Some(Color::Rgb(235, 203, 1)),
		TokenType::Plus => Some(Color::Rgb(180, 142, 173)),
		TokenType::Minus => Some(Color::Rgb(180, 142, 173)),
		TokenType::Multiply => Some(Color::Rgb(180, 142, 173)),
		TokenType::Division => Some(Color::Rgb(180, 142, 173)),
		TokenType::Exponentiation => Some(Color::Rgb(180, 142, 173)),
		TokenType::Modulo => Some(Color::Rgb(180, 142, 173)),
		TokenType::Not => Some(Color::Rgb(180, 142, 173)),
		TokenType::Equal => Some(Color::Rgb(180, 142, 173)),
		TokenType::NotEqual => Some(Color::Rgb(180, 142, 173)),
		TokenType::Greater => Some(Color::Rgb(180, 142, 173)),
		TokenType::Lesser => Some(Color::Rgb(180, 142, 173)),
		TokenType::GreaterOrEqual => Some(Color::Rgb(180, 142, 173)),
		TokenType::LesserOrEqual => Some(Color::Rgb(180, 142, 173)),
		TokenType::And => Some(Color::Rgb(180, 142, 173)),
		TokenType::Or => Some(Color::Rgb(180, 142, 173)),
		TokenType::If => Some(Color::Rgb(208, 135, 1)),
		TokenType::Else => Some(Color::Rgb(208, 135, 1)),
		TokenType::Match => Some(Color::Rgb(208, 135, 1)),
		TokenType::Function => Some(Color::Rgb(208, 135, 1)),
		TokenType::Return => Some(Color::Rgb(208, 135, 1)),
		TokenType::Separator => Some(Color::Rgb(236, 239, 244)),
		TokenType::Loop => Some(Color::Rgb(180, 142, 173)),
		TokenType::While => Some(Color::Rgb(180, 142, 173)),
		TokenType::For => Some(Color::Rgb(180, 142, 173)),
		TokenType::Break => Some(Color::Rgb(180, 142, 173)),
		TokenType::In => Some(Color::Rgb(180, 142, 173)),
		TokenType::OpenParenthesis => Some(Color::Rgb(143, 188, 187)),
		TokenType::CloseParenthesis => Some(Color::Rgb(143, 188, 187)),
		TokenType::OpenCurlyBracket => Some(Color::Rgb(143, 188, 187)),
		TokenType::CloseCurlyBracket => Some(Color::Rgb(143, 188, 187)),
		TokenType::Assign => Some(Color::Rgb(236, 239, 244)),
		TokenType::AssignPlus => Some(Color::Rgb(236, 239, 244)),
		TokenType::AssignMinus => Some(Color::Rgb(236, 239, 244)),
		TokenType::AssignMultiply => Some(Color::Rgb(236, 239, 244)),
		TokenType::AssignDivision => Some(Color::Rgb(236, 239, 244)),
		TokenType::AssignExponentiation => Some(Color::Rgb(236, 239, 244)),
		TokenType::AssignModulo => Some(Color::Rgb(236, 239, 244)),
		TokenType::Specifier => Some(Color::Rgb(236, 239, 244)),
		TokenType::Comment => Some(Color::Rgb(216, 222, 233)),
		TokenType::Dot => Some(Color::Rgb(236, 239, 244)),
		TokenType::Arrow => Some(Color::Rgb(236, 239, 244)),
	}
}

fn run(src: String) -> Result<()> {
	let mut stdout = StandardStream::stdout(ColorChoice::Auto);
	let tokenizer = Tokenizer::new(&src);
	let mut line_index = 1;
	let mut column_index = 0;

	let mut lines = src.lines();
	let mut line = lines.next().unwrap();

	for result in tokenizer {
		if let Err(err) = result {
			writeln!(&mut stdout, "{}", err)?;
			break;
		} else if let Ok(token) = result {
			stdout.set_color(ColorSpec::new().set_fg(color_for_token(&token)))?;

			while token.line > line_index {
				line_index += 1;
				column_index = 0;
				line = lines.next().unwrap();
				writeln!(&mut stdout)?;
			}

			if column_index < token.column {
				let diff = token.column - column_index;
				write!(&mut stdout, "{}", " ".repeat(diff as usize))?;
				column_index += diff;
			}

			write!(
				&mut stdout,
				"{}",
				line.chars()
					.skip(token.column as usize)
					.take(token.length as usize)
					.collect::<String>()
			)?;
			column_index += token.length;
		}
	}

	// Reset colors
	stdout.set_color(&ColorSpec::new())?;
	writeln!(&mut stdout)?;

	// Evaluate
	match Evaluator::evaluate(Parser::new(Tokenizer::new(&src).peekable())) {
		Ok(output) => {
			write!(&mut stdout, "Output: ")?;
			writeln!(&mut stdout, "{}", output)?;
		},
		Err(err) => {
			writeln!(&mut stdout, "{}", err)?;
		}
	};
	
	writeln!(&mut stdout)
}

fn interactive_mode() {
	println!("Initialized Tutara interactive mode. Use \".file [path]\" to read files or \".exit\" to leave.");
	println!();

	let mut buffer = Vec::new();

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
			buffer.push(input.clone());

			if input.starts_with("return") {
				run(buffer.join("")).expect("Could not write the desired output");

				buffer.clear();
			}
		}
	}
}

fn run_file(path: &str) {
	println!("Reading from file {}", path);
	let source = fs::read_to_string(path).expect("Could not read file");

	run(source).expect("Could not write the desired output");
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
				run(arg.to_string()).expect("Could not write the desired output");
			}
		}
		// Read from stdin
		None => {
			interactive_mode();
		}
	};
}
