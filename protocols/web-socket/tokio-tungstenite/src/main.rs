use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:9000").await.unwrap();
    while let Ok((stream, _)) = listener.accept().await {
        wtx_bench_common::bench_stream(&stream).unwrap();
        tokio::spawn(async move {
            let mut ws_stream = accept_async(stream).await.unwrap();
            while let Some(msg_rslt) = ws_stream.next().await {
                let msg = msg_rslt.unwrap();
                if msg.is_text() || msg.is_binary() {
                    ws_stream.send(msg).await.unwrap();
                }
            }
        });
    }
}
