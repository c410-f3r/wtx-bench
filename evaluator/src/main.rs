#[macro_use]
mod macros;

mod bench_stats;
mod data;
mod http2;
mod language;
mod protocol;
mod report_line;
mod web_socket;

use crate::{language::Language, protocol::Protocol, report_line::ReportLine};
use bench_stats::BenchStats;
use flate2::{
    bufread::{GzDecoder, GzEncoder},
    Compression,
};
use std::{
    io::Read,
    net::{Ipv4Addr, SocketAddrV4, ToSocketAddrs},
    path::{Path, PathBuf},
    str,
    time::Duration,
};
use tokio::{
    fs::{read_dir, OpenOptions},
    io::AsyncWriteExt,
    process::Command,
    time::sleep,
};
use wtx::{
    http::{Headers, Method, Request},
    http2::{ConnectParams, Http2Buffer, Http2Tokio, ReqResBuffer},
    misc::{ArrayString, FnMutFut, GenericTime, TokioRustlsConnector, UriRef},
    rng::StaticRng,
};

const _30_DAYS: Duration = Duration::from_secs(30 * 24 * 60 * 60);
const CSV_HEADER: &str = "environment,protocol,test,implementation,timestamp,min,max,mean,sd\n";
const SOCKET_ADDR: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 9000);
const SOCKET_STR: &str = "127.0.0.1:9000";

#[tokio::main]
async fn main() {
    let environment = std::env::args()
        .nth(1)
        .as_deref()
        .unwrap_or("Teste")
        .try_into()
        .unwrap();
    let timestamp = timestamp();
    let mut rps = Vec::new();
    if cfg!(feature = "deploy") {
        manage_prev_csv(timestamp, &mut rps).await;
    }
    let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_owned();
    manage_protocols_dir(&root_dir, environment, &mut rps, timestamp).await;
    if cfg!(feature = "deploy") {
        write_csv(root_dir, &mut rps).await;
    }
}

fn decode_report(bytes: &[u8]) -> wtx::Result<String> {
    let mut gz = GzDecoder::new(bytes);
    let mut buffer = String::new();
    gz.read_to_string(&mut buffer)?;
    Ok(buffer)
}

fn encode_report(bytes: &[u8]) -> Vec<u8> {
    let mut gz = GzEncoder::new(bytes, Compression::best());
    let mut buffer = Vec::new();
    gz.read_to_end(&mut buffer).unwrap();
    buffer
}

async fn handle_cmd_output(cmd: &mut Command) {
    let output = cmd.output().await.unwrap();
    println!(
        "Stdout of command {:?} {:?}: {}",
        cmd.as_std().get_program(),
        cmd.as_std().get_args().next().unwrap(),
        str::from_utf8(&output.stdout).unwrap()
    );
    eprintln!(
        "Stderr of command {:?} {:?}: {}",
        cmd.as_std().get_program(),
        cmd.as_std().get_args().next().unwrap(),
        str::from_utf8(&output.stderr).unwrap()
    );
}

async fn manage_prev_csv(curr_timestamp: u64, rps: &mut Vec<ReportLine>) {
    let csv_fun = || async move {
        let cp = ConnectParams::default();
        let uri = UriRef::new("https://c410-f3r.github.io:443/wtx-bench/report.csv.gzip");
        let mut http2 = Http2Tokio::connect(
            cp,
            Http2Buffer::builder(StaticRng::default()).build(),
            TokioRustlsConnector::from_webpki_roots()
                .http2()
                .with_tcp_stream(
                    uri.host().to_socket_addrs()?.next().unwrap(),
                    uri.hostname(),
                )
                .await?,
        )
        .await?;
        let mut rrb = ReqResBuffer::with_capacity();
        let mut stream = http2.stream().await?;
        let res = stream
            .send_req_recv_res(
                Request::http2(&[], &Headers::new(4096), Method::Get, uri.to_ref()),
                &mut rrb,
            )
            .await?;
        wtx::Result::Ok(decode_report(res.unwrap().body())?)
    };
    let Ok(csv) = csv_fun().await else {
        return;
    };
    let lower_bound = Duration::from_millis(curr_timestamp) - _30_DAYS;
    for line in csv.split('\n').skip(1) {
        if line.is_empty() {
            continue;
        }
        let mut values = line.split(',');
        let environment = values.next().unwrap().try_into().unwrap();
        let protocol = values.next().unwrap().into();
        let test = values.next().unwrap().try_into().unwrap();
        let implementation = values.next().unwrap().try_into().unwrap();
        let timestamp = values.next().unwrap().parse().unwrap();
        let min = values.next().unwrap().parse().unwrap();
        let max = values.next().unwrap().parse().unwrap();
        let mean = values.next().unwrap().parse().unwrap();
        let sd = values.next().unwrap().parse().unwrap();
        if Duration::from_millis(timestamp) < lower_bound {
            continue;
        }
        rps.push(ReportLine {
            bench_stats: BenchStats { max, mean, min, sd },
            environment,
            implementation,
            protocol,
            test,
            timestamp,
        })
    }
}

