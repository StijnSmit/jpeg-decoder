use std::fs;
pub mod jpeg;

use jpeg::JPEG;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

pub fn create_jpeg() -> JPEG {
    let img_data: Vec<u8> = fs::read("tutorial-image.jpg").unwrap();
    JPEG::new(img_data)
}
