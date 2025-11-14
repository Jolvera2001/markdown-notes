mod app_error;
mod document;

use tauri::State;

use crate::{
    app_error::AppError,
    document::{Document, DocumentState},
};

#[tauri::command]
async fn load_files(
    directory_path: &str,
    documents: State<'_, DocumentState>,
) -> Result<Vec<Document>, AppError> {
    documents.fetch_test_files(directory_path).await?;
    Ok(documents.get_documents())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(DocumentState::new())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![load_files])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
