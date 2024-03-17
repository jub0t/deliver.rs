// Import the FileFormat enum from the super module
use super::FileFormat;

// Convert a FileFormat enum variant to its corresponding string representation
pub fn format_to_string(f: FileFormat) -> String {
    match f {
        FileFormat::CSS => "css".to_string(),
        FileFormat::HTML => "html".to_string(),
        FileFormat::JS => "js".to_string(),
        FileFormat::None => String::new(),
    }
}

// Convert a string representation to the corresponding FileFormat enum variant
pub fn string_to_format(s: &str) -> Option<FileFormat> {
    match s {
        "css" => Some(FileFormat::CSS),
        "html" => Some(FileFormat::HTML),
        "js" => Some(FileFormat::JS),
        _ => None, // Return None for unrecognized formats
    }
}
