use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    sync::Mutex,
};
use tokio::fs::{read_dir, read_to_string, write};

// TODO: Add Result<> returns for proper error handling

pub struct DocumentState {
    pub documents: Mutex<Vec<Document>>,
}

impl DocumentState {
    pub fn new() -> Self {
        Self {
            documents: Mutex::new(Vec::new()),
        }
    }

    pub async fn fetch_test_files(&self) {
        let mut files = Vec::new();

        let mut entries = read_dir(PathBuf::from(
            r"C:\LocalProjects\rust\markdown-notes\src-tauri\test_files",
        ))
        .await
        .unwrap();

        while let Some(entry) = entries.next_entry().await.unwrap() {
            let path = entry.path();

            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "md" {
                        let doc_name = entry.file_name();

                        let doc = Document::new(path, doc_name.into_string().unwrap()).await;
                        files.push(doc);
                    }
                }
            }
        }

        self.documents.lock().unwrap().extend(files);
    }

    pub fn get_documents(&self) -> Vec<Document> {
        self.documents.lock().unwrap().clone()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Document {
    pub path: PathBuf,
    pub name: String,
    pub content: String,
    pub is_dirty: bool,
}

impl Document {
    pub async fn new(path: PathBuf, name: String) -> Self {
        let content = read_to_string(&path).await.unwrap();

        Self {
            path,
            name: name.to_string(),
            content,
            is_dirty: false,
        }
    }

    pub async fn save(&mut self, new_content: &str) -> std::io::Result<()> {
        let result = write(&self.path, new_content).await;
        self.content = new_content.to_string();
        self.is_dirty = false;

        result
    }

    pub fn dirty(&mut self) {
        self.is_dirty = true;
    }
}
