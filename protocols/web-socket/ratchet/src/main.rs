use bytes::BytesMut;
use ratchet_rs::{Message, NoExtProvider, PayloadType, ProtocolRegistry, WebSocketConfig};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:9000").await.unwrap();
    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let _jh = tokio::spawn(async move {
            let upgrader = ratchet_rs::accept_with(
                stream,
                WebSocketConfig::default(),
                NoExtProvider,
                ProtocolRegistry::default(),
            )
            .await
            .unwrap();
            let mut upgraded = upgrader.upgrade().await.unwrap();
            let mut buffer = BytesMut::with_capacity(1024 * 16);
            loop {
                match upgraded.websocket.read(&mut buffer).await.unwrap() {
                    Message::Close(_) => break,
                    Message::Binary => {
                        upgraded
                            .websocket
                            .write(&mut buffer, PayloadType::Binary)
                            .await
                            .unwrap();
                        buffer.clear();
                    }
                    Message::Ping(_) | Message::Pong(_) => {}
                    Message::Text => {
                        upgraded
                            .websocket
                            .write(&mut buffer, PayloadType::Text)
                            .await
                            .unwrap();
                        buffer.clear();
                    }
                }
            }
        });
    }
}
