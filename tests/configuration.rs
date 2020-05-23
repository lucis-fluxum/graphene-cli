use anyhow::Result;
use graphene_cli::config::Config;
use std::path::PathBuf;
use tokio::fs;

#[tokio::test]
async fn create_config_file_if_not_exists() -> Result<()> {
    let config_dir = PathBuf::from("tests/fixtures/create_config_file");
    if config_dir.exists() {
        fs::remove_dir_all(&config_dir).await?;
        fs::create_dir_all(&config_dir).await?;
    }
    let config_path = config_dir.join("config.toml");
    assert!(!config_path.exists());
    let config = Config::load(&config_path).await?;
    assert!(config.api_key().is_empty());
    assert!(config_path.exists());
    Ok(())
}

#[tokio::test]
async fn get_existing_config() -> Result<()> {
    let config_path = PathBuf::from("tests/fixtures/existing_config_file/config.toml");
    assert!(config_path.exists());
    let config = Config::load(&config_path).await?;
    assert_eq!("test_api_key", config.api_key());
    Ok(())
}

#[tokio::test]
async fn save_config() -> Result<()> {
    let config_path = PathBuf::from("tests/fixtures/save_config_file/config.toml");
    let config = Config::new(config_path.clone(), String::from("test_api_key"));
    if config_path.exists() {
        fs::remove_file(&config_path).await?;
    }
    assert!(!config_path.exists());
    config.save().await?;
    assert!(config_path.exists());
    assert_eq!("test_api_key", Config::load(&config_path).await?.api_key());
    Ok(())
}
