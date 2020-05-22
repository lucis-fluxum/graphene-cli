use anyhow::anyhow;
use clap::{crate_description, crate_name, crate_version, App, AppSettings};
use directories_next::ProjectDirs;
use graphene_cli::config;

fn main() -> anyhow::Result<()> {
    let project_dirs = ProjectDirs::from("", "", "graphene-cli")
        .ok_or(anyhow!("couldn't find home directory path"))?;
    println!(
        "cli config: {:?}",
        config::get_or_create_config(&project_dirs.config_dir().join("config.toml"))
    );

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
