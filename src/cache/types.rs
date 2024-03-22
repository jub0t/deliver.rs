use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Copy)]
pub enum ImageFormat {
    PNG,
    JPEG,
}

#[derive(Clone, Serialize, Deserialize, Copy)]
pub enum AudioFormats {
    MP3,
    ACC,
    OGG,
}

#[derive(Clone, Serialize, Deserialize, Copy)]
pub enum DocumentFormats {
    PDF,
    PPT,
}

#[derive(Clone, Serialize, Deserialize, Copy)]
pub enum VideoFormats {
    MP4,
}

#[derive(Clone, Serialize, Deserialize, Copy)]
pub enum ArchiveFormats {
    RAR,
    ZIP,
}

#[derive(Clone, Serialize, Deserialize, Copy)]
pub enum FeedFormats {
    JSON,
    XML,
    YAML,
}

#[derive(Clone, Serialize, Deserialize, Copy)]
pub enum FileFormat {
    CSS,
    JS,
    HTML,
    IMAGE(ImageFormat),
    DOCUMENT(DocumentFormats),
    AUDIO(AudioFormats),
    ARCHIVE(ArchiveFormats),
    FEED(FeedFormats),
    VIDEO(VideoFormats),
    None,
}
