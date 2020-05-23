use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{self, Write},
    path::Path,
};

#[derive(Debug, Serialize, Deserialize)]
// TODO: Clean up this interface, maybe add some accessors or make Config store its path
pub struct Config {
    api_key: Option<String>,
}

impl Config {
    pub fn new(api_key: Option<String>) -> Self {
        Config { api_key }
    }

    pub fn api_key(&self) -> Option<&str> {
        self.api_key.as_deref()
    }

    pub fn configure_api_key(&mut self) -> anyhow::Result<()> {
        let mut api_key = String::new();
        while api_key.trim().is_empty() {
            print!("Enter API client key: ");
            io::stdout().flush()?;
            io::stdin().read_line(&mut api_key)?;
        }
        self.api_key.replace(api_key.trim().to_string());
        Ok(())
    }

    pub fn load(path: &Path) -> anyhow::Result<Self> {
        let config: Config = if path.exists() {
            log::debug!("loading {}", path.to_string_lossy());
            toml::from_str(&fs::read_to_string(path).context("couldn't read config file")?)
                .context("couldn't parse config file")?
        } else {
            log::debug!("no config found, creating empty one");
            fs::create_dir_all(path.parent().context("invalid config dir")?)
                .context("couldn't create config directory")?;
            fs::File::create(path).context("couldn't create config file")?;
            Config::new(None)
        };

        Ok(config)
    }

    pub fn save(&self, path: &Path) -> anyhow::Result<()> {
        let parent = path.parent().context("invalid config dir")?;
        if !parent.exists() {
            log::debug!("no config dir found, creating one");
            fs::create_dir_all(parent)?;
        }
        Ok(fs::write(path, toml::to_string(self)?)?)
    }
}
