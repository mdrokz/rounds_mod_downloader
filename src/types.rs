use serde::Deserialize;

#[derive(Clone,Debug,Deserialize)]
pub struct Manifest {
    pub name: String,
    pub description: String,
    pub version_number: String,
    #[serde(rename = "dependencies")]
    pub deps: Vec<String>,
    pub website_url: String 
}