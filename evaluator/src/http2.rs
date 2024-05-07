use crate::{manage_cases, report_line::ReportLine, SOCKET_ADDR, URI_STR};
use tokio::net::TcpStream;
use wtx::{
    http::Method,
    http2::{ErrorCode, Http2Buffer, Http2Params, Http2Tokio, ReqResBuffer},
    rng::StaticRng,
};

pub(crate) async fn bench_all(
    (generic_rp, rps): (ReportLine, &mut Vec<ReportLine>),
) -> wtx::Result<()> {
    macro_rules! case {
        (($msg_size:expr, $streams:expr), $ex:expr) => {{
            let name = concat!(
                http2_connections!(),
                " connection(s) opening ",
                $streams,
                " stream(s) sending requests of ",
                $msg_size
            );
            (
                name,
                manage_case!(http2_connections!(), name, generic_rp, $ex),
            )
        }};
    }
    let params = [
        case!(("32 bytes", 1), write(1, &[4; 32]).await),
        case!(("32 bytes", 8), write(8, &[4; 32]).await),
    ];
    manage_cases(generic_rp, rps, params);
    Ok(())
}

async fn write(streams: usize, payload: &[u8]) -> wtx::Result<()> {
    let mut http2 = Http2Tokio::connect(
        Http2Buffer::new(StaticRng::default()),
        Http2Params::default(),
        TcpStream::connect(SOCKET_ADDR).await?,
    )
    .await?;
    let rrb = &mut ReqResBuffer::default();
    rrb.uri.push_str(URI_STR).unwrap();
    rrb.data.reserve(payload.len());
    rrb.data.extend_from_slice(payload).unwrap();
    for _ in 0..streams {
        let mut stream = http2.stream().await?;
        stream
            .send_req(rrb.as_http2_request_ref(Method::Get))
            .await?;
        let _res = stream.recv_res(rrb).await?;
        stream.send_reset(ErrorCode::NoError).await?;
    }
    http2.send_go_away(ErrorCode::NoError).await?;
    Ok(())
}
