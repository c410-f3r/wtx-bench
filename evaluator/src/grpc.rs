pub mod grpc_bindings {
    include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));
}

use crate::{data::string_bytes_8kib, manage_cases, report_line::ReportLine};
use grpc_bindings::wtx::{GenericRequest, GenericResponse};
use std::borrow::Cow;
use wtx::{
    data_transformation::dnsn::QuickProtobuf,
    grpc::Client,
    http::{client_framework::ClientFrameworkTokio, ReqResBuffer},
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
    let mut client = Client::new(ClientFrameworkTokio::tokio(1).build(), QuickProtobuf);
    let mut rrb = ReqResBuffer::default();
    for _ in 0..requests {
        rrb.uri
            .reset(format_args!("http://127.0.0.1:9000"))
            .unwrap();
        let res = client
            .send_unary_req(
                ("wtx", "GenericService", "generic_method"),
                GenericRequest {
                    generic_request_field0: Cow::Borrowed(payload),
                },
                rrb,
            )
            .await?;
        let generic_response: GenericResponse = client.des_from_res_bytes(&res.rrd.data).unwrap();
        assert_eq!(generic_response.generic_response_field0, payload);
        rrb = res.rrd;
    }
    Ok(())
}
