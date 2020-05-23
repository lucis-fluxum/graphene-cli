use crate::{config::Config, model};

pub struct DbCmd<'a> {
    config: &'a Config,
}

impl<'a> DbCmd<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self { config }
    }

    pub async fn list(&self) {
        let client = reqwest::Client::new();
        let response = client
            .get("https://api.graphenedb.com/v1/databases")
            .header("api_key", self.config.api_key().unwrap())
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        log::debug!("response: {}", response);
        let dbs: Vec<model::Db> = serde_json::from_str(&response).unwrap();
        log::debug!("resulting dbs: {:#?}", dbs);
    }
}
