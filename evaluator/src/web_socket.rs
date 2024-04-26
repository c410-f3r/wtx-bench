use crate::{
    bench_stats::BenchStats,
    data::{string_bytes_2mib, string_bytes_8kib},
    manage_tests,
    report_line::ReportLine,
    SOCKET_ADDR, SOCKET_STR,
};
use tokio::net::TcpStream;
use wtx::{
    misc::UriRef,
    rng::StaticRng,
    web_socket::{
        handshake::{HeadersBuffer, WebSocketConnect, WebSocketConnectRaw},
        FrameBufferVec, FrameMutVec, OpCode, WebSocketBuffer,
    },
};

pub(crate) async fn bench_all(
    (generic_rp, rps): (ReportLine, &mut Vec<ReportLine>),
) -> wtx::Result<()> {
    macro_rules! name {
        ($msgs:expr, $msg_size:expr, $frames:expr) => {
            concat!(
                connections!(),
                " connection(s) sending ",
                $msgs,
                " text message(s) of ",
                $msg_size,
                " composed by ",
                $frames,
                " frame(s)"
            )
        };
    }

    manage_tests(
        generic_rp,
        rps,
        [
            (
                name!(1, "8 KiB", 1),
                manage_bench!(write((1, 1), string_bytes_8kib()).await),
            ),
            (
                name!(1, "2 MiB", 1),
                manage_bench!(write((1, 1), string_bytes_2mib()).await),
            ),
            (
                name!(1, "8 KiB", 64),
                manage_bench!(write((1, 64), string_bytes_8kib()).await),
            ),
            (
                name!(1, "2 MiB", 64),
                manage_bench!(write((1, 64), string_bytes_2mib()).await),
            ),
            (
                name!(64, "8 KiB", 1),
                manage_bench!(write((64, 1), string_bytes_8kib()).await),
            ),
            (
                name!(64, "2 MiB", 1),
                manage_bench!(write((64, 1), string_bytes_2mib()).await),
            ),
            (
                name!(64, "8 KiB", 64),
                manage_bench!(write((64, 64), string_bytes_8kib()).await),
            ),
            (
                name!(64, "2 MiB", 64),
                manage_bench!(write((64, 64), string_bytes_2mib()).await),
            ),
        ],
    );
    Ok(())
}

async fn write((frames, msgs): (usize, usize), payload: &[u8]) -> wtx::Result<()> {
    let fb = &mut FrameBufferVec::default();
    let uri = &UriRef::new(SOCKET_STR);
    let mut ws = WebSocketConnectRaw {
        compression: (),
        fb,
        headers_buffer: &mut HeadersBuffer::default(),
        rng: StaticRng::default(),
        stream: TcpStream::connect(SOCKET_ADDR).await?,
        uri,
        wsb: WebSocketBuffer::default(),
    }
    .connect([])
    .await?
    .1;
    for _ in 0..msgs {
        let mut iter = payload.chunks(payload.len() / frames);
        let Some(first) = iter.next() else {
            panic!("No frames are being measured");
        };
        if let Some(last) = iter.next_back() {
            ws.write_frame(&mut FrameMutVec::new_unfin(fb, OpCode::Text, first)?)
                .await?;
            for elem in iter {
                ws.write_frame(&mut FrameMutVec::new_unfin(fb, OpCode::Continuation, elem)?)
                    .await?;
            }
            ws.write_frame(&mut FrameMutVec::new_fin(fb, OpCode::Continuation, last)?)
                .await?;
        } else {
            ws.write_frame(&mut FrameMutVec::new_fin(fb, OpCode::Text, first)?)
                .await?;
        }
        assert_eq!(ws.read_frame(fb).await?.fb().payload().len(), payload.len());
    }
    ws.write_frame(&mut FrameMutVec::new_fin(fb, OpCode::Close, &[])?)
        .await?;
    Ok(())
}
