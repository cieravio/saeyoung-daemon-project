mod dialogues;
mod connection;
mod server;

use dialogues::Dialogues;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let dialogues = Dialogues::load("dialogues.json");
    server::run("0.0.0.0:707", dialogues.await)
}