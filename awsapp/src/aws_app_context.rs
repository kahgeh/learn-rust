use crate::apps;
use crate::apps::Application;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::RwLock;


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

    pub fn init(&self) {
        let mut a = self.apps.write().unwrap();
        a.insert(
            String::from("queue"),
            apps::Application {
                description: String::from("manages queuing"),
                id: String::from("test"),
                name: String::from("test"),
                version: apps::AppVersion {
                    major: 1,
                    minor: 0,
                    revision: 0,
                    commit_id: String::from("213123123"),
                },
            },
        );
        ()
    }

    pub fn get_apps(&self) -> Vec<Application> {
        let apps = self.apps.read().unwrap();

        apps.clone().into_iter().map(|(_, app)| app).collect()
    }

}
