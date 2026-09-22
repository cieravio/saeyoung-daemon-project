use tokio::net::TcpListener;
use crate::dialogues::Dialogues;
use crate::connection::handle_connection;

pub async fn run(addr: &str, dialogues: Dialogues) -> std::io::Result<()> {
    let listener = TcpListener::bind(addr).await?;
    println!("listening on {addr}");

    loop {
        let (stream, _addr) = listener.accept().await?;
        let dialogues = dialogues.clone();
        tokio::spawn(handle_connection(stream, dialogues));
    }
}