use std::error::Error;
use crate::apps::{self, InitialisationError};
use crate::apps::Application;
use lazy_static::lazy_static;
use rusoto_core::{credential::ChainProvider, Region};
use rusoto_core::{HttpClient};
use rusoto_s3::S3Client;
use rusoto_s3::{GetObjectRequest, S3};
use std::{collections::HashMap, io::Write};
use std::sync::RwLock;
use futures::{StreamExt, future};
use std::env;

pub struct AwsAppContext {
    apps: RwLock<HashMap<String, apps::Application>>,
}

lazy_static! {
    pub static ref APPCONTEXT: RwLock<AwsAppContext> = AwsAppContext::new();
}

impl AwsAppContext {
    pub fn new() -> RwLock<Self> {
        RwLock::new(Self {
            apps: RwLock::new(HashMap::new()),
        })
    }

    pub async fn init(&self) -> Result<(),Box<dyn Error+'_>>{
        let mut apps = self.apps.write()?;

        let s3_client = S3Client::new_with(
            HttpClient::new().expect("failed to create request dispatcher"),
            ChainProvider::default(),
            Region::UsEast2,
        );

        let app = apps::Application {
            description: String::from("manages queuing"),
            id: String::from("test"),
            name: String::from("test"),
            remote_path: String::from("code/sum.js"),
            version: apps::AppVersion {
                major: 1,
                minor: 0,
                revision: 0,
                commit_id: String::from("213123123"),
            },
        };
        let bucket_name = match env::var("BUCKET_NAME"){
            Ok(s)=>s,
            Err(e)=> return Err(Box::new(InitialisationError {
                step_name: String::from("read_bucket_name_from_environment_variable"),
                reason:format!("{:?}",e),
                value:String::from("BUCKET_NAME")
            }))
        };
        let result = s3_client
            .get_object(GetObjectRequest {
                bucket: String::from(bucket_name),
                key: app.remote_path.clone(),
                ..GetObjectRequest::default()
            })
            .await?;

        let bs= result
                    .body
                    .unwrap();

        let mut file = std::fs::File::create(std::path::Path::new("code.js"))?;
        
        bs.for_each(|b|{
            let bytes = b.unwrap();
            file.write_all(&bytes.to_vec()).expect("error");
            future::ready(())
        }).await;
        
        apps.insert(app.id.clone(), app);

        Ok(())
    }

    pub fn get_apps(&self) -> Vec<Application> {
        let apps = self.apps.read().unwrap();
        apps.clone().into_iter().map(|(_, app)| app).collect()
    }

    pub fn save_app(&self, app: &Application) {
        let mut apps = self.apps.write().unwrap();
        apps.insert(app.id.clone(), app.clone());
        ()
    }
}
