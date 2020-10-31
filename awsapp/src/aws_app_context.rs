use crate::apps::Application;
use std::sync::RwLockReadGuard;
use std::{sync::RwLock};
use std::collections::HashMap;
use lazy_static::lazy_static;
use crate::apps;

pub struct AwsAppContext {
    apps: RwLock<HashMap<String, apps::Application>>,
}

lazy_static! {
    pub static ref APPCONTEXT: RwLock<AwsAppContext> = AwsAppContext::new();
}

impl AwsAppContext {
    pub  fn new() -> RwLock<Self> {
        RwLock::new(Self {
            apps: RwLock::new(HashMap::new()),
        })
    }

    pub fn init()  {
        let w =APPCONTEXT.write().unwrap();
        let mut a = w.apps.write().unwrap();
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

    pub fn get_apps(&self) -> Vec<Application>{
        let m: RwLockReadGuard<'static, AwsAppContext> = APPCONTEXT.read().unwrap();
        let apps = m.apps
            .read()
            .unwrap();
        
        apps.clone()
            .into_iter()    
            .map(|(_,app)|app)
            .collect()
    }

    pub fn get() -> RwLockReadGuard<'static, AwsAppContext> {
        let m: RwLockReadGuard<'static, AwsAppContext> = APPCONTEXT.read().unwrap();
        m
    }    
}

