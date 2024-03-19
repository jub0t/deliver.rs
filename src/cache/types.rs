use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum ImageFormat {
    PNG,
    JPEG,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum AudioFormats {
    MP3,
    ACC,
    OGG,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum DocumentFormats {
    PDF,
    PPT,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum ArchiveFormats {
    RAR,
    ZIP,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum FeedFormats {
    JSON,
    XML,
    YAML,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum FileFormat {
    CSS,
    JS,
    HTML,
    IMAGE(ImageFormat),
    DOCUMENT(DocumentFormats),
    AUDIO(AudioFormats),
    ARCHIVE(ArchiveFormats),
    FEED(FeedFormats),
    None,
}
