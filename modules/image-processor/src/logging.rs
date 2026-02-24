use tracing_subscriber::{EnvFilter, fmt};

pub fn init_logging() {
  let filter = EnvFilter::try_new("info,bank_api=debug").unwrap();

  let subscriber = fmt()
    .with_env_filter(filter)
    .with_target(false)
    .with_level(true)
    .without_time()
    .finish();

  let _ = tracing::subscriber::set_global_default(subscriber);
}
