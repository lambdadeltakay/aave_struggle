use std::collections::HashSet;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub tumblr_key: String,
    pub tumblr_secret: String,
    pub blog_name: String,
    #[serde(default)]
    pub phrases: HashSet<String>,
}
