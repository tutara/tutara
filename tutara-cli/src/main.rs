extern crate tutara_interpreter;

use tutara_interpreter::Analyzer;
use std::env;
use std::io;
use std::io::{Read, Write};
use std::result::Result;
use tutara_compiler_llvm::Evaluator;
use tutara_interpreter::{Parser, Statement, Token, TokenType, Tokenizer};

use clap::{crate_version, App, AppSettings, Arg, ArgSettings};
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

fn run(input: &str, output: &str, format: &str) -> Result<(), std::io::Error> {
	let mut input_read: Box<dyn Read> = if input == "-" {
		Box::new(std::io::stdin())
	} else {
		match std::fs::File::open(&input) {
			Ok(file) => Box::new(file),
			Err(err) => {
				println!("File could not be read. Are you sure it exists?");
				println!("{}", err);

				return Ok(());
			}
		}
	};

	let mut output_write: Box<dyn Write> = if output == "-" {
		Box::new(std::io::stdout())
	} else {
		match std::fs::File::create(&output) {
			Ok(file) => Box::new(file),
			Err(err) => {
				println!("File could not be written to: {}", err);
				return Ok(());
			}
		}
	};

	match format {
		"highlight" => if output == "-" {
			highlight(&mut input_read)
		} else {
			writeln!(output_write, "Highlight output can not be exported to files.")
		},
		"tokens" => tokenize(&mut input_read, &mut output_write),
		"statements" => parse(&mut input_read, &mut output_write),
		"analyzed_statements" => analyze(&mut input_read, &mut output_write),
		"result" => evaluate(&mut input_read, &mut output_write),
		_ => unreachable!(),
	}
}

fn highlight(input: &mut dyn std::io::Read) -> Result<(), std::io::Error> {	
	let mut output = StandardStream::stdout(ColorChoice::Auto);
	let mut src = String::new();
	input.read_to_string(&mut src)?;

	let tokenizer = Tokenizer::new(&src);
	let mut line_index = 1;
	let mut column_index = 0;

	let mut lines = src.lines();
	let mut line = lines.next().unwrap();

	for result in tokenizer {
		if let Err(err) = result {
			writeln!(output, "{}", err)?;
			break;
		} else if let Ok(token) = result {
			output.set_color(ColorSpec::new().set_fg(color_for_token(&token)))?;

			while token.line > line_index {
				line_index += 1;
				column_index = 0;
				line = lines.next().unwrap();
				writeln!(output)?;
			}

			if column_index < token.column {
				let diff = token.column - column_index;
				write!(output, "{}", " ".repeat(diff as usize))?;
				column_index += diff;
			}

			write!(
				output,
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
	output.set_color(&ColorSpec::new())?;

	writeln!(output)
}

fn tokenize(input: &mut dyn std::io::Read, output: &mut dyn Write) -> Result<(), std::io::Error> {
	let mut src = String::new();
	input.read_to_string(&mut src)?;

	let tokenizer = Tokenizer::new(&src);
	let tokens: Result<Vec<Token>, tutara_interpreter::Error> = tokenizer.collect();

	match tokens {
		Ok(tokens) => writeln!(output, "{}", serde_json::to_string_pretty(&tokens).unwrap()),
		Err(err) => writeln!(output, "Error: {}", err),
	}
}

fn parse(input: &mut dyn std::io::Read, output: &mut dyn Write) -> Result<(), std::io::Error> {
	let mut src = String::new();
	input.read_to_string(&mut src)?;

	let tokenizer = Tokenizer::new(&src);
	let parser = Parser::new(tokenizer.peekable());
	let statements: Result<Vec<Statement>, tutara_interpreter::Error> = parser.collect();

	match statements {
		Ok(statements) => writeln!(output, "{}", serde_json::to_string_pretty(&statements).unwrap()),
		Err(err) => writeln!(output, "Error: {}", err),
	}
}

fn analyze(input: &mut dyn std::io::Read, output: &mut dyn Write) -> Result<(), std::io::Error> {
	let mut src = String::new();
	input.read_to_string(&mut src)?;

	let tokenizer = Tokenizer::new(&src);
	let parser = Parser::new(tokenizer.peekable());
	let analyzer = Analyzer::new(parser);
	let statements: Result<Vec<Statement>, tutara_interpreter::Error> = analyzer.collect();

	match statements {
		Ok(statements) => writeln!(output, "{}", serde_json::to_string_pretty(&statements).unwrap()),
		Err(err) => writeln!(output, "Error: {}", err),
	}
}

fn evaluate(input: &mut dyn std::io::Read, output: &mut dyn Write) -> Result<(), std::io::Error> {
	let mut src = String::new();
	input.read_to_string(&mut src)?;

	let tokenizer = Tokenizer::new(&src);
	let parser = Parser::new(tokenizer.peekable());
	let evaluation = Evaluator::evaluate(parser);

	match evaluation {
		Ok(evaluation) => writeln!(output, "{}", evaluation),
		Err(err) => writeln!(output, "Error: {}", err),
	}
}

fn interactive_mode() -> Result<(), std::io::Error> {
	println!("Initialized Tutara interactive mode. Use \".exit\" to leave.");
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
		} else {
			buffer.push(input.clone());

			if input.starts_with("return") {
				evaluate(&mut buffer.join("").as_bytes(), &mut io::stdout())?;

				buffer.clear();
			}
		}
	}

	Ok(())
}

fn main() -> Result<(), std::io::Error> {
	let matches = App::new("Tutara")
		.version(crate_version!())
		.setting(AppSettings::ColoredHelp)
		.setting(AppSettings::SubcommandRequiredElseHelp)
		.setting(AppSettings::GlobalVersion)
		.setting(AppSettings::HelpRequired)
		.setting(AppSettings::VersionlessSubcommands)
		.subcommand(
			App::new("run")
				.about("Run a script")
				.arg(
					Arg::new("input")
						.short('i')
						.about("Set input file or '-' to use STDIN")
						.takes_value(true)
						.required(true),
				)
				.arg(
					Arg::new("output")
						.short('o')
						.about("Set output file or '-' to use STDOUT")
						.takes_value(true)
						.default_value("-"),
				)
				.arg(
					Arg::new("format")
						.short('f')
						.about("Set format")
						.setting(ArgSettings::Hidden)
						.takes_value(true)
						.possible_values(&["highlight", "tokens", "statements", "analyzed_statements" , "result"])
						.default_value("result"),
				),
		)
		.subcommand(
			App::new("generate-test")
				.about("Generate JSON files for a given test")
				.setting(AppSettings::Hidden)
				.arg(
					Arg::new("NAME")
						.about("Sets the name of the test")
						.multiple(true)
						.required(true),
				),
		)
		.subcommand(App::new("interactive").about("Start interactive mode"))
		.get_matches();

	match matches.subcommand() {
		Some(("run", run_matches)) => {
			let input = run_matches.value_of("input").unwrap();
			let output = run_matches.value_of("output").unwrap();
			let format = run_matches.value_of("format").unwrap();

			run(input, output, format)
		}
		Some(("interactive", _)) => interactive_mode(),
		_ => unreachable!(),
	}
}
