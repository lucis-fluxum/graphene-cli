use crate::{api::get, config::Config, model::Database};
use anyhow::{Context, Result};
use reqwest::Client;

pub struct DbCmd<'a> {
    client: &'a Client,
    config: &'a Config,
}

impl<'a> DbCmd<'a> {
    pub fn new(client: &'a Client, config: &'a Config) -> Self {
        Self { client, config }
    }

    pub async fn list(&self) -> Result<Vec<Database>> {
        let response = get(self.client, self.config, "databases")?
            .send()
            .await?
            .text()
            .await?;
        log::debug!("response: {}", response);
        Ok(serde_json::from_str(&response)?)
    }

    pub async fn show(&self, name: &str) -> Result<Database> {
        let databases: Vec<Database> = self.list().await?;
        let id = databases
            .into_iter()
            .find(|d| d.name == name)
            .context(format!("database not found: '{}'", name))?
            .id;
        let response = get(self.client, self.config, &format!("databases/{}", id))?
            .send()
            .await?
            .text()
            .await?;
        log::debug!("response: {}", response);
        Ok(serde_json::from_str(&response)?)
    }
}
