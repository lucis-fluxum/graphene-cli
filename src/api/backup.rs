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
}