async fn manage_protocol_dir(
    environment: ArrayString<32>,
    protocol: Protocol,
    protocol_dir: &Path,
    rps: &mut Vec<ReportLine>,
    timestamp: u64,
    mut fun: impl for<'any> FnMutFut<(ReportLine, &'any mut Vec<ReportLine>), wtx::Result<()>>,
) {
    let mut iter = read_dir(protocol_dir).await.unwrap();
    while let Some(implementation_dir_entry) = iter.next_entry().await.unwrap() {
        let mut path = implementation_dir_entry.path();
        let implementation = path.file_name().unwrap().to_str().unwrap().to_owned();
        let bytes: &[u8] = match Language::infer_from_dir(&path).await {
            Language::Go => include_bytes!("../assets/go.Dockerfile"),
            Language::Javascript => include_bytes!("../assets/javascript.Dockerfile"),
            Language::Rust => include_bytes!("../assets/rust.Dockerfile"),
        };
        path.push("Dockerfile");
        write_file(bytes, &path).await;
        println!("***** Building implementation '{implementation}' of protocol '{protocol}' *****");
        podman_build(&implementation, protocol).await;
        podman_run().await;
        sleep(Duration::from_secs(3)).await;
        println!(
            "***** Benchmarking implementation '{implementation}' of protocol '{protocol}' *****"
        );
        let rslt = fun((
            ReportLine::implementation_generic(environment, protocol, &implementation, timestamp),
            rps,
        ))
        .await;
        podman_logs().await;
        podman_rm().await;
        if let Err(err) = rslt {
            panic!("{err:?}");
        }
        sleep(Duration::from_secs(3)).await;
    }
}

async fn manage_protocols_dir(
    dir: &Path,
    environment: ArrayString<32>,
    rps: &mut Vec<ReportLine>,
    timestamp: u64,
) {
    let mut iter = read_dir(dir.join("protocols")).await.unwrap();
    while let Some(protocol) = iter.next_entry().await.unwrap() {
        let protocol_name = protocol.file_name().into_string().unwrap();
        match protocol_name.as_str() {
            "http2" => {
                manage_protocol_dir(
                    environment,
                    Protocol::Http2,
                    &protocol.path(),
                    rps,
                    timestamp,
                    http2::bench_all,
                )
                .await
            }
            "web-socket" => {
                manage_protocol_dir(
                    environment,
                    Protocol::WebSocket,
                    &protocol.path(),
                    rps,
                    timestamp,
                    web_socket::bench_all,
                )
                .await
            }
            _ => {
                panic!("'{protocol_name}' is an unknown protocol");
            }
        }
    }
}

fn manage_tests(
    mut generic_rp: ReportLine,
    rps: &mut Vec<ReportLine>,
    tests_params: impl IntoIterator<Item = (&'static str, BenchStats)>,
) {
    for test_params in tests_params {
        generic_rp.implementation_specific(test_params);
        rps.push(generic_rp.clone());
        generic_rp.implementation_clear();
    }
}

async fn podman_build(implementation: &str, protocol: Protocol) {
    handle_cmd_output(
        Command::new("podman").args([
            "build",
            "--build-arg",
            ArrayString::<64>::try_from(format_args!("IMPLEMENTATION={implementation}"))
                .unwrap()
                .as_str(),
            "-f",
            ArrayString::<64>::try_from(format_args!(
                "../protocols/{protocol}/{implementation}/Dockerfile"
            ))
            .unwrap()
            .as_str(),
            "-t",
            "bench",
        ]),
    )
    .await
}

async fn podman_logs() {
    handle_cmd_output(Command::new("podman").args(["logs", "bench"])).await;
}

async fn podman_rm() {
    handle_cmd_output(Command::new("podman").args(["rm", "-f", "bench"])).await;
}

async fn podman_run() {
    handle_cmd_output(Command::new("podman").args([
        "run",
        "-d",
        "--name",
        "bench",
        "--network",
        "host",
        "-p",
        "9000:9000",
        "bench",
    ]))
    .await
}

fn timestamp() -> u64 {
    GenericTime::now()
        .unwrap()
        .timestamp()
        .unwrap()
        .as_millis()
        .try_into()
        .unwrap()
}

async fn write_csv(mut root_dir: PathBuf, rps: &mut Vec<ReportLine>) {
    root_dir.push("site/static/report.csv.gzip");
    rps.sort_unstable_by(|a, b| {
        (b.timestamp, a.test, a.bench_stats.mean)
            .partial_cmp(&(a.timestamp, b.test, b.bench_stats.mean))
            .unwrap()
    });
    let mut string = String::from(CSV_HEADER);
    for rp in rps {
        rp.push_to_string(&mut string);
    }
    write_file(&encode_report(string.as_bytes()), &root_dir).await;
}

async fn write_file(bytes: &[u8], path: &Path) {
    OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(path)
        .await
        .unwrap()
        .write_all(bytes)
        .await
        .unwrap();
}
