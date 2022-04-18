mod lib;
use rand::Rng;
use tiny_http::{Server, Response};


// Slack outgoing webhooks are handled here. Requests come in and are run through
// the markov chain to generate a response, which is sent back to Slack.
//
// Create an outgoing webhook in your Slack here:
// https://my.slack.com/services/new/outgoing-webhook

const IP: &str = "127.0.0.1";
const PORT: &str = "8000";
const BOT_NAME: &str = "athena";
const RESPONSE_CHANCE: i32 = 5;

fn main() {
	let address = format!("{}:{}", IP, PORT);
	let server = Server::http(address).unwrap();
	let mut content = String::new();

	loop {
		let mut request = match server.recv() {
			Ok(rq) => rq,
			Err(e) => { println!("error: {}", e); break }
		};

		let mut input = String::new();
		request.as_reader().read_to_string(&mut input).unwrap();

		if input != "" {
			input = input.replace("@", "");
			input = input.replace("#", "");
			content.push_str(" ");
			content.push_str(&input);
		}

		let mut rng = rand::thread_rng();
		let chance = rng.gen_range(0..100);
		if (chance < RESPONSE_CHANCE) || input.starts_with(BOT_NAME) {
			let chain = lib::generate_chain(content.clone());
			let response = Response::from_string(chain);
			let _ = request.respond(response);
		}
	}
}
