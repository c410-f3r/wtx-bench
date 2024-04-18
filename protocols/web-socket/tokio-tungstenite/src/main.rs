use futures_util::{SinkExt, StreamExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::Result};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:9000").await.unwrap();
    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(async move {
            let _rslt = handle_connection(stream).await;
        });
    }
}

async fn handle_connection(stream: TcpStream) -> Result<()> {
    let mut ws_stream = accept_async(stream).await.unwrap();
    while let Some(msg_rslt) = ws_stream.next().await {
        let msg = msg_rslt?;
        if msg.is_text() || msg.is_binary() {
            ws_stream.send(msg).await?;
        }
    }
    Ok(())
}
