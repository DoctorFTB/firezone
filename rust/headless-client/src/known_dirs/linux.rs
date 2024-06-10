use connlib_shared::BUNDLE_ID;
use std::path::PathBuf;

#[allow(clippy::unnecessary_wraps)]
pub fn ipc_service_logs() -> Option<PathBuf> {
    // TODO: This is magic, it must match the systemd file
    Some(PathBuf::from("/var/log").join(connlib_shared::BUNDLE_ID))
}

/// e.g. `/home/alice/.cache/dev.firezone.client/data/logs`
///
/// Logs are considered cache because they're not configs and it's technically okay
/// if the system / user deletes them to free up space
pub fn logs() -> Option<PathBuf> {
    Some(dirs::cache_dir()?.join(BUNDLE_ID).join("data").join("logs"))
}

/// e.g. `/run/user/1000/dev.firezone.client/data`
///
/// Crash handler socket and other temp files go here
pub fn runtime() -> Option<PathBuf> {
    Some(dirs::runtime_dir()?.join(BUNDLE_ID).join("data"))
}

/// e.g. `/home/alice/.local/share/dev.firezone.client/data`
///
/// Things like actor name are stored here because they're kind of config,
/// the system / user should not delete them to free up space, but they're not
/// really config since the program will rewrite them automatically to persist sessions.
pub fn session() -> Option<PathBuf> {
    Some(dirs::data_local_dir()?.join(BUNDLE_ID).join("data"))
}

/// e.g. `/home/alice/.config/dev.firezone.client/config`
///
/// See connlib docs for details
pub fn settings() -> Option<PathBuf> {
    Some(dirs::config_local_dir()?.join(BUNDLE_ID).join("config"))
}