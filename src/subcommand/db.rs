use anyhow::Result;
use crate::{config::Config, model};

pub struct DbCmd<'a> {
    config: &'a Config,
}

impl<'a> DbCmd<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self { config }
    }

    pub async fn list(&self) -> Result<Vec<model::Database>> {
        let client = reqwest::Client::new();
        let response = client
            .get("https://api.graphenedb.com/v1/databases")
            .header("api_key", self.config.api_key())
            .send()
            .await?
            .text()
            .await?;
        log::debug!("response: {}", response);
        Ok(serde_json::from_str(&response)?)
    }
}
