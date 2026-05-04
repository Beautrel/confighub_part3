use tokio::sync::broadcast;

// on derive clone pour le broadcast (obligatoire dans tokio)
#[derive(Clone, Debug)]
pub struct ConfigUpdate {
    pub namespace: String,
    pub key: String,
    pub new_value: String,
}

// systeme pour gerer le push aux clients
pub struct PubSubSystem {
    pub sender: broadcast::Sender<ConfigUpdate>,
}

impl PubSubSystem {
    pub fn new(cap: usize) -> Self {
        let (tx, _) = broadcast::channel(cap);
        PubSubSystem { sender: tx }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<ConfigUpdate> {
        self.sender.subscribe()
    }

    pub fn broadcast_update(&self, namespace: &str, key: &str, new_value: String) {
        let msg = ConfigUpdate {
            namespace: namespace.to_string(),
            key: key.to_string(),
            new_value,
        };
        // c'est pas grave si y'a personne d'abonne, on ignore l'erreur
        let _ = self.sender.send(msg);
    }
}
