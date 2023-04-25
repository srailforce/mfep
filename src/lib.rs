pub mod service;
use std::path::Path;
use serde::{Serialize, Deserialize};
use tokio::io::AsyncRead;

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    pub auto_resp: Vec<AutoResp>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AutoResp {
    pub listen: String,
    pub responsors: Vec<Responsor>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Responsor {
    pub path: String,
    pub response: String,
}

pub fn load_config<P: AsRef<Path> + Send>(path: P) -> anyhow::Result<Configuration> {
    let file = std::fs::File::open(path)?;
    let configuration: Configuration = serde_yaml::from_reader(file)?;
    Ok(configuration)
}

