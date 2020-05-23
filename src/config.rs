use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::{
    io::{self, Write},
    path::{Path, PathBuf},
};
use tokio::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(skip)]
    path: PathBuf,
    api_key: Option<String>,
}

impl Config {
    pub fn new(path: PathBuf, api_key: Option<String>) -> Self {
        Config { path, api_key }
    }

    pub async fn load(path: &Path) -> anyhow::Result<Self> {
        let config: Config = if path.exists() {
            log::debug!("loading config from {}", path.to_string_lossy());
            let mut deserialized_config: Self = toml::from_str(
                &fs::read_to_string(path)
                    .await
                    .context("couldn't read config file")?,
            )
            .context("couldn't parse config file")?;
            // We don't store the config location in the config file, so add it here
            deserialized_config.path = path.to_path_buf();
            deserialized_config
        } else {
            log::debug!("no config found, creating empty one");
            fs::create_dir_all(path.parent().context("invalid config dir")?)
                .await
                .context("couldn't create config directory")?;
            fs::File::create(path)
                .await
                .context("couldn't create config file")?;
            Config::new(path.to_path_buf(), None)
        };

        Ok(config)
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn api_key(&self) -> Option<&str> {
        self.api_key.as_deref()
    }

    pub fn configure_api_key(&mut self) -> anyhow::Result<()> {
        log::debug!("prompting user for API key");
        let mut api_key = String::new();
        while api_key.trim().is_empty() {
            print!("Enter API client key: ");
            io::stdout().flush()?;
            io::stdin().read_line(&mut api_key)?;
        }
        self.api_key.replace(api_key.trim().to_string());
        Ok(())
    }

    pub async fn save(&self) -> anyhow::Result<()> {
        log::debug!("saving config: {:?}", self);
        let parent = self.path.parent().context("invalid config dir")?;
        if !parent.exists() {
            log::debug!("no config dir found, creating one");
            fs::create_dir_all(parent).await?;
        }
        Ok(fs::write(&self.path, toml::to_string(self)?).await?)
    }
}
