pub struct Image(json::Image);
/// Image data belonging to an imported glTF asset.
#[derive(Clone, Debug)]
pub struct Data {
    /// The image pixel data (8 bits per channel).
    pub pixels: Vec<u8>,

    /// The image pixel data format.
    pub format: Format,

    /// The image width in pixels.
    pub width: u32,

    /// The image height in pixels.
    pub height: u32,
}
