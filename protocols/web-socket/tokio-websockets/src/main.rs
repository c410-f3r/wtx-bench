use futures_util::{SinkExt, StreamExt};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio_websockets::{Config, Limits, ServerBuilder};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind(SocketAddr::from(([0, 0, 0, 0], 9000)))
        .await
        .unwrap();
    loop {
        let (conn, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            let mut server = unsafe {
                ServerBuilder::new()
                    .config(Config::default().frame_size(usize::MAX))
                    .limits(Limits::unlimited())
                    .accept(conn)
                    .await
                    .unwrap_unchecked()
            };
            while let Some(Ok(item)) = server.next().await {
                if item.is_text() || item.is_binary() {
                    unsafe { server.send(item).await.unwrap_unchecked() };
                }
            }
        });
    }
}
