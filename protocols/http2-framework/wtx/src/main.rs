use wtx::http::{
    server_framework::{get, post, Router, ServerFramework},
    ReqResBuffer, Request, Response, StatusCode,
};

#[tokio::main]
async fn main() -> wtx::Result<()> {
    let router = Router::paths(wtx::paths!(
        ("hello-world", get(hello_world)),
        ("json", post(json)),
    ));
    ServerFramework::new(router).listen("0.0.0.0:9000").await
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

async fn hello_world(mut req: Request<ReqResBuffer>) -> wtx::Result<Response<ReqResBuffer>> {
    req.rrd.clear();
    req.rrd.extend_body(b"Hello, World!")?;
    Ok(req.into_response(StatusCode::Ok))
}

async fn json(mut req: Request<ReqResBuffer>) -> wtx::Result<Response<ReqResBuffer>> {
    let de: RequestElement = simd_json::from_slice(req.rrd.body_mut())?;
    req.rrd.clear();
    let _sum = de._n0.wrapping_add(de._n1).into();
    simd_json::to_writer(&mut req.rrd, &ResponseElement { _sum })?;
    Ok(req.into_response(StatusCode::Ok))
}
