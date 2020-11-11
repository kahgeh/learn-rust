use std::{error::Error, fmt};

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

#[derive(Debug)]
pub struct InitialisationError {
    pub step_name : String,
    pub reason: String,
    pub value: String
}


impl fmt::Display for InitialisationError {
    fn fmt(&self, f: &mut fmt::Formatter)-> fmt::Result {
        write!(f, "")
    }
}

impl Error for InitialisationError {
}


