mod lib;
use env_logger;
use log::{info, error};
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json;
use tiny_http::{Server, Response};


// Slack outgoing webhooks are handled here. Requests come in and are run through
// the markov chain to generate a response, which is sent back to Slack.
//
// Create an outgoing webhook in your Slack here:
// https://my.slack.com/services/new/outgoing-webhook

const IP: &str = "192.168.86.82";
const PORT: &str = "8000";
const BOT_NAME: &str = "athena";
const RESPONSE_CHANCE: i32 = 5;

#[derive(Deserialize, Serialize)]
struct Message {
	text: String,
	user_id: String,
}

fn main() {
	env_logger::init();
	let address = format!("{}:{}", IP, PORT);
	let server = Server::http(address).unwrap();
	let mut content = String::new();

	// TODO: multi-thread server
	loop {
		let mut request = match server.recv() {
			Ok(rq) => rq,
			Err(e) => { error!("Error: {}", e); break }
		};

		let mut body = String::new();
    	request.as_reader().read_to_string(&mut body).unwrap();
    	let json = format!("{{\"{}\"}}", body.replace("=", r#"":""#).replace("&", r#"", ""#));
    	let slack_request: Message = serde_json::from_str(&json).unwrap();

		let mut input = slack_request.text.replace("+", " ");
		let name = slack_request.user_id;

		if input != "" && name != "" {
			info!("Handling incoming request: {}", input);

			input = input.replace("@", "");
			input = input.replace("#", "");
			// TODO: trim content length after concatenation exceeds a limit
			content.push_str(" ");
			content.push_str(&input);

			let mut rng = rand::thread_rng();
			let chance = rng.gen_range(0..100);
			if (chance < RESPONSE_CHANCE) || input.starts_with(BOT_NAME) {
				let chain = lib::generate_chain(content.clone());
				// TODO: decode UTF-8 to plain text
				let slack_response = Message { text: chain, user_id: BOT_NAME.to_string() };
				let response = serde_json::to_string(&slack_response).unwrap();
				let _ = request.respond(Response::from_string(response));
			}
		}
	}
}
