use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct AppVersion {
    pub major: u8,
    pub minor: u8,
    pub revision: u8,
    pub commit_id: String,
}

#[derive(Serialize, Clone)]
pub struct Application {
    pub id: String,
    pub name: String,
    pub version: AppVersion,
    pub description: String,
}

