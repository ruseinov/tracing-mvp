pub mod recorder;

use metrics::{counter, gauge, histogram};
use std::time::Instant;
use tracing::span::Attributes;
use tracing::Id;
use tracing_subscriber::layer::Context;
use tracing_subscriber::Layer;

struct Timed {
    start: Instant,
}

pub struct TimedMetrics;

impl<S> Layer<S> for TimedMetrics
where
    S: tracing::Subscriber,
    S: for<'lookup> tracing_subscriber::registry::LookupSpan<'lookup>,
{
    fn on_new_span(&self, _: &Attributes<'_>, id: &Id, ctx: Context<'_, S>) {
        let span = ctx.span(id);
        if let Some(span) = span {
            span.extensions_mut().insert(Timed {
                start: Instant::now(),
            });
        }
    }

    fn on_close(&self, id: Id, ctx: Context<'_, S>) {
        let span = ctx.span(&id);
        if let Some(span) = span {
            let name = span.metadata().name();
            if let Some(timed) = span.extensions().get::<Timed>() {
                counter!(format!("{}_cnt", name)).increment(1);
                gauge!(format!("{}_gauge", name)).increment(timed.start.elapsed());
                histogram!(format!("{}_histogram", name)).record(timed.start.elapsed());
            }
        }
    }
}
