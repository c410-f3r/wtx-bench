use crate::{manage_cases, report_line::ReportLine, SOCKET_ADDR, URI_STR};
use tokio::net::TcpStream;
use wtx::{
    http::{Method, Request},
    http2::{Http2Buffer, Http2ErrorCode, Http2Params, Http2Tokio, StreamBuffer},
    misc::UriRef,
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
        //case!(("32 bytes", 2), write(8, &[4; 32]).await),
    ];
    manage_cases(generic_rp, rps, params);
    Ok(())
}

async fn write(streams: usize, payload: &'static [u8]) -> wtx::Result<()> {
    let mut http2 = Http2Tokio::connect(
        Http2Buffer::new(StaticRng::default()),
        Http2Params::default(),
        TcpStream::connect(SOCKET_ADDR).await?,
    )
    .await?;
    let mut set = tokio::task::JoinSet::new();
    for _ in 0..streams {
        let uri = UriRef::new(URI_STR);
        let mut sb = Box::new(StreamBuffer::default());
        let mut stream = http2.stream().await.unwrap();
        let _handle = set.spawn(async move {
            stream
                .send_req(
                    &mut sb.hpack_enc_buffer,
                    Request::http2(payload, Method::Get, uri),
                )
                .await
                .unwrap();
            let res = stream.recv_res(sb).await.unwrap();
            assert_eq!(res.0.rrb.body.as_ref(), payload);
            stream.send_reset(Http2ErrorCode::NoError).await;
        });
    }
    while let Some(rslt) = set.join_next().await {
        rslt.unwrap();
    }
    http2.send_go_away(Http2ErrorCode::NoError).await;
    Ok(())
}
