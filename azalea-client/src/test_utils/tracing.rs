use bevy_log::tracing_subscriber::{
    self, EnvFilter, Layer,
    layer::{Context, SubscriberExt},
    util::SubscriberInitExt,
};
use tracing::{Event, Level, Subscriber};

pub fn init_tracing() {
    init_tracing_with_level(Level::WARN);
}

pub fn init_tracing_with_level(max_level: Level) {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer().with_filter(
                EnvFilter::builder()
                    .with_default_directive(max_level.into())
                    .from_env_lossy(),
            ),
        )
        .with(TestTracingLayer {
            panic_on_level: max_level,
        })
        .init();
}

struct TestTracingLayer {
    panic_on_level: Level,
}
impl<S: Subscriber> Layer<S> for TestTracingLayer {
    fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
        let level = *event.metadata().level();
        if level <= self.panic_on_level {
            panic!("logged on level {level}");
        }
    }
}
