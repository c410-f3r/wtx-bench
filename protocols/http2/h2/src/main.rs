use h2::{server, SendStream};
use tokio::net::TcpListener;
use bytes::Bytes;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:9000").await.unwrap();
    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let _jh = tokio::spawn(async move {
            let mut conn = server::Builder::new()
                .max_concurrent_streams(32)
                .handshake(stream)
                .await
                .unwrap();
            while let Some(result) = conn.accept().await {
                let (mut req, mut respond) = result.unwrap();
                let _jh = tokio::spawn(async move {
                    let body = req.body_mut();
                    let mut data = Vec::new();
                    while let Some(rslt) = body.data().await {
                        let local_data = rslt.unwrap();
                        let _ = body.flow_control().release_capacity(local_data.len());
                        data.extend(local_data.as_ref());
                    }
                    let mut send: SendStream<Bytes> = respond
                        .send_response(http::Response::new(()), false)
                        .unwrap();
                    send.send_data(data.into(), true).unwrap();
                });
            }
        });
    }
}
