#![feature(drain_filter)]


use clap::{Parser};

use resolve_path::PathResolveExt;

use crate::{args::RoundsArgs};

mod args;

mod types;

fn main() {
    const CONFIG_PATH: &str = "~/.config/rounds_mod_downloader";
    const DOWNLOAD_URL: &str = "https://rounds.thunderstore.io/package/download";

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
            let download_path = c
                .download_path
                .try_resolve()
                .expect("failed to resolve home path");

            if !download_path.is_dir() {
                panic!("Provided directory {:?} does not exist", download_path);
            }

            println!("Starting Download...");

            c.download_mod(DOWNLOAD_URL.to_string(), download_path, None);

            println!("Completed Download...");
        }
    }
}
