use tokio::net::TcpStream;
use wtx::{
  http::server::TokioWebSocket,
  rng::StdRng,
  web_socket::{FrameBufferVec, OpCode, WebSocketBuffer, WebSocketServer},
};

#[tokio::main]
async fn main() {
  TokioWebSocket::tokio_web_socket(
    "0.0.0.0:9000".parse().unwrap(),
    None,
    || (),
    |err| println!("Connection error: {err:?}"),
    handle,
  )
  .await
  .unwrap()
}

async fn handle(
  (fb, mut ws): (&mut FrameBufferVec, WebSocketServer<(), StdRng, TcpStream, &mut WebSocketBuffer>),
) -> wtx::Result<()> {
  loop {
    let mut frame = ws.read_frame(fb).await?;
    match frame.op_code() {
      OpCode::Binary | OpCode::Text => {
        ws.write_frame(&mut frame).await?;
      }
      OpCode::Close => break,
      _ => {}
    }
  }
  Ok(())
}
