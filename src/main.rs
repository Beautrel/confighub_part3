mod pubsub;
mod store;

use std::sync::{Arc, RwLock};

#[tokio::main]
async fn main() {
    println!("Lancement du serveur (Partie 3)");

    // on init le faux store
    let store: store::SharedConfigStore = Arc::new(RwLock::new(store::ConfigStore::new()));

    // init du broadcast
    let pubsub_sys = Arc::new(pubsub::PubSubSystem::new(64));

    // Todo pour le binome: 
    // - lancer le tcplistener
    // - faire la boucle accept
    // - faire le tokio::spawn
    
    println!("Pret pour gerer les sockets TCP !");
}
