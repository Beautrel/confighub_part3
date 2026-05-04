use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tokio::sync::broadcast;

// La structure principale du store
pub struct ConfigStore {
    data: HashMap<String, String>,
    pub sender: broadcast::Sender<(String, String, String)>,
}

impl ConfigStore {
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(100);
        ConfigStore {
            data: HashMap::new(),
            sender,
        }
    }

    // Lire une valeur
    pub fn get(&self, namespace: &str, key: &str) -> Option<String> {
        self.data.get(&format!("{}:{}", namespace, key)).cloned()
    }

    // Ecrire une valeur et notifier les abonnés
    pub fn set(&mut self, namespace: &str, key: &str, value: &str) {
        let k = format!("{}:{}", namespace, key);
        self.data.insert(k, value.to_string());
        let _ = self.sender.send((
            namespace.to_string(),
            key.to_string(),
            value.to_string(),
        ));
    }
}

pub type SharedStore = Arc<RwLock<ConfigStore>>;

pub fn new_shared_store() -> SharedStore {
    Arc::new(RwLock::new(ConfigStore::new()))
}