<p align="center"><img src="https://github.com/tutara/tutara-assets/raw/master/logos/logo.svg" width="200" /></p>
<h1 align="center">Tutara</h1>

<p align="center">
	<a href="https://tutara.dev/"><img src="https://img.shields.io/badge/website-tutara.dev-orange" alt="Website" /></a>
	<a href="https://img.shields.io/github/workflow/status/tutara/tutara/cargo"><img src="https://img.shields.io/github/workflow/status/tutara/tutara/cargo" alt="cargo" /></a>
	<a href="https://github.com/orgs/tutara/projects/1"><img src="https://img.shields.io/badge/tutara-Roadmap-darkgreen" alt="Roadmap" /></a>
	<a href="LICENSE"><img src="https://img.shields.io/github/license/tutara/tutara" alt="License" /></a>
	<a href="https://github.com/tutara/tutara/graphs/contributors"><img src="https://img.shields.io/github/contributors/tutara/tutara" alt="Contributers" /></a>
	<a href="CODE_OF_CONDUCT.md"><img src="https://img.shields.io/badge/contributor%20covenant-v2.0%20adopted-ff69b4.svg" alt="Contributor Covenant" /></a>
</p>

---

Tutara is an experimental programming language aimed at creating a reliable sandboxed contextual function platform. This open-source platform enables the ability to write scoped scripts. The platform can be embedded in software, or used with the standard integrations. For example, the HTTP context supports scripts to perform small actions like hashing a value or aggregating RSS feeds - there are no limits to the possibilities.

_Be aware that Tutara is still in an early stage of development._

## Contributing

Check out our [Contributing guidelines](CONTRIBUTING.md) if you'd like to help us out.

## Development

### Cargo

We use the official package manager for Rust, Cargo, as tool for managing dependencies, building and running the code. Visit the [Rust documentation](https://doc.rust-lang.org/cargo/) for detailed information on Cargo.

The repository hosts multiple crates. It is therefor required to switch your working directory before running the cargo commands.

### LLVM

The compiler uses LLVM on for resolving, compiling and executing the code. It is a necessary dependency for development. Installing LLVM differs per operating system.

#### Linux

LLVM can be installed on linux using a package manager. For example, in Debian/Ubuntu you can run the following example:

```
sudo apt install llvm-10-dev
```

#### Windows

The LLVM Windows builds don't include the full toolset that is used for development in the interpreter. You will need to build the currently used version (LLVM-10) yourself or find a working build online.

[Installing LLVM on Windows](https://llvm.org/docs/GettingStartedVS.html)

In the future we would like to supply LLVM builds for contributers.

### Running interactive mode

The CLI crate has an interactive command line tool where you can write code and pass it to the interpreter. To exit the command line use the `.exit` command.

```sh
cd tutara-cli
cargo run interactive
```

**Sample usage**

```sh
Initialized Tutara interactive mode. Use ".exit" to leave.

> var foo = 1 + 2
> return foo
3

> .exit
Exiting interactive mode
```

### Running with a file

To run the interpreter once with in input file use the cargo run command with an input argument.

```sh
cd tutara-cli
cargo run run -i ../sample/math_plus.ttr
```
