use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

pub fn init_tracing() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;
    info!("Initialized tracing");

    Ok(())
}
