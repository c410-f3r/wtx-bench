use futures::{SinkExt, StreamExt};
use http_body_util::Empty;
use hyper::{
    Request, Response,
    body::{Bytes, Incoming},
    server::conn::http1,
    service::service_fn,
};
use tokio::net::TcpListener;
use yawc::{CompressionLevel, Options, WebSocket, WebSocketError, frame::OpCode};

#[tokio::main]
async fn main() -> yawc::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:9000").await?;
    loop {
        let (stream, _) = listener.accept().await?;
        tokio::spawn(async move {
            let io = hyper_util::rt::TokioIo::new(stream);
            let _rslt = http1::Builder::new()
                .serve_connection(io, service_fn(server_upgrade))
                .with_upgrades()
                .await;
        });
    }
}

async fn handle_client(fut: yawc::UpgradeFut) -> yawc::Result<()> {
    let mut ws = fut.await?;
    loop {
        let frame = ws.next().await.ok_or(WebSocketError::ConnectionClosed)?;
        match frame.opcode {
            OpCode::Close => break,
            OpCode::Text | OpCode::Binary => {
                ws.send(frame).await?;
            }
            _ => {}
        }
    }
    Ok(())
}

async fn server_upgrade(mut req: Request<Incoming>) -> yawc::Result<Response<Empty<Bytes>>> {
    let (response, fut) = WebSocket::upgrade_with_options(
        &mut req,
        Options::default()
            .with_utf8()
            .with_max_payload_read(100 * 1024 * 1024)
            .with_max_read_buffer(200 * 1024 * 1024)
            .with_compression_level(CompressionLevel::none()),
    )?;
    tokio::task::spawn(async move {
        let _rslt = handle_client(fut).await;
    });
    Ok(response)
}

