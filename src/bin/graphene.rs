use anyhow::{anyhow, Result};
use clap::{crate_description, crate_name, crate_version, App, AppSettings};
use directories_next::ProjectDirs;
use graphene_cli::{api, config::Config};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let project_dirs = ProjectDirs::from("", "", "graphene-cli")
        .ok_or(anyhow!("couldn't find home directory path"))?;
    let config_path = project_dirs.config_dir().join("config.toml");
    let mut config = Config::load(&config_path).await?;
    log::debug!("loaded config: {:?}", config);

    if config.api_key().is_empty() {
        config.configure_api_key().await?;
        config.save().await?;
    }

    let app_matches = App::new(crate_name!())
        .about(crate_description!())
        // TODO: clap doesn't add a newline to version output
        .version(format!("{}\n", crate_version!()).as_str())
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .global_setting(AppSettings::ColoredHelp)
        .global_setting(AppSettings::VersionlessSubcommands)
        .global_setting(AppSettings::DisableHelpSubcommand)
        .subcommand(
            App::new("db")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(App::new("list")),
        )
        .get_matches();

    match app_matches.subcommand() {
        ("db", Some(matches)) => {
            let db_cmd = api::db::DbCmd::new(&config);
            match matches.subcommand() {
                ("list", _) => {
                    log::debug!("{:#?}", db_cmd.list().await);
                }
                _ => {}
            }
        }
        _ => {}
    }

    Ok(())
}
