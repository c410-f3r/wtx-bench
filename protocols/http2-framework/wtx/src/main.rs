use wtx::http::server_framework::{Router, SerdeJsonOwned, ServerFrameworkBuilder, get, post};
use wtx::rng::{ChaCha20, SeedableRng};

#[tokio::main]
async fn main() -> wtx::Result<()> {
    let router = Router::paths(wtx::paths!(
        ("/hello-world", get(hello_world)),
        ("/json", post(json)),
    ))
    .unwrap();
    ServerFrameworkBuilder::new(ChaCha20::from_os()?, router)
        .without_aux()
        .tokio("0.0.0.0:9000", |_| {}, |_| Ok(()), |_| Ok(()), |_| {})
        .await
}

#[derive(serde::Deserialize)]
struct RequestElement {
    _n0: u64,
    _n1: u64,
}

#[derive(serde::Serialize)]
struct ResponseElement {
    _sum: u128,
}

async fn json(
    SerdeJsonOwned(de): SerdeJsonOwned<RequestElement>,
) -> wtx::Result<SerdeJsonOwned<ResponseElement>> {
    let _sum = de._n0.wrapping_add(de._n1).into();
    Ok(SerdeJsonOwned(ResponseElement { _sum }))
}

async fn hello_world() -> &'static str {
    "Hello, World!"
}
