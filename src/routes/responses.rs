use serde::Serialize;

#[derive(Serialize)]
pub struct IndexResponse {
    pub success: bool,
}

#[derive(Serialize)]
pub struct DiagnosticsResponse {
    pub bytes_cached: usize,
    pub total_files: usize,
}
