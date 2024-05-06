use wtx::{
    http::{server::TokioHttp2, Headers, RequestMut, Response, StatusCode},
    misc::ByteVector,
};

#[tokio::main]
async fn main() {
    TokioHttp2::tokio_http2(
        "0.0.0.0:9000".parse().unwrap(),
        None,
        |err| eprintln!("Connection error: {err:?}"),
        |err| eprintln!("Request error: {err:?}"),
        handle,
    )
    .await
    .unwrap()
}

async fn handle<'buffer>(
    req: RequestMut<'buffer, 'buffer, 'buffer, ByteVector>,
) -> Result<Response<(&'buffer mut ByteVector, &'buffer mut Headers)>, ()> {
    req.headers.clear();
    Ok(Response::http2((req.data, req.headers), StatusCode::Ok))
}
