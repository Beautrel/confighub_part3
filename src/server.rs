use crate::store::SharedStore;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;

pub async fn run_server(store: SharedStore) {
    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();
    println!("Serveur démarré sur 127.0.0.1:7878");

    loop {
        let (socket, addr) = listener.accept().await.unwrap();
        println!("Nouveau client connecté : {}", addr);

        let store = store.clone();

        tokio::spawn(async move {
            handle_client(socket, store).await;
        });
    }
}

async fn handle_client(
    socket: tokio::net::TcpStream,
    store: SharedStore,
) {
    let (reader, mut writer) = socket.into_split();
    let mut lines = BufReader::new(reader).lines();

    while let Ok(Some(line)) = lines.next_line().await {
        let line = line.trim().to_string();
        println!("Reçu : {}", line);

        if line.starts_with("SUBSCRIBE ") {
            let parts: Vec<&str> = line.splitn(2, ' ').collect();
            if parts.len() == 2 {
                let key_parts: Vec<&str> = parts[1].splitn(2, '.').collect();
                if key_parts.len() == 2 {
                    let namespace = key_parts[0].to_string();
                    let key = key_parts[1].to_string();

                    let mut rx = {
                        let store = store.read().unwrap();
                        store.sender.subscribe()
                    };

                    writer
                        .write_all(
                            format!("OK subscribed to {}.{}\n", namespace, key).as_bytes(),
                        )
                        .await
                        .unwrap();

                    loop {
                        match rx.recv().await {
                            Ok((ns, k, value)) => {
                                if ns == namespace && k == key {
                                    let msg = format!("UPDATE {}.{}={}\n", ns, k, value);
                                    if writer.write_all(msg.as_bytes()).await.is_err() {
                                        break;
                                    }
                                }
                            }
                            Err(_) => break,
                        }
                    }
                } else {
                    writer.write_all(b"ERROR format: SUBSCRIBE namespace.key\n").await.unwrap();
                }
            }
        } else if line.starts_with("SET ") {
            let parts: Vec<&str> = line.splitn(2, ' ').collect();
            if parts.len() == 2 {
                let kv: Vec<&str> = parts[1].splitn(2, '=').collect();
                if kv.len() == 2 {
                    let key_parts: Vec<&str> = kv[0].splitn(2, '.').collect();
                    if key_parts.len() == 2 {
                        let namespace = key_parts[0];
                        let key = key_parts[1];
                        let value = kv[1];
                        {
                            let mut store = store.write().unwrap();
                            store.set(namespace, key, value.to_string());
                        }
                        writer.write_all(b"OK\n").await.unwrap();
                    }
                }
            }
        } else {
            writer.write_all(b"ERROR commande inconnue\n").await.unwrap();
        }
    }

    println!("Client déconnecté");
}