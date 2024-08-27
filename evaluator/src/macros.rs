#[cfg(feature = "deploy")]
macro_rules! grpc_connections {
    () => {
        32
    };
}
#[cfg(feature = "deploy")]
macro_rules! http2_framework_connections {
    () => {
        32
    };
}
#[cfg(feature = "deploy")]
macro_rules! web_socket_connections {
    () => {
        64
    };
}

#[cfg(not(feature = "deploy"))]
macro_rules! grpc_connections {
    () => {
        1
    };
}
#[cfg(not(feature = "deploy"))]
macro_rules! http2_framework_connections {
    () => {
        1
    };
}
#[cfg(not(feature = "deploy"))]
macro_rules! web_socket_connections {
    () => {
        1
    };
}

macro_rules! manage_case {
    ($n:expr, $name:expr, $generic_rp:expr, $ex:expr) => {{
        println!(
            "***** Running case '{}'. Implementation '{}' of protocol '{}' *****",
            $name, &$generic_rp.implementation, &$generic_rp.protocol
        );
        let mut data = [0.0; $n];
        let mut set = tokio::task::JoinSet::new();
        for idx in 0..$n {
            let _handle = set.spawn(async move {
                let now = wtx::misc::GenericTime::now();
                $ex?;
                Ok::<_, wtx::Error>((idx, now.elapsed().unwrap().as_millis() as f64))
            });
        }
        while let Some(rslt) = set.join_next().await {
            let (idx, value) = rslt.unwrap()?;
            data[idx] = value;
        }
        crate::BenchStats::new(&data)
    }};
}
