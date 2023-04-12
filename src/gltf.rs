use crate::properties::buffer::Blob;
// use crate::properties::image::Data as ImageData;
use crate::properties::Document;

/// A parsed glTF file.
/// More useful when converted into a `Document`.
pub struct GlTF {
    pub root: json::Root,
    pub buffers: Vec<Blob>,
    // images: Vec<ImageData>,
}

impl GlTF {
    pub fn new(root: json::Root, buffers: Vec<Blob>) -> Self {
        Self {
            root,
            buffers,
            // images,
        }
    }

    // pub fn read()

    pub fn into_document(self) -> Document<'static> {
        Document::from_gltf(self)
    }
}
