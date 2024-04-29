mod config;
mod generator;

use crate::generator::Generator;
use config::Config;
use rand::prelude::thread_rng;
use std::{path::PathBuf, time::Duration};
use tokio::{fs::read_to_string, time::sleep};
use tumblr_api::{auth::Credentials, client::Client, npf::ContentBlockText};

#[tokio::main]
async fn main() {
    flexi_logger::Logger::try_with_str("debug")
        .unwrap()
        .start()
        .unwrap();

    let rng = thread_rng();

    let config: Config = toml::from_str(
        &read_to_string(PathBuf::from("config/config.toml"))
            .await
            .unwrap(),
    )
    .expect("Failed to parse config");

    // Setup the text generator
    let mut generator = Generator::new(rng.clone(), config.phrases);

    // Connect to tumblr
    let client = Client::new(Credentials::new(config.tumblr_key, config.tumblr_secret));

    loop {
        let generation = generator.generate();
        log::info!("Generated struggle text: {}", generation);

        let post = vec![ContentBlockText::builder(generation).build()];

        client
            .create_post(config.blog_name.clone(), post)
            .send()
            .await
            .unwrap();

        log::info!("Commited to tumblr!");

        // Posts hourly
        sleep(Duration::from_secs(30 * 60)).await;
    }
}
