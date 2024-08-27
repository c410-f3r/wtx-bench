use wtx::http::server_framework::{get, post, Router, SerdeJson, ServerFramework};

#[tokio::main]
async fn main() -> wtx::Result<()> {
    let router = Router::paths(wtx::paths!(
        ("hello-world", get(hello_world)),
        ("json", post(json)),
    ));
    ServerFramework::new(router)
        .listen("0.0.0.0:9000", |_| {})
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

async fn json(SerdeJson(de): SerdeJson<RequestElement>) -> wtx::Result<SerdeJson<ResponseElement>> {
    let _sum = de._n0.wrapping_add(de._n1).into();
    Ok(SerdeJson(ResponseElement { _sum }))
}

async fn hello_world() -> &'static str {
    "Hello, World!"
}
