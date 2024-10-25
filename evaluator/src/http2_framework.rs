use crate::{manage_cases, report_line::ReportLine};
use std::sync::LazyLock;
use wtx::{
    http::{client_framework::ClientFrameworkTokio, Header, KnownHeaderName, Method, ReqResBuffer},
    misc::Uri,
};

static CF: LazyLock<ClientFrameworkTokio> =
    LazyLock::new(|| ClientFrameworkTokio::tokio(1).build());

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
    CF.close_all().await;
    manage_cases(generic_rp, rps, params);
    Ok(())
}

async fn hello_world(streams: usize) -> wtx::Result<()> {
    let mut rrb = ReqResBuffer::empty();
    for _ in 0..streams {
        rrb = CF
            .send(
                Method::Get,
                rrb,
                &Uri::new("http://localhost:9000/hello-world"),
            )
            .await
            .unwrap()
            .rrd;
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

    let mut rrb = ReqResBuffer::empty();
    for _ in 0..streams {
        rrb.clear();
        rrb.headers.push_from_iter(Header::from_name_and_value(
            KnownHeaderName::ContentType.into(),
            ["application/json".as_bytes()],
        ))?;
        serde_json::to_writer(&mut rrb, &RequestElement { _n0: 4, _n1: 11 })?;
        rrb = CF
            .send(Method::Post, rrb, &Uri::new("http://localhost:9000/json"))
            .await
            .unwrap()
            .rrd;
        assert_eq!(
            serde_json::from_slice::<ResponseElement>(&rrb.body)?._sum,
            15
        );
    }
    Ok(())
}
