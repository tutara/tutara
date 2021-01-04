use hyper::{
	service::{make_service_fn, service_fn},
	StatusCode,
};
use hyper::{Body, Request, Response, Server};
use std::net::SocketAddr;
use std::{convert::Infallible, path::PathBuf};
use std::{env, future::Future};
use tokio::{fs::File, io::AsyncReadExt};
use tutara_compiler_llvm::Evaluator;
use tutara_interpreter::{parser::Parser, Tokenizer};

struct TutaraServer {
	address: SocketAddr,
	working_directory: PathBuf,
}

impl TutaraServer {
	fn start(&self) -> impl Future<Output = Result<(), hyper::Error>> {
		let working_directory = self.working_directory.to_owned();

		let make_service = make_service_fn(move |_| {
			let wd = working_directory.to_owned();

			async { Ok::<_, Infallible>(service_fn(move |req| handle(req, wd.to_owned()))) }
		});

		println!("Starting server on http://{}", self.address);
		println!("Serving from {}", self.working_directory.display());
		println!("Press CTRL+C to terminate.");

		let server = Server::bind(&self.address).serve(make_service);
		server.with_graceful_shutdown(shutdown_signal())
	}
}

async fn shutdown_signal() {
	// Wait for the CTRL+C signal
	tokio::signal::ctrl_c()
		.await
		.expect("failed to install CTRL+C signal handler");
	println!("Stopping server");
}

async fn handle(
	req: Request<Body>,
	working_directory: PathBuf,
) -> Result<Response<Body>, hyper::http::Error> {
	let path = req.uri().path().trim_start_matches('/');
	let script = working_directory.join(path);

	println!("Request {} to {}", path, script.display());

	if !script.starts_with(&working_directory) {
		Response::builder()
			.status(StatusCode::BAD_REQUEST)
			.body(Body::empty())
	} else if !script.is_file() || !script.exists() {
		Response::builder()
			.status(StatusCode::NOT_FOUND)
			.body(Body::empty())
	} else {
		let mut src = String::new();
		let mut file = File::open(script).await.unwrap();
		file.read_to_string(&mut src).await.unwrap();

		let tokenizer = Tokenizer::new(&src);
		let parser = Parser::new(tokenizer.peekable());
		let evaluation = Evaluator::evaluate(parser);

		match evaluation {
			Ok(evaluation) => Response::builder().body(Body::from(format!("{}", evaluation))),
			Err(err) => Response::builder()
				.status(StatusCode::INTERNAL_SERVER_ERROR)
				.body(Body::from(format!("{}", err))),
		}
	}
}

#[tokio::main]
async fn main() {
	let server = TutaraServer {
		address: SocketAddr::from(([127, 0, 0, 1], 3000)),
		working_directory: env::current_dir().unwrap(),
	};

	let instance = server.start();

	if let Err(e) = instance.await {
		eprintln!("server error: {}", e);
	}

	println!("Goodbye");
}
