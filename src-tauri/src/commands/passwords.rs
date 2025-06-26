use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use tauri::{AppHandle, Manager};

#[derive(Serialize, Deserialize)]
pub struct PwEntry {
    title: String,
    username: String,
    password: String,
}

#[tauri::command]
pub fn load_passwords(app: AppHandle) -> Result<Vec<PwEntry>, String> {
    let mut path: PathBuf = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Konnte app_data_dir nicht aufloesen: {}", e))?;
    path.push("passwords.json");

    let entries: Vec<PwEntry> = match fs::read_to_string(&path) {
        Ok(s) => serde_json::from_str::<Vec<PwEntry>>(&s)
            .map_err(|e| format!("JSON Parsing Fehler: {}", e))?,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Vec::new(),
        Err(err) => return Err(format!("Datei-Lese-Fehler: {}", err)),
    };

    Ok(entries)
}