use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    let stream = TcpStream::connect("127.0.0.1:7878").await.unwrap();
    let (reader, mut writer) = stream.into_split();
    let mut lines = BufReader::new(reader).lines();

    // Envoie SUBSCRIBE
    writer.write_all(b"SUBSCRIBE db.host\n").await.unwrap();
    println!("Envoyé : SUBSCRIBE db.host");

    // Attend les réponses
    while let Ok(Some(line)) = lines.next_line().await {
        println!("Reçu : {}", line);
    }
}