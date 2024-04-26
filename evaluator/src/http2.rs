use crate::{
    bench_stats::BenchStats, manage_tests, report_line::ReportLine, SOCKET_ADDR, SOCKET_STR,
};
use tokio::net::TcpStream;
use wtx::{
    http::{Headers, Method, Request},
    http2::{ConnectParams, Http2Buffer, Http2Tokio, ReqResBuffer},
    misc::UriRef,
    rng::StaticRng,
};

pub(crate) async fn bench_all(
    (generic_rp, rps): (ReportLine, &mut Vec<ReportLine>),
) -> wtx::Result<()> {
    macro_rules! name {
        ($msg_size:expr) => {
            concat!(
                connections!(),
                " connection(s) opening one stream that sends requests of ",
                $msg_size
            )
        };
    }

    manage_tests(
        generic_rp,
        rps,
        [(name!("0B"), manage_bench!(write().await))],
    );
    Ok(())
}

async fn write() -> wtx::Result<()> {
    let uri = &UriRef::new(SOCKET_STR);
    let mut http2 = Http2Tokio::connect(
        ConnectParams::default(),
        Http2Buffer::builder(StaticRng::default()).build(),
        TcpStream::connect(SOCKET_ADDR).await?,
    )
    .await?;
    let rrb = &mut ReqResBuffer::default();
    let mut stream = http2.stream().await?;
    let _res = stream
        .send_req_recv_res(
            Request::http2(&[], &Headers::new(4096), Method::Get, uri.to_ref()),
            rrb,
        )
        .await;
    Ok(())
}
