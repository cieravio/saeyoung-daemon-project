use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::time::{interval, Duration};
use crate::dialogues::Dialogues;

pub async fn handle_connection(mut stream: TcpStream, dialogues: Dialogues) {
    if let Some(msg) = dialogues.random_from("connect_success") {
        let _ = stream.write_all(format!("{msg}\n").as_bytes()).await;
    }

    let mut ticker = interval(Duration::from_secs(300));
    let mut buf = [0u8; 1024];

    loop {
        tokio::select! {
            read_result = stream.read(&mut buf) => {
                match read_result {
                    Ok(0) | Err(_) => break,
                    Ok(_) => {}
                }
            }
            _ = ticker.tick() => {
                if let Some(msg) = dialogues.random_from("care_checker_general") {
                    if stream.write_all(format!("{msg}\n").as_bytes()).await.is_err() {
                        break;
                    }
                }
            }
        }
    }
}