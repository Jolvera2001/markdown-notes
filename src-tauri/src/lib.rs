mod document;

use tauri::State;

use crate::document::{Document, DocumentState};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn load_files(documents: State<'_, DocumentState>) -> Result<Vec<Document>, String> {
    documents.fetch_test_files().await; // TODO: add .map_err for proper error handling
    Ok(documents.get_documents())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(DocumentState::new())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
