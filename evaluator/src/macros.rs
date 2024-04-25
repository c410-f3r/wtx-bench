#[cfg(feature = "deploy")]
macro_rules! connections {
    () => {
        64
    };
}
#[cfg(not(feature = "deploy"))]
macro_rules! connections {
    () => {
        1
    };
}

macro_rules! manage_bench {
    ($ex:expr) => {{
        let mut data = [0.0; connections!()];
        let mut set = tokio::task::JoinSet::new();
        for idx in 0..connections!() {
            let _handle = set.spawn(async move {
                let now = wtx::misc::GenericTime::now().unwrap();
                $ex?;
                Ok::<_, wtx::Error>((idx, now.elapsed().unwrap().as_millis() as f64))
            });
        }
        while let Some(rslt) = set.join_next().await {
            let (idx, value) = rslt.unwrap()?;
            data[idx] = value;
        }
        BenchStats::new(&data)
    }};
}
