use crate::{
    data::{string_bytes_2mib, string_bytes_8kib},
    manage_cases,
    report_line::ReportLine,
    SOCKET_ADDR, SOCKET_STR,
};
use tokio::net::TcpStream;
use wtx::{
    misc::NoStdRng,
    misc::UriRef,
    web_socket::{FrameBufferVec, FrameMutVec, HeadersBuffer, OpCode, WebSocket, WebSocketBuffer},
};

pub(crate) async fn bench_all(
    generic_rp: ReportLine,
    rps: &mut Vec<ReportLine>,
) -> wtx::Result<()> {
    macro_rules! case {
        (($msgs:expr, $msg_size:expr, $frames:expr), $ex:expr) => {{
            let name = concat!(
                web_socket_connections!(),
                " connection(s) sending ",
                $msgs,
                " text message(s) of ",
                $msg_size,
                " composed by ",
                $frames,
                " frame(s)"
            );
            (
                name,
                manage_case!(web_socket_connections!(), name, generic_rp, $ex),
            )
        }};
    }
    let params = [
        case!((1, "8 KiB", 1), write((1, 1), string_bytes_8kib()).await),
        case!((1, "2 MiB", 1), write((1, 1), string_bytes_2mib()).await),
        case!((1, "8 KiB", 64), write((1, 64), string_bytes_8kib()).await),
        case!((1, "2 MiB", 64), write((1, 64), string_bytes_2mib()).await),
        case!((64, "8 KiB", 1), write((64, 1), string_bytes_8kib()).await),
        case!((64, "2 MiB", 1), write((64, 1), string_bytes_2mib()).await),
        case!(
            (64, "8 KiB", 64),
            write((64, 64), string_bytes_8kib()).await
        ),
        case!(
            (64, "2 MiB", 64),
            write((64, 64), string_bytes_2mib()).await
        ),
    ];
    manage_cases(generic_rp, rps, params);
    Ok(())
}

async fn write((frames, msgs): (usize, usize), payload: &[u8]) -> wtx::Result<()> {
    let fb = &mut FrameBufferVec::default();
    let uri = &UriRef::new(SOCKET_STR);
    let mut ws = WebSocket::connect(
        (),
        fb,
        [],
        &mut HeadersBuffer::default(),
        NoStdRng::default(),
        TcpStream::connect(SOCKET_ADDR).await?,
        uri,
        WebSocketBuffer::default(),
    )
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
