use std::thread;
use std::time::Duration;

use metrics::histogram;
use metrics_exporter_prometheus::PrometheusBuilder;
use metrics_util::MetricKindMask;

fn main() {
    tracing_subscriber::fmt::init();

    let builder = PrometheusBuilder::new();
    builder
        .set_quantiles(&[0.0, 0.1, 0.2, 0.5, 0.9, 0.95, 0.99, 0.999, 1.0])
        .unwrap()
        .idle_timeout(
            MetricKindMask::COUNTER | MetricKindMask::HISTOGRAM,
            Some(Duration::from_secs(10)),
        )
        .install()
        .unwrap();

    // the first request takes 20s due to some resources needing to be initialized
    histogram!("request_time_seconds").record(20.0);

    // the following requests are significantly faster at 0.1 - 0.6ms
    let values = [0.0001, 0.0002, 0.0003, 0.0004, 0.0005, 0.0006];
    let mut index = 0;

    loop {
        histogram!("request_time_seconds").record(values[index % values.len()]);

        thread::sleep(Duration::from_millis(10));

        index += 1;
    }
}
