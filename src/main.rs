mod store;
mod server;

use store::new_shared_store;
use server::run_server;

#[tokio::main]
async fn main() {
    let store = new_shared_store();
    run_server(store).await;
}