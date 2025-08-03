/// ログの初期化（開発ビルド限定）
///
/// - `RUST_LOG=debug` 等の環境変数でログレベルを制御可能
/// - リリースビルドでは呼び出されても何も起きない（無効化）
#[cfg(debug_assertions)]
pub fn init_logging() {
    use tracing_subscriber::fmt::Subscriber;
    use tracing_subscriber::EnvFilter;
    // debug_assertions が true のとき（= devビルド）
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
    // リリースビルドでは何もしない（空定義）
}
