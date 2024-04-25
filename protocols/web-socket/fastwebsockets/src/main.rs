use fastwebsockets::{upgrade, FragmentCollector, OpCode, WebSocketError};
use hyper::{server::conn::http1, service::service_fn};
use tokio::net::TcpListener;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:9000").await.unwrap();
    loop {
        let (stream, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            http1::Builder::new()
                .serve_connection(
                    hyper_util::rt::TokioIo::new(stream),
                    service_fn(|mut req| async move {
                        let (response, fut) = upgrade::upgrade(&mut req)?;
                        tokio::task::spawn(async move {
                            tokio::task::unconstrained(async move {
                                let mut ws = FragmentCollector::new(fut.await.unwrap());
                                loop {
                                    let frame = ws.read_frame().await.unwrap();
                                    match frame.opcode {
                                        OpCode::Close => break,
                                        OpCode::Text | OpCode::Binary => {
                                            ws.write_frame(frame).await.unwrap();
                                        }
                                        _ => {}
                                    }
                                }
                            })
                            .await
                        });
                        Ok::<_, WebSocketError>(response)
                    }),
                )
                .with_upgrades()
                .await
                .unwrap();
        });
    }
}
