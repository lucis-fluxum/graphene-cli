use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub api_key: Option<String>,
}

pub fn get_or_create_config(config_path: &Path) -> anyhow::Result<Config> {
    let config: Config = if config_path.exists() {
        log::debug!("loading {}", config_path.to_string_lossy());
        toml::from_str(&fs::read_to_string(config_path).context("couldn't read config file")?)
            .context("couldn't parse config file")?
    } else {
        log::debug!("no config found, creating empty one");
        fs::create_dir_all(config_path.parent().context("invalid config dir")?)
            .context("couldn't create config directory")?;
        fs::File::create(config_path).context("couldn't create config file")?;
        Config { api_key: None }
    };

    Ok(config)
}
