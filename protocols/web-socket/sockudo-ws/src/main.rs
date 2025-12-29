use bytes::BytesMut;
use core::net::SocketAddr;
use futures_util::{SinkExt, StreamExt};
use sockudo_ws::error::Result;
use sockudo_ws::handshake::{build_response, generate_accept_key, parse_request};
use sockudo_ws::protocol::Message;
use sockudo_ws::{Config, WebSocketStream};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    let addr: SocketAddr = "0.0.0.0:9000".parse().unwrap();
    let listener = TcpListener::bind(addr).await.unwrap();

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        wtx_bench_common::bench_stream(&stream).unwrap();
        tokio::spawn(handle_connection(stream));
    }
}

async fn handle_connection(stream: TcpStream) {
    let stream = do_handshake(stream).await.unwrap();
    let mut ws = WebSocketStream::server(stream, Config::default());
    while let Some(msg) = ws.next().await {
        match msg.unwrap() {
            Message::Text(text) => {
                ws.send(Message::Text(text)).await.unwrap();
            }
            Message::Binary(data) => {
                ws.send(Message::Binary(data)).await.unwrap();
            }
            Message::Ping(_) => {}
            Message::Pong(_) => {}
            Message::Close(_) => break,
        }
    }
}

async fn do_handshake(mut stream: TcpStream) -> Result<TcpStream> {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    let mut buf = BytesMut::with_capacity(4096);
    loop {
        let n = stream.read_buf(&mut buf).await.unwrap();
        if n == 0 {
            return Err(sockudo_ws::Error::ConnectionClosed);
        }
        if let Some((req, _)) = parse_request(&buf).unwrap() {
            let accept_key = generate_accept_key(req.key);
            let response = build_response(&accept_key, None, None);
            stream.write_all(&response).await.unwrap();
            stream.flush().await.unwrap();
            break;
        }
    }
    Ok(stream)
}
