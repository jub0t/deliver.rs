use serde::Serialize;

use crate::cache::File;

#[derive(Serialize)]
pub struct MessageResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Serialize)]
pub struct IndexResponse {
    pub success: bool,
}

#[derive(Serialize)]
pub struct DiagnosticsResponse {
    pub bytes_cached: usize,
    pub total_files: usize,
}

#[derive(Serialize)]
pub struct ListAllResponse {
    pub(crate) files: Vec<File>,
}
