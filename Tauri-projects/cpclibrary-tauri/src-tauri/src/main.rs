// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// -- Sub-Modules
mod ctx;
mod error;
mod event;
mod ipc;
mod model;
mod prelude;
mod utils;

pub use error::{Error, Result};
pub use ipc::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
struct LoginPayload {
    email: String,
    password: String,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn login(data: String) -> String {
    format!("{}", data).into()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
