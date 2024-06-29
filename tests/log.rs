use tokio::runtime::Handle;
use tracing::{info, span, Level};
use tracing_perfetto::*;
use tracing_subscriber::fmt::format::Format;
use tracing_subscriber::{fmt, layer::SubscriberExt, Registry};

#[tokio::test]
async fn write() -> anyhow::Result<()> {
    let file = std::env::temp_dir().join("test.pftrace");
    let perfetto_layer =
        PerfettoSubscriber::new(std::sync::Mutex::new(std::fs::File::create(&file)?));

    let fmt_layer = fmt::layer()
        .with_writer(std::io::stdout)
        .event_format(Format::default().with_thread_ids(true))
        .with_span_events(fmt::format::FmtSpan::FULL);

    let subscriber = Registry::default().with(fmt_layer).with(perfetto_layer);

    tracing::subscriber::set_global_default(subscriber)?;

    info!(?file, "start");

    let demo_span = span!(Level::TRACE, "demo_span");
    let _enter = demo_span.enter();

    info!("in span");
    sync_fn();
    let handle = Handle::current();
    let t = std::thread::spawn(move || {
        handle.spawn(async_fn());
    });
    t.join().unwrap();

    _ = tokio::spawn(async_fn()).await;
    Ok(())
}

#[tracing::instrument]
fn sync_fn() {
    info!("inside function");
    sync_inner();
}

#[tracing::instrument]
fn sync_inner() {
    let x = 1;
    info!(x, "inner");
}

#[tracing::instrument]
async fn async_fn() {
    info!("test");
    async_inner().await;
}

#[tracing::instrument]
async fn async_inner() {
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
}
