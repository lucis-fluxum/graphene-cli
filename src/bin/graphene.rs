use clap::*;

fn main() {
    App::new(crate_name!())
        .about(crate_description!())
        // TODO: clap doesn't add a newline to version output
        .version(format!("{}\n", crate_version!()).as_str())
        .global_setting(AppSettings::ColoredHelp)
        .global_setting(AppSettings::SubcommandRequiredElseHelp)
        .global_setting(AppSettings::VersionlessSubcommands)
        .global_setting(AppSettings::DisableHelpSubcommand)
        .get_matches();
}
