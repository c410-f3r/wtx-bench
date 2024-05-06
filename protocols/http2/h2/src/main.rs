use h2::server;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:9000").await.unwrap();
    loop {
        if let Ok((socket, _)) = listener.accept().await {
            tokio::spawn(async move {
                let mut conn: server::Connection<_, bytes::Bytes> = server::Builder::new()
                    .max_concurrent_streams(128)
                    .handshake(socket)
                    .await
                    .unwrap();
                while let Some(result) = conn.accept().await {
                    let (mut req, mut respond) = result.unwrap();
                    tokio::spawn(async move {
                        let body = req.body_mut();
                        let mut send = respond
                            .send_response(http::Response::new(()), false)
                            .unwrap();
                        let mut data = Vec::new();
                        while let Some(rslt) = body.data().await {
                            let local_data = rslt.unwrap();
                            let _ = body.flow_control().release_capacity(local_data.len());
                            data.extend(local_data.as_ref());
                        }
                        send.send_data(data.into(), true).unwrap();
                    });
                }
            });
        }
    }
}
