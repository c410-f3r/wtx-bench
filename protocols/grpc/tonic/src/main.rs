mod grpc_bindings;

use grpc_bindings::generic_service_server::{GenericService, GenericServiceServer};
use grpc_bindings::GenericRequest;
use tonic::{transport::Server, Request, Response, Status};

#[derive(Default)]
pub struct LocalService {}

#[tonic::async_trait]
impl GenericService for LocalService {
    async fn generic_method(
        &self,
        request: Request<GenericRequest>,
    ) -> Result<Response<grpc_bindings::GenericResponse>, Status> {
        Ok(Response::new(grpc_bindings::GenericResponse {
            generic_response_field0: request.into_parts().2.generic_request_field0,
        }))
    }
}

#[tokio::main]
async fn main() {
    Server::builder()
        .initial_connection_window_size(32 * 1024 * 1024)
        .max_frame_size(64 * 1024)
        .add_service(GenericServiceServer::new(LocalService::default()))
        .serve("0.0.0.0:9000".parse().unwrap())
        .await
        .unwrap();
}
