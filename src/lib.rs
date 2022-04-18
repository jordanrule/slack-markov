use markov::Chain;

pub fn generate_chain(input: &str) -> String {
	let mut chain = Chain::new();
	chain.feed_str(input);
	return chain.generate_str();
}
