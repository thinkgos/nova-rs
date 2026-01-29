use tracing_subscriber::{
    EnvFilter, Registry, fmt,
    fmt::{MakeWriter, time},
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

pub fn try_init_subscriber<Sink>(default_env_filter: &str, sink: Sink) -> anyhow::Result<()>
where
    // This "weird" syntax is a higher-ranked trait bound (HRTB)
    // It basically means that Sink implements the `MakeWriter`
    // trait for all choices of the lifetime parameter `'a`
    // Check out https://doc.rust-lang.org/nomicon/hrtb.html
    // for more details.
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    // if the RUST_LOG environment variable has not been set, then use default.
    let layer_env_filter = EnvFilter::builder()
        .with_default_directive(default_env_filter.parse()?)
        .from_env_lossy();
    let layer_formatting = fmt::layer()
        .json()
        .with_timer(time::LocalTime::rfc_3339())
        .with_line_number(true)
        .with_writer(sink)
        .with_file(true);
    // The `with` method is provided by `SubscriberExt`, an extension
    // trait for `Subscriber` exposed by `tracing_subscriber`
    Registry::default()
        .with(layer_env_filter)
        .with(layer_formatting)
        .try_init()?;
    Ok(())
}
