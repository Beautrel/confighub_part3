use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    // Ce client envoie une commande SET pour simuler une mise à jour
    let mut stream = TcpStream::connect("127.0.0.1:7878").await.unwrap();
    stream.write_all(b"SET db.host=localhost\n").await.unwrap();
    println!("Envoyé : SET db.host=localhost");
}