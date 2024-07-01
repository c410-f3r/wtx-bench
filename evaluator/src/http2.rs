use crate::{data::string_bytes_8kib, manage_cases, report_line::ReportLine, SOCKET_ADDR, URI_STR};
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
    let params = [case!(("8 KiB", 1), write(1, string_bytes_8kib()).await)];
    manage_cases(generic_rp, rps, params);
    Ok(())
}

async fn write(streams: usize, payload: &'static [u8]) -> wtx::Result<()> {
    let mut http2 = Http2Tokio::connect(
        Http2Buffer::<Box<StreamBuffer>>::new(StaticRng::default()),
        Http2Params::default(),
        TcpStream::connect(SOCKET_ADDR).await?,
    )
    .await?;
    for _ in 0..streams {
        let mut sb = Box::new(StreamBuffer::default());
        let mut stream = http2.stream().await.unwrap();
        stream
            .send_req(
                &mut sb.hpack_enc_buffer,
                Request::http2(payload, Method::Get, UriRef::new(URI_STR)),
            )
            .await
            .unwrap();
        let res = stream.recv_res(sb).await.unwrap();
        assert_eq!(res.0.rrb.body.as_ref(), payload);
    }
    http2.send_go_away(Http2ErrorCode::NoError).await;
    Ok(())
}
