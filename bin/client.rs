use anyhow::{Context, Result};
use axum::http::Uri;
use futures_util::{SinkExt, StreamExt};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_websockets::{ClientBuilder, Message};

const WEBSOCKETS_URI: &str = "ws://127.0.0.1:3000/ws";

#[tokio::main]
async fn main() -> Result<()> {
    let uri = Uri::from_static(WEBSOCKETS_URI);
    let (mut socket, _) = ClientBuilder::from_uri(uri)
        .connect()
        .await
        .with_context(|| "Failed to connect.")?;

    let stdin = tokio::io::stdin();
    let mut stdin = BufReader::new(stdin).lines();

    loop {
        let cli_input = stdin
            .next_line()
            .await
            .with_context(|| "Failed to read from stdin.")?;

        if let Some(msg) = cli_input {
            tokio::spawn(async move { socket.send(Message::text(msg)).await });
        }
    }
}
