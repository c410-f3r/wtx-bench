use crate::{bench_stats::BenchStats, protocol::Protocol};
use std::fmt::Write;
use wtx::collection::ArrayStringU8;

#[derive(Clone, Debug)]
pub(crate) struct ReportLine {
    pub(crate) bench_stats: BenchStats,
    pub(crate) environment: ArrayStringU8<32>,
    pub(crate) implementation: ArrayStringU8<24>,
    pub(crate) protocol: Protocol,
    pub(crate) test: ArrayStringU8<96>,
    pub(crate) timestamp: u64,
}

impl ReportLine {
    pub(crate) fn implementation_generic(
        environment: ArrayStringU8<32>,
        protocol: Protocol,
        implementation: &str,
        timestamp: u64,
    ) -> Self {
        Self {
            bench_stats: BenchStats::default(),
            environment,
            implementation: ArrayStringU8::try_from(implementation).unwrap(),
            protocol,
            test: ArrayStringU8::new(),
            timestamp,
        }
    }

    pub(crate) fn implementation_clear(&mut self) {
        self.bench_stats = BenchStats::default();
        self.test.clear();
    }

    pub(crate) fn implementation_specific(&mut self, (test, bench_stats): (&str, BenchStats)) {
        self.bench_stats = bench_stats;
        self.test.push_str(test).unwrap();
    }

    pub(crate) fn push_to_string(&self, string: &mut String) {
        let Self {
            bench_stats: BenchStats { max, mean, min, sd },
            environment,
            implementation,
            protocol,
            test,
            timestamp,
        } = self;
        string.write_fmt(format_args!("{environment},{protocol},{test},{implementation},{timestamp},{min},{max},{mean},{sd}\n")).unwrap();
    }
}
