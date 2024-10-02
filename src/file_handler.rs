use std::path::PathBuf;
use std::sync::Arc;
use crate::editor::EditorError;

const ROOT_DIR: &'static str = env!("CARGO_MANIFEST_DIR");

pub async fn pick_file() -> Result<(PathBuf, Arc<String>), EditorError> {
    let handler = rfd::AsyncFileDialog::new()
        .set_title("Choose a text file ...")
        .set_directory(format!("{}",ROOT_DIR))
        .pick_file()
        .await
        .ok_or(EditorError::DialogClose)?;
    load_file(handler.path().to_owned())
        .await
}

pub async fn save_file(file_text: String) -> Result<PathBuf, EditorError> {
    let handle = rfd::AsyncFileDialog::new()
        .set_title("Save file ...")
        .set_directory(format!("{}",ROOT_DIR))
        .save_file()
        .await
        .ok_or(EditorError::DialogClose)?;
    tokio::fs::write(handle.path(), file_text)
        .await
        .map_err(|error| EditorError::IOFailed(error.kind()))?;
    Ok(PathBuf::from(handle.path()))
}

pub async fn load_file(path: PathBuf) -> Result<(PathBuf,Arc<String>), EditorError> {
    let contents = tokio::fs::read_to_string(&path)
        .await
        .map(Arc::new)
        .map_err(|e| EditorError::IO(e.kind()))?;
    Ok((path, contents))
}

pub fn default_file() -> PathBuf {
    PathBuf::from(format!("{}\\README.md", ROOT_DIR))
}