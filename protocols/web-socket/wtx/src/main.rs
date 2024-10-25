use tokio::net::TcpListener;
use wtx::{
    misc::{simple_seed, Xorshift64},
    web_socket::{OpCode, WebSocket, WebSocketBuffer},
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:9000").await.unwrap();
    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let _jh = tokio::spawn(async move {
            let mut ws = WebSocket::accept(
                (),
                Xorshift64::from(simple_seed()),
                stream,
                WebSocketBuffer::with_capacity(0, 1024 * 16).unwrap(),
                |_| wtx::Result::Ok(()),
            )
            .await
            .unwrap();
            let (mut common, mut reader, mut writer) = ws.parts();
            loop {
                let mut frame = reader.read_frame(&mut common).await.unwrap();
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
