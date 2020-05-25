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

    pub(crate) async fn find_id(&self, db_name: &str) -> Result<String> {
        let databases: Vec<Database> = self.list().await?;
        Ok(databases
            .into_iter()
            .find(|d| d.name == db_name)
            .context(format!("database not found: '{}'", db_name))?
            .id)
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

    pub async fn show(&self, db_name: &str) -> Result<Database> {
        let response = get(
            self.client,
            self.config,
            &format!("databases/{}", self.find_id(db_name).await?),
        )?
        .send()
        .await?
        .text()
        .await?;
        log::debug!("response: {}", response);
        Ok(serde_json::from_str(&response)?)
    }
}
