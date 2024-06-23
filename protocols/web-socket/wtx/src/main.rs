use tokio::net::TcpListener;
use wtx::{
    rng::StdRng,
    web_socket::{
        handshake::{WebSocketAccept, WebSocketAcceptRaw},
        FrameBufferVec, OpCode, WebSocketBuffer,
    },
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:9000").await.unwrap();
    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let _jh = tokio::spawn(async move {
            let mut ws = WebSocketAcceptRaw {
                compression: (),
                rng: StdRng::default(),
                stream,
                wsb: WebSocketBuffer::with_capacity(1024 * 16, 1024 * 16),
            }
            .accept(|_| true)
            .await
            .unwrap();
            let mut fb = FrameBufferVec::new(Vec::with_capacity(1024 * 16));
            loop {
                let mut frame = ws.read_frame(&mut fb).await.unwrap();
                match frame.op_code() {
                    OpCode::Binary | OpCode::Text => {
                        ws.write_frame(&mut frame).await.unwrap();
                    }
                    OpCode::Close => break,
                    _ => {}
                }
            }
        });
    }
}
