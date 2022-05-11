use std::{env::args, fs, os::unix::fs::DirEntryExt, path::Path};

use clap::{Arg, Command};

use resolve_path::PathResolveExt;

fn main() {
    const CONFIG_PATH: &str = "~/.config/rounds_mod_downloader";

    let config_path = CONFIG_PATH
        .try_resolve()
        .expect("couldnt resolve home path");

    if !config_path.is_dir() {
        std::fs::create_dir(
            CONFIG_PATH
                .try_resolve()
                .expect("couldnt resolve home path"),
        )
        .expect("couldnt create config directory");
    }

    let plugin_path = args().next().expect("please provide your plugin path");

    let matches = Command::new("Rounds Mod Downloader")
        .version("1.0")
        .about("CLI for downloading mods from thunderstore.io")
        .subcommand(
            Command::new("download")
                .about("Download mod by providing a mod_id")
                .arg(
                    Arg::new("mod_id")
                        .short('i')
                        .value_name("MOD_ID")
                        .help("provide a mod_id to download the mod"),
                ),
        )
        .get_matches();
}
