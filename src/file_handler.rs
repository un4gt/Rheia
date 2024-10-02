use std::path::{Path, PathBuf};
use std::sync::Arc;
use crate::editor::EditorError;

const ROOT_DIR: &'static str = env!("CARGO_MANIFEST_DIR");

pub async fn open_file() -> Result<(PathBuf, Arc<String>), EditorError> {
    let handler = rfd::AsyncFileDialog::new()
        .set_title("Choose a text file ...")
        .set_directory(format!("{}",ROOT_DIR))
        .pick_file()
        .await
        .ok_or(EditorError::DialogClose)?;
    load_file(handler)
        .await
}

pub async fn save_file(
    path: Option<PathBuf>,
    contents: String
) -> Result<PathBuf, EditorError> {
    let path = if let Some(path) = path {
        path
    } else {
        rfd::AsyncFileDialog::new()
            .set_title("Save file")
            .set_directory(format!("{}", ROOT_DIR))
            .save_file()
            .await
            .as_ref()
            .map(rfd::FileHandle::path)
            .map(Path::to_owned)
            .ok_or(EditorError::DialogClose)?
    };
    rfd::AsyncFileDialog::new()
        .set_title("Save file ...")
        .set_directory(format!("{}",ROOT_DIR))
        .save_file()
        .await
        .ok_or(EditorError::DialogClose)?;
    tokio::fs::write(&path, contents)
        .await
        .map_err(|error| EditorError::IoError(error.kind()))?;
    Ok(path)
}

pub async fn load_file(path: impl Into<PathBuf>) -> Result<(PathBuf,Arc<String>), EditorError> {
    let path = path.into();
    let contents = tokio::fs::read_to_string(&path)
        .await
        .map(Arc::new)
        .map_err(|e| EditorError::IoError(e.kind()))?;
    Ok((path, contents))
}

pub fn default_file() -> PathBuf {
    PathBuf::from(format!("{}\\README.md", ROOT_DIR))
}