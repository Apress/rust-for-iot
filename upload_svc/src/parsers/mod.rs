
pub mod image;
pub mod video;

use image::ImageMetaData;
use video::VideoMetaData;

#[derive(Debug)]
pub enum FileMetaData {
    Image(ImageMetaData),
    Video(VideoMetaData),
    None
}
