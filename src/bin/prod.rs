use std::env;
use std::thread::sleep;
use std::time::Duration;
use tracing::{debug, instrument, trace};
use tracing_subscriber::{EnvFilter, Layer, Registry};
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
#[cfg(feature = "enable-metrics")]
use tracing_mvp::metrics::TimedMetrics;


fn tracing_init() {
    let config_name = env::var("TRACING_CONFIG").unwrap_or("default".to_string());

    let log_layer = tracing_subscriber::fmt::layer();

    let layered = match config_name.as_str() {
        "detailed-spans" => {
            log_layer
                // Log span events. Could potentially be it's own env variable similar to how it's
                // done in test_log.
                .with_span_events(FmtSpan::CLOSE)
        },
        _ =>  {
            log_layer
        },
    };

    let layered = layered.with_filter(EnvFilter::from_default_env());
    let sub = Registry::default().with(layered);

    #[cfg(feature = "tracing-tree")]
    let sub = sub.with(tracing_span_tree::span_tree());

    #[cfg(feature = "enable-metrics")]
    let sub = sub.with(TimedMetrics);

    sub.init()
}

fn main() {
    #[cfg(feature = "enable-metrics")]
    tracing_mvp::metrics::recorder::init_print_logger();
    tracing_init();
    root_method();

}

#[instrument(level = "debug")]
fn root_method() {
    some_method_to_trace(20);
    some_other_method_to_trace(30);
}

#[tracing::instrument(level = "debug")]
fn some_method_to_trace(duration: u64) {
    sleep(Duration::from_millis(duration));
    debug!("some_method_to_trace has been called");
}

#[tracing::instrument(level = "trace", skip_all)]
fn some_other_method_to_trace(duration: u64) {
    sleep(Duration::from_millis(duration));
    trace!("some_method_to_trace has been called");
}
