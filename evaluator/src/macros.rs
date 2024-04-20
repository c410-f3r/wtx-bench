macro_rules! connections {
    () => {
        128
    };
}

macro_rules! manage_bench {
    ($ex:expr) => {{
        let mut data = [0.0; connections!()];
        let mut set = tokio::task::JoinSet::new();
        for idx in 0..connections!() {
            let _handle = set.spawn(async move {
                let now = wtx::misc::GenericTime::now().unwrap();
                $ex;
                (idx, now.elapsed().unwrap().as_millis() as f64)
            });
        }
        while let Some(rslt) = set.join_next().await {
            let (idx, value) = rslt.unwrap();
            data[idx] = value;
        }
        BenchStats::new(&data)
    }};
}
