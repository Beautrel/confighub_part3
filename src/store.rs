use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tokio::sync::broadcast;

pub type SharedConfigStore = Arc<RwLock<ConfigStore>>;
pub type SharedStore = Arc<RwLock<ConfigStore>>;

pub struct ConfigStore {
    data: HashMap<String, String>,
    pub global_version: u64,
    pub sender: broadcast::Sender<(String, String, String)>,
}

impl ConfigStore {
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(100);
        ConfigStore {
            data: HashMap::new(),
            global_version: 0,
            sender,
        }
    }

    pub fn get(&self, namespace: &str, key: &str) -> Option<String> {
        let k = format!("{}.{}", namespace, key);
        self.data.get(&k).cloned()
    }

    pub fn set(&mut self, namespace: &str, key: &str, value: String) {
        let k = format!("{}.{}", namespace, key);
        self.data.insert(k, value.clone());
        self.global_version += 1;
        let _ = self.sender.send((
            namespace.to_string(),
            key.to_string(),
            value,
        ));
    }

    pub fn delete(&mut self, namespace: &str, key: &str) -> Option<String> {
        let k = format!("{}.{}", namespace, key);
        let val = self.data.remove(&k);
        if val.is_some() {
            self.global_version += 1;
        }
        val
    }

    pub fn get_namespace(&self, namespace: &str) -> HashMap<String, String> {
        let pref = format!("{}.", namespace);
        let mut res = HashMap::new();
        for (k, v) in self.data.iter() {
            if k.starts_with(&pref) {
                res.insert(k.clone(), v.clone());
            }
        }
        res
    }

    pub fn changes_since(&self, _version: u64) -> Vec<(String, String)> {
        let mut res = Vec::new();
        for (k, v) in self.data.iter() {
            res.push((k.clone(), v.clone()));
        }
        res
    }
}

pub fn new_shared_store() -> SharedStore {
    Arc::new(RwLock::new(ConfigStore::new()))
}