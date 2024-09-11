pub mod grpc_bindings;

use wtx::{
    data_transformation::dnsn::QuickProtobuf,
    grpc::GrpcManager,
    http::{
        server_framework::{post, Router, ServerFrameworkBuilder, State},
        ReqResBuffer, StatusCode,
    },
};

#[tokio::main]
async fn main() -> wtx::Result<()> {
    let router = Router::paths(wtx::paths!((
        "/wtx.GenericService/generic_method",
        post(wtx_generic_service_generic_method)
    )))
    .unwrap();
    ServerFrameworkBuilder::new(router)
        .with_req_aux(|| QuickProtobuf::default())
        .listen("0.0.0.0:9000", |_| {})
        .await
}

async fn wtx_generic_service_generic_method(
    _: State<'_, (), GrpcManager<QuickProtobuf>, ReqResBuffer>,
) -> wtx::Result<StatusCode> {
    Ok(StatusCode::Ok)
}
