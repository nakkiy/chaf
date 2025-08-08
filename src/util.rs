// Initializes logging for development builds only.
//
// - The logging level can be configured via the `RUST_LOG` environment variable (e.g., `RUST_LOG=debug`)
// - Has no effect in release builds (no-op)
#[cfg(debug_assertions)]
pub fn init_logging() {
    use tracing_subscriber::fmt::Subscriber;
    use tracing_subscriber::EnvFilter;
    // When debug_assertions is true (i.e., development build)
    {
        let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"));

        Subscriber::builder()
            .with_env_filter(filter)
            .with_writer(std::io::stderr)
            .with_line_number(true) // default=false
            .with_ansi(true)
            .init();
    }
}
#[cfg(not(debug_assertions))]
pub fn init_logging() {
    // No-op in release builds
}
