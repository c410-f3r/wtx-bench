use futures::io::{BufReader, BufWriter};
use soketto::{connection, handshake};
use tokio::net::TcpListener;
use tokio_stream::{wrappers::TcpListenerStream, StreamExt};
use tokio_util::compat::TokioAsyncReadCompatExt;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:9000").await.unwrap();
    let mut incoming = TcpListenerStream::new(listener);
    while let Some(socket_rslt) = incoming.next().await {
        let socket = socket_rslt.unwrap();
        let mut server = handshake::Server::new(BufReader::new(BufWriter::new(socket.compat())));
        let accept = handshake::server::Response::Accept {
            key: server.receive_request().await.unwrap().key(),
            protocol: None,
        };
        server.send_response(&accept).await.unwrap();
        let (mut sender, mut receiver) = server.into_builder().finish();
        let mut message = Vec::new();
        loop {
            message.clear();
            match receiver.receive_data(&mut message).await {
                Ok(soketto::Data::Binary(_)) => {
                    sender.send_binary_mut(&mut message).await.unwrap();
                    sender.flush().await.unwrap()
                }
                Ok(soketto::Data::Text(_)) => {
                    if let Ok(txt) = std::str::from_utf8(&message) {
                        sender.send_text(txt).await.unwrap();
                        sender.flush().await.unwrap()
                    } else {
                        break;
                    }
                }
                Err(connection::Error::Closed) => break,
                Err(_) => {
                    break;
                }
            }
        }
    }
}
