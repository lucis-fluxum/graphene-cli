use anyhow::anyhow;
use clap::{crate_description, crate_name, crate_version, App, AppSettings};
use directories_next::ProjectDirs;
use graphene_cli::config::Config;
use std::io::{self, Write};

fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let project_dirs = ProjectDirs::from("", "", "graphene-cli")
        .ok_or(anyhow!("couldn't find home directory path"))?;
    let config_path = project_dirs.config_dir().join("config.toml");
    let mut config = Config::load(&config_path)?;
    log::debug!("loaded config: {:?}", config);

    if config.api_key.is_none() {
        config.api_key = Some(prompt_for_api_key()?);
        config.save(&config_path)?;
    }

    App::new(crate_name!())
        .about(crate_description!())
        // TODO: clap doesn't add a newline to version output
        .version(format!("{}\n", crate_version!()).as_str())
        .global_setting(AppSettings::ColoredHelp)
        .global_setting(AppSettings::SubcommandRequiredElseHelp)
        .global_setting(AppSettings::VersionlessSubcommands)
        .global_setting(AppSettings::DisableHelpSubcommand)
        .get_matches();

    Ok(())
}

fn prompt_for_api_key() -> anyhow::Result<String> {
    let mut api_key = String::new();
    while api_key.trim().is_empty() {
        print!("Enter API client key: ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut api_key)?;
    }
    Ok(api_key.trim().to_string())
}
