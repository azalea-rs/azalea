use std::sync::LazyLock;

use bevy_log::tracing_subscriber::{
    self, EnvFilter, Layer, Registry,
    filter::Filtered,
    fmt,
    layer::{Context, SubscriberExt},
    reload::{self, Handle},
    util::SubscriberInitExt,
};
use parking_lot::{Mutex, MutexGuard};
use tracing::{Event, Level, Subscriber};

static LOCK: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

pub fn init<'a>() -> MutexGuard<'a, ()> {
    init_with_level(Level::WARN)
}

// can't treat these as one layer due to https://github.com/tokio-rs/tracing/issues/1629
struct LayerReloadHandles {
    filter: Handle<Filtered<fmt::Layer<Registry>, EnvFilter, Registry>, Registry>,
    test: Handle<TestTracingLayer, Registry>,
}

static RELOAD_HANDLES: LazyLock<LayerReloadHandles> = LazyLock::new(|| {
    let (filter_layer, filter_reload_handle) = reload::Layer::new(
        fmt::layer().with_filter(
            EnvFilter::builder()
                .with_default_directive(Level::WARN.into())
                .from_env_lossy(),
        ),
    );
    let (test_layer, test_reload_handle) = reload::Layer::new(TestTracingLayer {
        panic_on_level: Level::WARN,
    });

    tracing_subscriber::registry()
        .with(filter_layer.and_then(test_layer))
        .init();

    LayerReloadHandles {
        filter: filter_reload_handle,
        test: test_reload_handle,
    }
});

pub fn init_with_level<'a>(max_level: Level) -> MutexGuard<'a, ()> {
    let lock = LOCK.lock();

    RELOAD_HANDLES
        .filter
        .modify(|layer| {
            *layer.filter_mut() = EnvFilter::builder()
                .with_default_directive(max_level.into())
                .from_env_lossy()
        })
        .unwrap();
    RELOAD_HANDLES
        .test
        .modify(|layer| layer.panic_on_level = max_level)
        .unwrap();

    lock
}

struct TestTracingLayer {
    panic_on_level: Level,
}
impl<S: Subscriber> Layer<S> for TestTracingLayer {
    fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
        let level = *event.metadata().level();
        if level <= self.panic_on_level {
            panic!("Logged on level {level}.");
        }
    }
}
