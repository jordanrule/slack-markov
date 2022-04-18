use markov::Chain;

let mut chain = Chain::new();
chain.feed_str("I like cats and I like dogs.");
println!("{}", chain.generate_str());