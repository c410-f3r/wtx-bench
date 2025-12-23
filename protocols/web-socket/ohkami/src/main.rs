use ohkami::prelude::*;
use ohkami::ws::{Message, WebSocket, WebSocketContext};

async fn echo_text(ctx: WebSocketContext<'_>) -> WebSocket {
    ctx.upgrade(|mut conn| async move {
        while let Ok(Some(Message::Text(text))) = conn.recv().await {
            conn.send(text).await.expect("failed to send text");
        }
    })
}

#[tokio::main]
async fn main() {
    Ohkami::new("/".GET(echo_text)).howl("0.0.0.0:9000").await
}
