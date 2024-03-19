use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum ImageFormat {
    PNG,
    JPEG,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum DataFormat {
    JSON,
    XML,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum FileFormat {
    CSS,
    JS,
    HTML,
    IMAGE(ImageFormat),
    DATA(DataFormat),
    None,
}
