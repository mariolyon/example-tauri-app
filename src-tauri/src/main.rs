// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;

use db::Config;
use serde;


#[derive(Debug, Clone, serde::Serialize)]
struct Settings {
    config: Option<Config>
}

#[tauri::command]
fn save(name: &str, email: &str) -> String {
    let _ = db::init();
    let result = db::insert(Config {name: name.to_string(), email: email.to_string()});

    match result {
        Ok(_) => format!("Ok Saving settings: {} {}", name, email),
        Err(err) => format!("Error {}", err)
    }
}


#[tauri::command]
fn load() -> Settings {
    let _ = db::init();
    match db::read() {
        Ok(config) => Settings{ config: Some(config) },
        _ => Settings{ config: None }
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![save, load])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
