use anyhow::{anyhow, Context};
use directories_next::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub api_key: Option<String>,
}

pub fn get_or_create_config() -> anyhow::Result<Config> {
    let project_dirs = ProjectDirs::from("", "", "graphene-cli")
        .ok_or(anyhow!("couldn't find home directory path"))?;
    let config_path = project_dirs.config_dir().join("config.toml");
    let config: Config = if config_path.exists() {
        toml::from_str(&fs::read_to_string(config_path).context("couldn't read config file")?)
            .context("couldn't parse config file")?
    } else {
        fs::create_dir_all(
            config_path
                .parent()
                .ok_or(anyhow!("no known config directory"))?,
        )
        .context("couldn't create config directory")?;
        fs::File::create(config_path).context("couldn't create config file")?;
        Config { api_key: None }
    };

    Ok(config)
}
