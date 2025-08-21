use crate::{
    SOCKET_ADDR, SOCKET_STR,
    data::{string_bytes_2mib, string_bytes_8kib},
    manage_cases,
    report_line::ReportLine,
};
use wtx::web_socket::WebSocketPayloadOrigin;
use tokio::net::TcpStream;
use wtx::{
    collection::Vector, misc::UriRef, web_socket::{Frame, OpCode, WebSocketConnector}
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
    let uri = &UriRef::new(SOCKET_STR);
    let mut buffer = Vector::new();
    let mut ws = WebSocketConnector::default()
        .connect(TcpStream::connect(SOCKET_ADDR).await?, uri)
        .await?;
    for _ in 0..msgs {
        let mut iter = payload.chunks(payload.len() / frames);
        let Some(first) = iter.next() else {
            panic!("No frames are being measured");
        };
        if let Some(last) = iter.next_back() {
            ws.write_frame(&mut Frame::new_unfin(OpCode::Text, first.to_vec()))
                .await?;
            for elem in iter {
                ws.write_frame(&mut Frame::new_unfin(OpCode::Continuation, elem.to_vec()))
                    .await?;
            }
            ws.write_frame(&mut Frame::new_fin(OpCode::Continuation, last.to_vec()))
                .await?;
        } else {
            ws.write_frame(&mut Frame::new_fin(OpCode::Text, first.to_vec()))
                .await?;
        }
        assert_eq!(ws.read_frame(&mut buffer, WebSocketPayloadOrigin::Adaptive).await?.payload().len(), payload.len());
    }
    ws.write_frame(&mut Frame::new_fin(OpCode::Close, &mut []))
        .await?;
    Ok(())
}
