use std::{borrow::Cow, path::Path};

use clap::{Args, Command, Parser, Subcommand};
use resolve_path::PathResolveExt;

#[derive(Debug, Parser)]
#[clap(author = "mdrokz", version = "1.0", about)]
pub struct RoundsArgs {
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    Download(DownloadCommand),
}

pub struct ResolveDownload;

const ROUNDS_PATH: &'static str = "~/.steam/steam/steamapps/common/ROUNDSs";

impl ResolveDownload {
    fn resolve() -> &'static str {
        let _ = ROUNDS_PATH
            .try_resolve()
            .expect("unable to resolve home path");

        ROUNDS_PATH
    }
}

#[derive(Debug, Args)]
pub struct DownloadCommand {
    #[clap(default_value = ResolveDownload::resolve(),short, long)]
    pub download_path: String,
    #[clap(short, long)]
    pub mod_id: String,
}
