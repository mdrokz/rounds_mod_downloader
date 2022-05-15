use std::{
    borrow::Cow,
    fs::{read_to_string, remove_dir_all},
    io::Write,
    path::Path,
};

use clap::{Args, Parser, Subcommand};
use resolve_path::PathResolveExt;

use std::fs;

use crate::types::Manifest;

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

const ROUNDS_PATH: &'static str = "~/.steam/steam/steamapps/common/ROUNDS/BepInEx/plugins";

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

impl DownloadCommand {
    pub fn download_mod(
        &self,
        download_url: String,
        download_path: Cow<Path>,
        dep_id: Option<String>,
    ) {
        // let plugins = fs::read_dir(&download_path).expect("failed to read plugins directory");

        let (mod_id, split) = if let Some(id) = dep_id {
            (id, "-")
        } else {
            (self.mod_id.clone(), "/")
        };

        let mut name_split = mod_id.split(split);

        name_split.next();

        let mod_name = name_split.next().expect("failed to take value");

        println!("Fetching MOD {}", format!("{}/{}", download_url, mod_id));

        let mod_resp = attohttpc::get(format!("{}/{}", download_url, mod_id.replace("-", "/")))
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
                c_split.next();
                let ctype = c_split.next().expect("failed to get value");
                Ok(ctype.to_string())
            })
            .expect("failed to convert to string");

        let zip_bytes = mod_resp.bytes().expect("failed to get bytes");

        let zip_path = download_path.join(format!("{}.{}", mod_name, file_type));

        let mut mod_zip = fs::File::create(&zip_path).expect("failed to create file");

        mod_zip
            .write_all(&zip_bytes)
            .expect("failed to write bytes");

        let mod_zip = fs::File::open(&zip_path).expect("failed to open zip");

        let zip_path = download_path.join(&mod_name);

        if zip_path.is_dir() {
            remove_dir_all(&zip_path).expect("failed to delete Plugin");
        }

        let unzipper = unzip::Unzipper::new(mod_zip, &zip_path);

        unzipper.unzip().expect("failed to unzip mod");

        let deps = read_to_string(zip_path.join("manifest.json")).expect("failed to open manifest");

        let manifest: Manifest = serde_json::from_str(&deps).expect("failed to parse json");

        self.download_deps(download_url, download_path, manifest.deps);
    }

    pub fn download_deps(&self, download_url: String, download_path: Cow<Path>, deps: Vec<String>) {
        println!("Fetching Dependencies...");
        // let threads = deps.iter().map(|dep| {
        //     std::thread::spawn(move || {
        //         self.download_mod(download_url.clone(), download_path.clone(), Some(dep))
        //     })
        // }).collect::<Vec<JoinHandle<()>>>();

        for dep in deps {
            self.download_mod(download_url.clone(), download_path.clone(), Some(dep));
        }
    }
}
