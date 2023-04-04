#[derive(Debug, Clone)]
pub struct Image(json::Image);
impl Image {
    pub fn new(json: &json::Image) -> Self {
        Image(json.clone())
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Format {
    /// Red only.
    R8,

    /// Red, green.
    R8G8,

    /// Red, green, blue.
    R8G8B8,

    /// Red, green, blue, alpha.
    R8G8B8A8,

    /// Red only (16 bits).
    R16,

    /// Red, green (16 bits).
    R16G16,

    /// Red, green, blue (16 bits).
    R16G16B16,

    /// Red, green, blue, alpha (16 bits).
    R16G16B16A16,

    /// Red, green, blue (32 bits float)
    R32G32B32FLOAT,

    /// Red, green, blue, alpha (32 bits float)
    R32G32B32A32FLOAT,
}

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
