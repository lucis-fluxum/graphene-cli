use crate::config::Config;
use anyhow::Result;
use reqwest::{Client, RequestBuilder};

pub mod db;

pub(crate) fn get(client: &Client, config: &Config, path: &str) -> Result<RequestBuilder> {
    Ok(client
        .get(&format!("https://api.graphenedb.com/v1/{}", path))
        .header("api_key", config.api_key()))
}
