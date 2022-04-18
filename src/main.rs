mod lib;
use tiny_http::Server;

fn main() {
	let server = Server::http("127.0.0.1:8080").unwrap();
	loop {
		let mut request = match server.recv() {
			Ok(rq) => rq,
			Err(e) => { println!("error: {}", e); break }
		};

		let mut content = String::new();
		request.as_reader().read_to_string(&mut content).unwrap();
		println!("{}", lib::generate_chain(content));
	}
}
