#![feature(drain_filter)]

use clap::Parser;

use resolve_path::PathResolveExt;

use crate::args::RoundsArgs;

mod args;

mod types;

fn main() {
    const DOWNLOAD_URL: &str = "https://rounds.thunderstore.io/package/download";

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
