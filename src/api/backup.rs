use crate::{
    api::{db::DbCmd, get},
    config::Config,
    model::Backup,
};
use anyhow::Result;
use reqwest::Client;

pub struct BackupCmd<'a> {
    client: &'a Client,
    config: &'a Config,
}

impl<'a> BackupCmd<'a> {
    pub fn new(client: &'a Client, config: &'a Config) -> Self {
        Self { client, config }
    }

    // TODO: Not sure if this works since my test DB doesn't support backups
    pub async fn list(&self, name: &str) -> Result<Vec<Backup>> {
        let id = DbCmd::new(self.client, self.config).find_id(name).await?;
        let response = get(
            self.client,
            self.config,
            &format!("databases/{}/backups", id),
        )?
        .send()
        .await?
        .text()
        .await?;
        Ok(serde_json::from_str(&response)?)
    }
}
