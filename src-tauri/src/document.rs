use std::{fs::{read_dir, read_to_string}, path::{Path, PathBuf}, sync::Mutex};

pub struct DocumentState {
    pub documents: Mutex<Option<Vec<Document>>>,
}

impl DocumentState {
    pub fn new() -> Self {
        Self {
            documents: Mutex::new(None)
        }
    }

    pub fn fetch_test_files(&mut self) {
        let mut files = Vec::new();

        let entries = read_dir(PathBuf::from(r"C:\LocalProjects\rust\markdown-notes\src-tauri\test_files")).unwrap();
        
        for entry in entries {
            let entry = entry.unwrap();
            let path = entry.path();

            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "md" {
                        let doc_name = entry.file_name();

                        let doc = Document::new(path, doc_name.into_string().unwrap());
                        files.push(doc);
                    }
                }
            }
        }

        self.documents = Mutex::new(Some(files));
    }
}

pub struct Document {
    pub path: PathBuf,
    pub name: String,
    pub content: String,
    pub is_dirty: bool,
}

impl Document {
    pub fn new(path: PathBuf, name: String) -> Self {
        let content = read_to_string(&path).unwrap();

        Self {
            path,
            name: name.to_string(),
            content,
            is_dirty: false,
        }
    }

    pub fn save(&mut self, new_content: &str) -> std::io::Result<()> {
        let result = std::fs::write(&self.path, new_content);
        self.content = new_content.to_string();
        self.is_dirty = false;

        result
    }

    pub fn dirty(&mut self) {
        self.is_dirty = true;
    }
}
