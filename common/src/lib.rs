use tokio::net::TcpStream;

pub fn bench_stream(stream: &TcpStream) -> std::io::Result<()> {
    stream.set_nodelay(true)?;
    stream.set_quickack(true)?;
    Ok(())
}
