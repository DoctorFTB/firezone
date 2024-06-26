//! The Firezone GUI client for Linux and Windows

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod client;

fn main() -> anyhow::Result<()> {
    client::run()
}
