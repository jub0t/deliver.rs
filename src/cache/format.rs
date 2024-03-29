// Import the FileFormat enum from the super module
use super::{
    types::{
        ArchiveFormats, AudioFormats, DocumentFormats, FeedFormats, ImageFormat, VideoFormats,
    },
    FileFormat,
};

// Convert a FileFormat enum variant to its corresponding string representation
pub fn format_to_string(f: FileFormat) -> String {
    match f {
        FileFormat::CSS => "css".to_string(),
        FileFormat::HTML => "html".to_string(),
        FileFormat::JS => "js".to_string(),
        FileFormat::IMAGE(i) => match i {
            ImageFormat::JPEG => "jpeg".to_string(),
            ImageFormat::PNG => "png".to_string(),
            ImageFormat::JPG => "jpg".to_string(),
        },

        _ => String::new(),
    }
}

// Convert a string representation to the corresponding FileFormat enum variant
pub fn string_to_format(s: &str) -> Option<FileFormat> {
    match s {
        "css" => Some(FileFormat::CSS),
        "html" => Some(FileFormat::HTML),
        "js" => Some(FileFormat::JS),
        "png" => Some(FileFormat::IMAGE(ImageFormat::PNG)),
        "jpeg" => Some(FileFormat::IMAGE(ImageFormat::JPEG)),
        "mp4" => Some(FileFormat::VIDEO(VideoFormats::MP4)),
        "avi" => Some(FileFormat::VIDEO(VideoFormats::AVI)),
        "json" => Some(FileFormat::FEED(FeedFormats::JSON)),
        "xml" => Some(FileFormat::FEED(FeedFormats::XML)),
        "yaml" => Some(FileFormat::FEED(FeedFormats::YAML)),
        _ => None, // Return None for unrecognized formats
    }
}

pub fn format_to_mime(ft: FileFormat) -> String {
    match ft {
        FileFormat::IMAGE(image_format) => match image_format {
            ImageFormat::PNG => "image/png".to_string(),
            ImageFormat::JPEG => "image/jpeg".to_string(),
            ImageFormat::JPG => "image/jpg".to_string(),
        },
        FileFormat::AUDIO(audio_format) => match audio_format {
            AudioFormats::MP3 => "audio/mpeg".to_string(),
            AudioFormats::ACC => "audio/aac".to_string(),
            AudioFormats::OGG => "audio/ogg".to_string(),
        },
        FileFormat::DOCUMENT(document_format) => match document_format {
            DocumentFormats::PDF => "application/pdf".to_string(),
            DocumentFormats::PPT => "application/vnd.ms-powerpoint".to_string(),
        },
        FileFormat::ARCHIVE(archive_format) => match archive_format {
            ArchiveFormats::RAR => "application/x-rar-compressed".to_string(),
            ArchiveFormats::ZIP => "application/zip".to_string(),
        },
        FileFormat::FEED(feed_format) => match feed_format {
            FeedFormats::JSON => "application/json".to_string(),
            FeedFormats::XML => "application/xml".to_string(),
            FeedFormats::YAML => "application/x-yaml".to_string(),
        },
        FileFormat::HTML => "text/html".to_string(),
        FileFormat::JS => "text/javascript".to_string(),
        FileFormat::CSS => "text/css".to_string(),
        FileFormat::None => "application/text".to_string(),
        FileFormat::VIDEO(vid_fmt) => match vid_fmt {
            VideoFormats::MP4 => "video/mp4".to_string(),
            VideoFormats::AVI => "video/x-msvideo".to_string(),
            VideoFormats::WEBM => "video/webm".to_string(),
        },
    }
}
