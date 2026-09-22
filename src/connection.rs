use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::time::{interval_at, Instant, Duration};
use crate::dialogues::Dialogues;

pub async fn handle_connection(mut stream: TcpStream, dialogues: Dialogues) {
    let _ = stream.write_all(b"Connected to Agent 707.\n").await;
    if let Some(msg) = dialogues.random_from("connect_success") {
        let _ = stream.write_all(format!("{msg}\n").as_bytes()).await;
    }

    let delay = Duration::from_secs(900);
    let mut ticker = interval_at(Instant::now() + delay, Duration::from_secs(300));
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