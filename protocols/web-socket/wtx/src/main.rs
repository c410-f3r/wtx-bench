use tokio::net::TcpListener;
use wtx::{
    collection::Vector,
    rng::{Xorshift64, simple_seed},
    web_socket::{OpCode, WebSocketAcceptor, WebSocketPayloadOrigin},
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:9000").await.unwrap();
    let xorshift = Xorshift64::from(simple_seed());
    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let _jh = tokio::spawn(async move {
            let mut buffer = Vector::new();
            let mut ws = WebSocketAcceptor::default()
                .rng(xorshift)
                .accept(stream)
                .await
                .unwrap();
            let (mut common, mut reader, mut writer) = ws.parts_mut();
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
