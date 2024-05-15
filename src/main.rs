use tracing::*;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::try_new("info,leptos_demo=trace").unwrap())
        .init();

	info!("Beginning");
}
