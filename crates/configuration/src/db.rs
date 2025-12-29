use secrecy::SecretBox;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Database {
    url: SecretBox<String>,
}

impl Database {
    pub fn url(&self) -> &SecretBox<String> {
        &self.url
    }
}
