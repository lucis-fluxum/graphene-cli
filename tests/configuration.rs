use graphene_cli::config::Config;
use std::{fs, path::PathBuf};

#[test]
fn create_config_file_if_not_exists() {
    let config_dir = PathBuf::from("tests/fixtures/create_config_file");
    if config_dir.exists() {
        fs::remove_dir_all(&config_dir).unwrap();
        fs::create_dir_all(&config_dir).unwrap();
    }
    let config_path = config_dir.join("config.toml");
    assert!(!config_path.exists());
    let config = Config::load(&config_path).unwrap();
    assert!(config.api_key().is_none());
    assert!(config_path.exists());
}

#[test]
fn get_existing_config() {
    let config_path = PathBuf::from("tests/fixtures/existing_config_file/config.toml");
    assert!(config_path.exists());
    let config = Config::load(&config_path).unwrap();
    assert_eq!(Some("test_api_key"), config.api_key());
}

#[test]
fn save_config() {
    let config_path = PathBuf::from("tests/fixtures/save_config_file/config.toml");
    let config = Config::new(config_path.clone(), Some(String::from("test_api_key")));
    if config_path.exists() {
        fs::remove_file(&config_path).unwrap();
    }
    assert!(!config_path.exists());
    config.save().unwrap();
    assert!(config_path.exists());
    assert_eq!(
        Some("test_api_key"),
        Config::load(&config_path).unwrap().api_key()
    );
}
