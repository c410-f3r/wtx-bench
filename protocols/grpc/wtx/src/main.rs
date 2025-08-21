pub mod grpc_bindings;

use wtx::{
    de::format::QuickProtobuf,
    grpc::GrpcManager,
    http::{
        ReqResBuffer, StatusCode,
        server_framework::{Router, ServerFrameworkBuilder, State, post},
    },
    rng::SeedableRng,
    rng::ChaCha20
};

#[tokio::main]
async fn main() -> wtx::Result<()> {
    let router = Router::paths(wtx::paths!((
        "/wtx.GenericService/generic_method",
        post(wtx_generic_service_generic_method)
    )))
    .unwrap();
    ServerFrameworkBuilder::new(ChaCha20::from_os()?, router)
        .with_stream_aux(|_| Ok(QuickProtobuf::default()))
        .tokio("0.0.0.0:9000", |_| {}, |_| Ok(()), |_| {})
        .await
}

async fn wtx_generic_service_generic_method(
    _: State<'_, (), GrpcManager<QuickProtobuf>, ReqResBuffer>,
) -> wtx::Result<StatusCode> {
    Ok(StatusCode::Ok)
}
