mod tun_device_manager;

use clap::Args;
use tracing_log::LogTracer;
use tracing_subscriber::{
    fmt, prelude::__tracing_subscriber_SubscriberExt, EnvFilter, Layer, Registry,
};
use url::Url;

/// Mark for Firezone sockets to prevent routing loops on Linux.
pub const FIREZONE_MARK: u32 = 0xfd002021;

#[cfg(any(target_os = "linux", target_os = "windows"))]
pub use tun_device_manager::TunDeviceManager;

pub fn setup_global_subscriber<L>(additional_layer: L)
where
    L: Layer<Registry> + Send + Sync,
{
    let subscriber = Registry::default()
        .with(additional_layer.with_filter(EnvFilter::from_default_env()))
        .with(fmt::layer().with_filter(EnvFilter::from_default_env()));
    tracing::subscriber::set_global_default(subscriber).expect("Could not set global default");
    LogTracer::init().unwrap();
}

/// Arguments common to all Firezone CLI components.
#[derive(Args, Clone)]
pub struct CommonArgs {
    #[arg(
        short = 'u',
        long,
        hide = true,
        env = "FIREZONE_API_URL",
        default_value = "wss://api.firezone.dev"
    )]
    pub api_url: Url,
    /// Token generated by the portal to authorize websocket connection.
    #[arg(env = "FIREZONE_TOKEN")]
    pub token: String,
    /// Friendly name to display in the UI
    #[arg(short = 'n', long, env = "FIREZONE_NAME")]
    pub firezone_name: Option<String>,
}