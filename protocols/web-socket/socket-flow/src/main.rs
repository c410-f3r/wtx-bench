use futures::StreamExt;
use socket_flow::handshake::accept_async;
use socket_flow::stream::SocketFlowStream;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:9000").await.unwrap();
    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(async move {
            let mut ws_connection = accept_async(SocketFlowStream::Plain(stream)).await.unwrap();
            while let Some(result) = ws_connection.next().await {
                ws_connection.send_message(result.unwrap()).await.unwrap();
            }
        });
    }
}
