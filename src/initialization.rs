//! Provides startup-time initialization for the application runtime.
//!
//! This includes:
//! - Logging setup (e.g., tracing subscriber / env filter)
//! - Panic hook configuration


use tracing::Level;
use tracing_subscriber::{
    fmt,
    Registry,
    filter::{
        self,
        Targets,
    },
    layer::{
        Layer,
        Layered,
        SubscriberExt,
    },
};


/// Logging and panic-hook setup.
pub fn setup_logging(package_name: &str, log_level: Level) {
    //✅ Logging.
    let fmt_layer: Box<dyn Layer<Layered<Targets, Registry>> + Send + Sync> =
        fmt::layer()
            .with_ansi(true)
            .with_level(true)
            .with_target(true)
            .with_writer(std::io::stderr)
            .boxed();
    let subscriber = Registry::default()
        .with(
            filter::Targets::new()
            .with_target(package_name, log_level)
            .with_target("other_crate", log_level)
        )
        .with(fmt_layer)
        ;
    tracing::subscriber::set_global_default(subscriber)
        .expect("❌ Failed to set global default subscriber");

    //✅ Panic-hook.
    std::panic::set_hook(Box::new(|info| {
        eprintln!("PANIC OCCURRED: {info:?}");
    }));        
}
