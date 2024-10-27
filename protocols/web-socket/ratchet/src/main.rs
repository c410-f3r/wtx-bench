use bytes::BytesMut;
use ratchet_rs::{Message, NoExtProvider, PayloadType, SubprotocolRegistry, WebSocketConfig};
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
                SubprotocolRegistry::default(),
            )
            .await
            .unwrap();
            let mut upgraded = upgrader.upgrade().await.unwrap();
            let mut buffer = BytesMut::with_capacity(0);
            loop {
                match upgraded.websocket.read(&mut buffer).await.unwrap() {
                    Message::Binary => {
                        upgraded
                            .websocket
                            .write(&mut buffer, PayloadType::Binary)
                            .await
                            .unwrap();
                        buffer.clear();
                    }
                    Message::Close(_) => break,
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
