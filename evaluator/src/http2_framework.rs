use crate::{manage_cases, report_line::ReportLine};
use std::sync::LazyLock;
use wtx::{
    collections::Vector,
    executor::TokioExecutor,
    http::{
        Header, HttpClient, KnownHeaderName, Method, MsgBuffer, MsgDataMut as _,
        http2_client_pool::{Http2ClientPool, Http2ClientPoolBuilder},
    },
    rng::{ChaCha20, CryptoSeedableRng},
    tls::{PlaintextCtx, TlsConfig},
};

static CF: LazyLock<Http2ClientPool<(), TokioExecutor, PlaintextCtx>> = LazyLock::new(|| {
    Http2ClientPoolBuilder::new(
        TokioExecutor::default(),
        1,
        ChaCha20::from_std_random().unwrap(),
        TlsConfig::plaintext(),
    )
    .unwrap()
    .build()
});

pub(crate) async fn bench_all(
    generic_rp: ReportLine,
    rps: &mut Vec<ReportLine>,
) -> wtx::Result<()> {
    macro_rules! case {
        ($name:expr, $ex:expr) => {
            (
                $name,
                manage_case!(http2_framework_connections!(), $name, generic_rp, $ex),
            )
        };
    }
    let params = [
        case!(
            "hello-world",
            hello_world(http2_framework_connections!()).await
        ),
        case!("serialization", json(http2_framework_connections!()).await),
    ];
    manage_cases(generic_rp, rps, params);
    Ok(())
}

async fn hello_world(streams: usize) -> wtx::Result<()> {
    let mut rrb = MsgBuffer::from_uri(String::from("http://localhost:9000/hello-world").into());
    for _ in 0..streams {
        let client = &*CF;
        rrb = client
            .send_req_recv_res(&mut Vector::new(), rrb.into_http2_request(Method::Post))
            .await
            .unwrap()
            .msg_data;
        rrb.clear()
    }
    Ok(())
}

async fn json(streams: usize) -> wtx::Result<()> {
    #[derive(serde::Serialize)]
    struct RequestElement {
        _n0: u64,
        _n1: u64,
    }

    #[derive(serde::Deserialize)]
    struct ResponseElement {
        _sum: u128,
    }

    let mut rrb = MsgBuffer::from_uri(String::from("http://localhost:9000/json").into());
    for _ in 0..streams {
        rrb.clear();
        rrb.headers.push_from_iter(Header::from_name_and_value(
            KnownHeaderName::ContentType.into(),
            ["application/json"],
        ))?;
        serde_json::to_writer(&mut rrb, &RequestElement { _n0: 4, _n1: 11 })?;
        let client = &*CF;
        rrb = client
            .send_req_recv_res(&mut Vector::new(), rrb.into_http2_request(Method::Post))
            .await
            .unwrap()
            .msg_data;
        assert_eq!(
            serde_json::from_slice::<ResponseElement>(&rrb.body)?._sum,
            15
        );
    }
    Ok(())
}
