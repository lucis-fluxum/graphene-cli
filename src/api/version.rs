use crate::{api::get, config::Config, model::Version};
use anyhow::Result;
use reqwest::Client;

pub struct VersionCmd<'a> {
    client: &'a Client,
    config: &'a Config,
}

impl<'a> VersionCmd<'a> {
    pub fn new(client: &'a Client, config: &'a Config) -> Self {
        Self { client, config }
    }

    pub async fn list(&self) -> Result<Vec<Version>> {
        let response = get(self.client, self.config, "databases/versions")?
            .send()
            .await?
            .text()
            .await?;
        log::debug!("response: {}", response);
        Ok(serde_json::from_str(&response)?)
    }
}
