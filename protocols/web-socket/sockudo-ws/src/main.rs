use bytes::BytesMut;
use core::net::SocketAddr;
use sockudo_ws::frame::encode_frame;
use sockudo_ws::handshake::{build_response, generate_accept_key, parse_request};
use sockudo_ws::protocol::Protocol;
use sockudo_ws::{Config, OpCode, Role};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
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

async fn handle_connection(mut stream: TcpStream) {
    let mut buf = BytesMut::with_capacity(4096);
    loop {
        let n = stream.read_buf(&mut buf).await.unwrap();
        if n == 0 {
            return;
        }
        if let Some((req, _)) = parse_request(&buf).unwrap() {
            let accept_key = generate_accept_key(req.key);
            let response = build_response(&accept_key, None, None);
            stream.write_all(&response).await.unwrap();
            break;
        }
    }
    buf.clear();

    let config = Config::default();
    let mut protocol = Protocol::new(Role::Server, config.max_frame_size, config.max_message_size);
    let mut write_buf = BytesMut::with_capacity(64 * 1024);
    let mut messages = Vec::with_capacity(8);

    loop {
        let n = stream.read_buf(&mut buf).await.unwrap();
        if n == 0 {
            break;
        }

        if protocol.process_raw_into(&mut buf, &mut messages).is_err() {
            break;
        }

        for msg in messages.drain(..) {
            match msg {
                sockudo_ws::RawMessage::Text(data) | sockudo_ws::RawMessage::Binary(data) => {
                    encode_frame(&mut write_buf, OpCode::Binary, &data, true, None);
                }
                sockudo_ws::RawMessage::Ping(data) => {
                    encode_frame(&mut write_buf, OpCode::Pong, &data, true, None);
                }
                sockudo_ws::RawMessage::Close(_) => {
                    encode_frame(&mut write_buf, OpCode::Close, &[], true, None);
                    stream.write_all(&write_buf).await.ok();
                    return;
                }
                _ => {}
            }
        }

        if !write_buf.is_empty() {
            stream.write_all(&write_buf).await.unwrap();
            write_buf.clear();
        }
    }
}
