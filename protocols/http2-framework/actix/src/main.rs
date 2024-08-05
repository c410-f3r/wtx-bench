use actix_web::{web, App, HttpResponse, HttpServer};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/hello-world").route(web::get().to(hello_world)))
            .service(web::resource("/json").route(web::post().to(json)))
    })
    .bind_auto_h2c(("0.0.0.0", 9000))?
    .run()
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

async fn hello_world() -> HttpResponse {
    HttpResponse::Ok().body("Hello, World!")
}

async fn json(web::Json(de): web::Json<RequestElement>) -> HttpResponse {
    let _sum = de._n0.wrapping_add(de._n1).into();
    HttpResponse::Ok().json(ResponseElement { _sum })
}
