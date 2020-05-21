use clap::*;

fn main() {
    App::new(crate_name!())
        // TODO: clap doesn't add a newline to version output
        .version(format!("{}\n", crate_version!()).as_str())
        .get_matches();
}
