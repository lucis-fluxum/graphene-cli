use crate::config::Config;
use anyhow::Result;
use reqwest::{Client, RequestBuilder};

pub mod db;
pub mod version;

const API_ROOT: &'static str = "https://api.graphenedb.com/v1";

pub(crate) fn get(client: &Client, config: &Config, path: &str) -> Result<RequestBuilder> {
    Ok(client
        .get(&format!("{}/{}", API_ROOT, path))
        .header("api_key", config.api_key()))
}
