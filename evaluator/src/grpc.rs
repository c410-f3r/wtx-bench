pub mod grpc_bindings {
    include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));
}

use crate::{data::string_bytes_8kib, manage_cases, report_line::ReportLine};
use grpc_bindings::wtx::{GenericRequest, GenericResponse};
use std::borrow::Cow;
use wtx::{
    codec::format::QuickProtobuf,
    executor::TokioExecutor,
    grpc::GrpcClient,
    http::{MsgBuffer, http2_client_pool::Http2ClientPoolBuilder},
    rng::{ChaCha20, CryptoSeedableRng},
    tls::TlsConfig,
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
    let http_client = Http2ClientPoolBuilder::new(
        TokioExecutor::default(),
        1,
        ChaCha20::from_std_random()?,
        TlsConfig::plaintext(),
    )
    .unwrap()
    .build();
    let mut buffer = MsgBuffer::from_uri(
        String::from("http://127.0.0.1:9000/wtx.GenericService/generic_method").into(),
    );
    for _ in 0..requests {
        let http_client = &mut http_client.lock(&buffer.uri.to_ref()).await?.client;
        let mut grpc_client = GrpcClient::new(http_client, QuickProtobuf);
        let res = grpc_client
            .send_unary_req(
                GenericRequest {
                    generic_request_field0: Cow::Borrowed(payload),
                },
                buffer,
            )
            .await?;
        let generic_response: GenericResponse = grpc_client
            .des_from_res_bytes(&mut res.msg_data.body.as_slice())
            .unwrap();
        assert_eq!(generic_response.generic_response_field0, payload);
        buffer = res.msg_data;
    }
    Ok(())
}
