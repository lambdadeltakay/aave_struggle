use std::collections::HashSet;

use markovish::Chain;
use rand::prelude::Rng;

const PUNCTUATION: &[&str] = &[".", "!", "?", ",", ";", ":", "(", ")", "[", "]", "{", "}"];

pub struct Generator<T: Rng> {
    chain: Chain,
    rng: T,
}

impl<T: Rng> Generator<T> {
    pub fn new(rng: T, text: HashSet<String>) -> Self {
        let mut chain = Chain::builder();

        for line in text {
            chain = chain.feed_str(&line).unwrap().into();
        }

        let chain = chain.build().unwrap();

        Self { chain, rng }
    }

    pub fn generate(&mut self) -> String {
        let mut final_string = String::new();
        let token_count = self.rng.gen_range(5..35);
        let mut generation = self.chain.generate_str(&mut self.rng, token_count).unwrap();

        while let Some(word) = generation.first() {
            if *word == " " || PUNCTUATION.contains(word) {
                generation.remove(0);
            } else {
                break;
            }
        }

        for (index, string) in generation.iter().enumerate() {
            final_string.push_str(string);

            if let Some(next_string) = generation.get(index + 1) {
                // Insert spaces when needed
                if *string != " " && *next_string != " " && !PUNCTUATION.contains(next_string) {
                    final_string.push(' ');
                }
            }
        }

        final_string.trim().to_string()
    }
}
