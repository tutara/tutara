<p align="center"><img src="https://github.com/tutara/tutara-assets/raw/master/logos/logo.svg" width="200" /></p>
<h1 align="center">Tutara</h1>

<p align="center">
	<a href="https://tutara.dev/"><img src="https://img.shields.io/badge/Website-tutara.dev-orange" alt="Website" /></a>
	<a href="LICENSE"><img src="https://img.shields.io/github/license/tutara/tutara" alt="License" /></a>
	<a href="https://github.com/tutara/tutara/graphs/contributors"><img src="https://img.shields.io/github/contributors/tutara/tutara" alt="Contributers" /></a>
	<a href="code_of_conduct.md"><img src="https://img.shields.io/badge/Contributor%20Covenant-v2.0%20adopted-ff69b4.svg" alt="Contributor Covenant" /></a>
</p>

---

Tutara is an experimental programming language aimed at creating a reliable sandboxed contextual function platform. This open-source platform enables the ability to write scoped scripts. The platform can be embedded in software, or used with the standard integrations. For example, the HTTP context supports scripts to perform small actions like hashing a value or aggregating RSS feeds - there are no limits to the possibilities.

_Be aware that Tutara is still in an early stage of development._

## Contributing

Check out our [Contributing guidelines](CONTRIBUTING.md) if you'd like to help us out.

## Development

We use the official package manager for Rust, Cargo, as tool for managing dependencies, building and running the code. Visit the [Rust documentation](https://doc.rust-lang.org/cargo/) for detailed information on Cargo.

The repository hosts multiple crates. It is therefor required to switch your working directory before running the cargo commands.

### Running interactive mode

The CLI crate is an interactive command line tool where you can write code and pass it to the interpreter. The interactive mode supports text inputs and files. To use files write `.file [path]` in the command line. To exit the command line use the `.exit` command.

```sh
cd tutara-cli
cargo run
```

**Sample usage**

```sh
Initialized Tutara interactive mode. Use ".file [path]" to read files or ".exit" to leave.

> 1 + 2
3

> .file math_plus.ttr
Reading from file math_plus.ttr

> .exit
Exiting interactive mode
```

### Running with a file

To run the interpreter once with in input file use the cargo run command with a file argument.

```sh
cd tutara-cli
cargo run -- --file ./sample/math_plus.ttr
```

### Running an input string

To run the interpreter once with input text use the cargo run command and pass the code as a string.

```sh
cd tutara-cli
cargo run -- "val foo = 1 + 2"
cargo run -- "val bar = 'Hello world'"
```
