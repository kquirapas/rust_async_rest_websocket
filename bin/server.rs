use anyhow::{Context, Result};
use axum::{
    extract::ws::{WebSocket, WebSocketUpgrade},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};

const LOCALHOST: &str = "127.0.0.1";
const PORT: &str = "3000";

#[tokio::main]
async fn main() -> Result<()> {
    let listener = tokio::net::TcpListener::bind(format!("{LOCALHOST}:{PORT}"))
        .await
        .with_context(|| "Failed to start TCP listener.")?;

    let app = Router::new()
        .route("/", get(|| async { "hotdog" }))
        .route("/ws", get(handler));

    axum::serve(listener, app)
        .await
        .with_context(|| "Failed to serve.")?;

    Ok(())
}

async fn handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        let msg = if let Ok(msg) = msg {
            msg
        } else {
            return;
        };

        // error converting Message to &str
        if msg.to_text().is_err() {
            return;
        }

        match msg.to_text() {
            Ok(text) => {
                println!("Sent from client: {text}");
            }
            Err(_) => return,
        }

        if socket.send(msg).await.is_err() {
            return;
        }
    }
}
