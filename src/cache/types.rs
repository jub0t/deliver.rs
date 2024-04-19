use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Copy)]
pub enum ImageFormat {
    PNG,
    JPEG,
    JPG,
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
    AVI,
    WEBM,
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
pub enum FontFormats {
    OTF,
    TTF,
    WOFF,
    WOFF2,
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
    FONT(FontFormats),
    None,
}
