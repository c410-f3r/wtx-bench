use tokio::net::TcpListener;
use wtx::{
    collections::Vector,
    rng::{ChaCha20, CryptoSeedableRng},
    tls::{TlsAcceptor, TlsConfig},
    web_socket::{OpCode, WebSocketAcceptor, WebSocketPayloadOrigin},
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:9000").await.unwrap();
    loop {
        let (stream, _) = listener.accept().await.unwrap();
        wtx_bench_common::bench_stream(&stream).unwrap();
        let _jh = tokio::spawn(async move {
            let mut buffer = Vector::new();
            let mut ws = WebSocketAcceptor::default()
                .accept(TlsAcceptor::new(
                    TlsConfig::plaintext(),
                    ChaCha20::from_std_random().unwrap(),
                    stream,
                ))
                .await
                .unwrap();
            let (mut common, mut reader, mut writer) = ws.split_mut();
            loop {
                let mut frame = reader
                    .read_frame(&mut buffer, &mut common, WebSocketPayloadOrigin::Adaptive)
                    .await
                    .unwrap();
                match frame.op_code() {
                    OpCode::Binary | OpCode::Text => {
                        writer.write_frame(&mut common, &mut frame).await.unwrap();
                    }
                    OpCode::Close => break,
                    _ => {}
                }
            }
        });
    }
}
