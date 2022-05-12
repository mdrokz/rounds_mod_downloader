use std::{env::args, fs, os::unix::fs::DirEntryExt, path::Path};

use clap::{Arg, Command, Parser};

use resolve_path::PathResolveExt;

use crate::args::RoundsArgs;

mod args;

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
            let (download_path, mod_id) = (
                c.download_path
                    .try_resolve()
                    .expect("failed to resolve home path"),
                c.mod_id,
            );

            if !download_path.is_dir() {
                panic!("Provided directory {:?} does not exist", download_path);
            }

            let mod_resp = attohttpc::get(format!("{}/{}", DOWNLOAD_URL, mod_id))
                .send()
                .expect("failed to download mod");

            let headers = mod_resp.headers();

            let file_type = headers
                .get("content-type")
                .expect("failed to get content-type")
                .to_str()
                .and_then(|x| {
                    let content_type = String::from(x);
                    let mut c_split = content_type.split("/");
                    let ctype = c_split.next().expect("failed to get value");
                    Ok(ctype.to_string())
                })
                .expect("failed to convert to string");

            

            println!("{:?}", mod_resp.headers());
        }
    }
}
