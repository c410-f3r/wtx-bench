use fastwebsockets::{FragmentCollector, OpCode, WebSocketError, upgrade};
use hyper::{server::conn::http1, service::service_fn};
use tokio::net::TcpListener;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:9000").await.unwrap();
    loop {
        let (stream, _) = listener.accept().await.unwrap();
        wtx_bench_common::bench_stream(&stream).unwrap();
        tokio::spawn(async move {
            http1::Builder::new()
                .serve_connection(
                    hyper_util::rt::TokioIo::new(stream),
                    service_fn(|mut req| async move {
                        let (response, fut) = upgrade::upgrade(&mut req)?;
                        tokio::task::spawn(async move {
                            async move {
                                let mut ws = FragmentCollector::new(fut.await.unwrap());
                                loop {
                                    let frame = ws.read_frame().await.unwrap();
                                    match frame.opcode {
                                        OpCode::Binary | OpCode::Text => {
                                            ws.write_frame(frame).await.unwrap();
                                        }
                                        OpCode::Close => break,
                                        _ => {}
                                    }
                                }
                            }
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
