use crate::config::Config;

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
            .unwrap();
        log::debug!("response: {}", response.text().await.unwrap());
    }
}
