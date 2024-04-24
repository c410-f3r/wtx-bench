use crate::report_line::ReportLine;

pub(crate) async fn bench_all(_: (ReportLine, &mut Vec<ReportLine>)) -> wtx::Result<()> {
    Ok(())
}
