use h2::server;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:9000").await.unwrap();
    loop {
        if let Ok((socket, _)) = listener.accept().await {
            tokio::spawn(async move {
                let mut conn = server::handshake(socket).await.unwrap();
                while let Some(result) = conn.accept().await {
                    let (mut req, mut respond) = result.unwrap();
                    tokio::spawn(async move {
                        let body = req.body_mut();
                        let mut send = respond
                            .send_response(http::Response::new(()), false)
                            .unwrap();
                        while let Some(data) = body.data().await {
                            let data = data.unwrap();
                            let _ = body.flow_control().release_capacity(data.len());
                            send.send_data(data, false).unwrap();
                        }
                        send.send_data([].as_slice().into(), true).unwrap();
                    });
                }
            });
        }
    }
}
