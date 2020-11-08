use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct AppVersion {
    pub major: u8,
    pub minor: u8,
    pub revision: u8,
    pub commit_id: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Application {
    pub id: String,
    pub name: String,
    pub version: AppVersion,
    pub description: String,
    pub remote_path: String,
}

