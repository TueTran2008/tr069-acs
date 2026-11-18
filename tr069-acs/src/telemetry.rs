use tracing::subscriber::set_global_default;
use tracing::Subscriber;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::filter::{Directive, EnvFilter};
use tracing_subscriber::{layer::SubscriberExt, Registry};

pub fn get_subscriber(name: String, directive: Directive) -> impl Subscriber + Send + Sync {
    let env_filter = EnvFilter::from_default_env().add_directive(directive);
    let formatting_layer = BunyanFormattingLayer::new(name, std::io::stdout);
    Registry::default()
        .with(env_filter)
        .with(formatting_layer)
        .with(JsonStorageLayer)
    //.with(V)
}

pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    LogTracer::init().expect("Get log tracer logger failed");
    set_global_default(subscriber).expect("Failed to set subscriber");
}
