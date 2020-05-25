use crate::{
    api::{db::DbCmd, get},
    config::Config,
    model::User,
};
use anyhow::{Context, Result};
use reqwest::Client;

pub struct UserCmd<'a> {
    client: &'a Client,
    config: &'a Config,
}

impl<'a> UserCmd<'a> {
    pub fn new(client: &'a Client, config: &'a Config) -> Self {
        Self { client, config }
    }

    // TODO: It's pretty inefficient to keep fetching the ID from the name
    pub(crate) async fn find_id(&self, db_name: &str, user_name: &str) -> Result<String> {
        let databases: Vec<User> = self.list(db_name).await?;
        Ok(databases
            .into_iter()
            .find(|u| u.username.as_deref() == Some(user_name))
            .context(format!("database not found: '{}'", db_name))?
            .id
            // TODO: unwrapping because User fields are forced to be Options (see model::User)
            .unwrap())
    }

    pub async fn list(&self, db_name: &str) -> Result<Vec<User>> {
        let response = get(
            self.client,
            self.config,
            &format!(
                "databases/{}/users",
                DbCmd::new(self.client, self.config)
                    .find_id(db_name)
                    .await?
            ),
        )?
        .send()
        .await?
        .text()
        .await?;
        log::debug!("response: {}", response);
        Ok(serde_json::from_str(&response)?)
    }

    pub async fn show(&self, db_name: &str, user_name: &str) -> Result<User> {
        let response = get(
            self.client,
            self.config,
            &format!(
                "databases/{}/users/{}",
                DbCmd::new(self.client, self.config)
                    .find_id(db_name)
                    .await?,
                self.find_id(db_name, user_name).await?
            ),
        )?
        .send()
        .await?
        .text()
        .await?;
        log::debug!("response: {}", response);
        Ok(serde_json::from_str(&response)?)
    }
}
