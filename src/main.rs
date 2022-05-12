use std::{env::args, fs, os::unix::fs::DirEntryExt, path::Path};

use clap::{Arg, Command, Parser};

use resolve_path::PathResolveExt;

use crate::args::RoundsArgs;

mod args;

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

    let args = RoundsArgs::parse();

    match args.entity_type {
        args::EntityType::Download(c) => {
            println!("{:?}",c);
        },
    }
}
