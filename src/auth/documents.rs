use serde::{Deserialize, Serialize};

use crate::cache::FileKey;

#[derive(Serialize, Deserialize)]
pub struct AllowedDocuments {
    pub data: Vec<FileKey>,
}

impl AllowedDocuments {
    pub fn new() -> Self {
        return Self { data: Vec::new() };
    }

    pub fn is_allowed(&self, doc_id: FileKey) -> bool {
        for key in &self.data {
            if key == &doc_id {
                return true;
            }
        }

        return false;
    }

    pub fn allow(&mut self, doc_id: FileKey) {
        self.data.push(doc_id);
    }
}
