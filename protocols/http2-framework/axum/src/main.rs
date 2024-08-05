use axum::{
    routing::{get, post},
    Json, Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/hello-world", get(hello_world))
        .route("/json", post(json));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:9000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
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

async fn hello_world() -> &'static str {
    "Hello, World!"
}

async fn json(Json(de): Json<RequestElement>) -> Json<ResponseElement> {
    Json(ResponseElement {
        _sum: de._n0.wrapping_add(de._n1).into(),
    })
}
