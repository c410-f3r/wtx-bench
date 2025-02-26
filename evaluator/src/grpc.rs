pub mod grpc_bindings {
    include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));
}

use crate::{data::string_bytes_8kib, manage_cases, report_line::ReportLine};
use grpc_bindings::wtx::{GenericRequest, GenericResponse};
use std::borrow::Cow;
use wtx::{
    data_transformation::dnsn::QuickProtobuf,
    grpc::GrpcClient,
    http::{ReqResBuffer, client_pool::ClientPoolBuilder},
    misc::UriRef,
};

pub(crate) async fn bench_all(
    generic_rp: ReportLine,
    rps: &mut Vec<ReportLine>,
) -> wtx::Result<()> {
    macro_rules! case {
        (($requests:expr, $request_size:expr), $ex:expr) => {{
            let name = concat!(
                grpc_connections!(),
                " connection(s) sending ",
                $requests,
                " unary request(s) of ",
                $request_size
            );
            (
                name,
                manage_case!(grpc_connections!(), name, generic_rp, $ex),
            )
        }};
    }
    let params = [
        case!((1, "8 KiB"), write(1, string_bytes_8kib()).await),
        case!((16, "8 KiB"), write(16, string_bytes_8kib()).await),
    ];
    manage_cases(generic_rp, rps, params);
    Ok(())
}

async fn write(requests: usize, payload: &[u8]) -> wtx::Result<()> {
    let http_client = ClientPoolBuilder::tokio(1).build();
    let mut rrb = ReqResBuffer::empty();
    for _ in 0..requests {
        let http_client = &mut http_client.lock(&rrb.uri.to_ref()).await?.client;
        let mut grpc_client = GrpcClient::new(http_client, QuickProtobuf);
        let res = grpc_client
            .send_unary_req(
                GenericRequest {
                    generic_request_field0: Cow::Borrowed(payload),
                },
                rrb,
                &UriRef::new("http://127.0.0.1:9000/wtx.GenericService/generic_method"),
            )
            .await?;
        let generic_response: GenericResponse = grpc_client
            .des_from_res_bytes(&mut res.rrd.body.as_slice())
            .unwrap();
        assert_eq!(generic_response.generic_response_field0, payload);
        rrb = res.rrd;
    }
    Ok(())
}
