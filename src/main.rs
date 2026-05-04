mod store;
mod server;

use store::new_shared_store;
use server::run_server;

#[tokio::main]
async fn main() {
    println!("Lancement du serveur ConfigHub - Partie 3");

    let store = new_shared_store();

    println!("Store initialisé ✓");
    println!("Démarrage du serveur TCP...");

    run_server(store).await;
}