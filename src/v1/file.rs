use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct FileData {
    pub id: String,
    pub oejct: String,
    pub bytes: i32,
    pub created_at: i64,
    pub filename: String,
    pub purpose: String,
}

#[derive(Debug, Deserialize)]
pub struct FileListResponse {
    pub object: String,
    pub data: Vec<FileData>,
}

#[derive(Debug, Serialize)]
pub struct FileUploadRequest {
    pub file: String,
    pub purpose: String,
}

impl FileUploadRequest {
    pub fn new(file: String, purpose: String) -> Self {
        Self { file, purpose }
    }
}

#[derive(Debug, Deserialize)]
pub struct FileUploadResponse {
    pub id: String,
    pub oejct: String,
    pub bytes: i32,
    pub created_at: i64,
    pub filename: String,
    pub purpose: String,
}

#[derive(Debug, Serialize)]
pub struct FileDeleteRequest {
    pub file_id: String,
}

impl FileDeleteRequest {
    pub fn new(file_id: String) -> Self {
        Self { file_id }
    }
}

#[derive(Debug, Deserialize)]
pub struct FileDeleteResponse {
    pub id: String,
    pub oejct: String,
    pub delete: bool,
}

#[derive(Debug, Serialize)]
pub struct FileRetrieveRequest {
    pub file_id: String,
}

impl FileRetrieveRequest {
    pub fn new(file_id: String) -> Self {
        Self { file_id }
    }
}

#[derive(Debug, Deserialize)]
pub struct FileRetrieveResponse {
    pub id: String,
    pub oejct: String,
    pub bytes: i32,
    pub created_at: i64,
    pub filename: String,
    pub purpose: String,
}

#[derive(Debug, Serialize)]
pub struct FileRetrieveContentRequest {
    pub file_id: String,
}

impl FileRetrieveContentRequest {
    pub fn new(file_id: String) -> Self {
        Self { file_id }
    }
}

#[derive(Debug, Deserialize)]
pub struct FileRetrieveContentResponse {
    pub id: String,
    pub oejct: String,
    pub bytes: i32,
    pub created_at: i64,
    pub filename: String,
    pub purpose: String,
}
