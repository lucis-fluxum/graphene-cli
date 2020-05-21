use clap::{crate_description, crate_name, crate_version, App, AppSettings};
use graphene_cli::config;

fn main() -> anyhow::Result<()> {
    println!("cli config: {:?}", config::get_or_create_config());

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
