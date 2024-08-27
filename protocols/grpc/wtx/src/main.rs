pub mod grpc_bindings;

use wtx::grpc::GrpcStatusCode;
use wtx::{
    data_transformation::dnsn::QuickProtobuf,
    grpc::{Server, ServerData},
    http::{
        server_framework::{post, Router},
        ReqResBuffer, Request, StatusCode,
    },
};

#[tokio::main]
async fn main() -> wtx::Result<()> {
    let router = Router::paths(wtx::paths!((
        "/wtx.GenericService/generic_method",
        post(wtx_generic_service_generic_method)
    ),));
    Server::new(router).listen("0.0.0.0:9000", |_| {}).await
}

async fn wtx_generic_service_generic_method(
    _: &mut Request<ReqResBuffer>,
    _: ServerData<QuickProtobuf>,
) -> wtx::Result<(StatusCode, GrpcStatusCode)> {
    Ok((StatusCode::Ok, GrpcStatusCode::Ok))
}
