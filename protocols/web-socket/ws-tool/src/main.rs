use tokio::net::TcpListener;
use ws_tool::{
    codec::{default_handshake_handler, AsyncStringCodec},
    ServerBuilder,
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:9000").await.unwrap();
    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let _jh = tokio::spawn(async move {
            let (mut read, mut write) = ServerBuilder::async_accept(
                stream,
                default_handshake_handler,
                AsyncStringCodec::factory,
            )
            .await
            .unwrap()
            .split();
            loop {
                let msg = read.receive().await.unwrap();
                write.send(msg).await.unwrap();
            }
        });
    }
}
