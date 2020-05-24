use anyhow::{anyhow, Result};
use clap::{crate_description, crate_name, crate_version, App, AppSettings, Arg};
use directories_next::ProjectDirs;
use graphene_cli::{
    api::{db::DbCmd, version::VersionCmd},
    config::Config,
};

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

    let http_client = reqwest::Client::builder().build()?;

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
                .subcommand(App::new("list"))
                .subcommand(App::new("show").arg(Arg::with_name("name"))),
        )
        .subcommand(App::new("versions"))
        .get_matches();

    match app_matches.subcommand() {
        ("db", Some(matches)) => {
            let db_cmd = DbCmd::new(&http_client, &config);
            match matches.subcommand() {
                ("list", _) => {
                    log::debug!("{:#?}", db_cmd.list().await);
                }
                ("show", Some(matches)) => {
                    let name: String = matches.value_of_t_or_exit("name");
                    log::debug!("{:#?}", db_cmd.show(&name).await);
                }
                _ => {}
            }
        }
        ("versions", _) => {
            let version_cmd = VersionCmd::new(&http_client, &config);
            log::debug!("{:#?}", version_cmd.list().await);
        }
        _ => {}
    }

    Ok(())
}
