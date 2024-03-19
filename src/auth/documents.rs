use serde::{Deserialize, Serialize};

use crate::cache::FileKey;

#[derive(Serialize, Deserialize)]
pub struct AllowedDocuments {
    pub data: Vec<FileKey>,
}

impl Default for AllowedDocuments {
    fn default() -> Self {
        Self::new()
    }
}

impl AllowedDocuments {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn is_allowed(&self, doc_id: FileKey) -> bool {
        for key in &self.data {
            if key == &doc_id {
                return true;
            }
        }

        false
    }

    pub fn allow(&mut self, doc_id: FileKey) {
        self.data.push(doc_id);
    }
}
