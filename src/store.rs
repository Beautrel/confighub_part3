use std::collections::HashMap;
use std::sync::{Arc, RwLock};

// Alias pour eviter d'ecrire le type en entier a chaque fois
pub type SharedConfigStore = Arc<RwLock<ConfigStore>>;

// Fake store pour remplacer la partie 1 et faire nos tests
pub struct ConfigStore {
    data: HashMap<String, String>,
    pub global_version: u64,
}

impl ConfigStore {
    pub fn new() -> Self {
        ConfigStore {
            data: HashMap::new(),
            global_version: 0,
        }
    }

    pub fn get(&self, namespace: &str, key: &str) -> Option<String> {
        let k = format!("{}.{}", namespace, key);
        self.data.get(&k).cloned()
    }

    pub fn set(&mut self, namespace: &str, key: &str, value: String) {
        let k = format!("{}.{}", namespace, key);
        self.data.insert(k, value);
        self.global_version += 1; // maj de la version
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

    // Fonction necessaire pour le proto
    pub fn changes_since(&self, _version: u64) -> Vec<(String, String)> {
        // on fait simple pour le stub, on renvoie tout
        let mut res = Vec::new();
        for (k, v) in self.data.iter() {
            res.push((k.clone(), v.clone()));
        }
        res
    }
}
