use tokio::net::TcpListener;
use wtx::{
    misc::StdRng,
    misc::Vector,
    web_socket::{FrameBufferVec, OpCode, WebSocket, WebSocketBuffer},
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:9000").await.unwrap();
    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let _jh = tokio::spawn(async move {
            let mut ws = WebSocket::accept(
                (),
                StdRng::default(),
                stream,
                WebSocketBuffer::with_capacity(0, 1024 * 16),
                |_| true,
            )
            .await
            .unwrap();
            let mut fb = FrameBufferVec::new(Vector::with_capacity(1024 * 16).unwrap());
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
