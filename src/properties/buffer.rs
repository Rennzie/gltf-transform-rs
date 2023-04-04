use crate::document::Document;
pub use json::buffer::Target;
use std::ops;

/// A buffer points to binary data representing geometry, animations, or skins.
#[derive(Clone, Debug)]
pub struct Buffer<'a> {
    /// The parent `Document` struct.
    #[allow(dead_code)]
    document: &'a Document,

    /// The corresponding JSON index.
    index: usize,

    /// The corresponding JSON struct.
    json: &'a json::buffer::Buffer,
}

/// A view into a buffer generally representing a subset of the buffer.
#[derive(Clone, Debug)]
pub struct View<'a> {
    /// The parent `Document` struct.
    root_json: &'a json::Root,

    /// The corresponding JSON index.
    index: usize,

    /// The corresponding JSON struct.
    json: &'a json::buffer::View,

    /// The parent `Buffer`.
    #[allow(dead_code)]
    parent: json::Buffer,
}

/// Describes a buffer data source.
#[derive(Clone, Debug)]
pub enum Source<'a> {
    /// Buffer data is contained in the `BIN` section of binary glTF.
    Bin,

    /// Buffer data is contained in an external data source.
    Uri(&'a str),
}

/// Buffer data belonging to an imported glTF asset.
#[derive(Clone, Debug)]
pub struct Blob(pub Vec<u8>);

impl ops::Deref for Blob {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

impl<'a> Buffer<'a> {
    /// Constructs a `Buffer`.
    pub(crate) fn new(
        document: &'a Document,
        index: usize,
        json: &'a json::buffer::Buffer,
    ) -> Self {
        Self {
            document,
            index,
            json,
        }
    }

    /// Returns the internal JSON index.
    pub fn index(&self) -> usize {
        self.index
    }

    /// Returns the buffer data source.
    pub fn source(&self) -> Source<'a> {
        if let Some(uri) = self.json.uri.as_deref() {
            Source::Uri(uri)
        } else {
            Source::Bin
        }
    }

    /// The length of the buffer in bytes.
    pub fn length(&self) -> usize {
        self.json.byte_length as usize
    }

    /// Optional user-defined name for this object.
    pub fn name(&self) -> Option<&'a str> {
        self.json.name.as_deref()
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &'a json::Extras {
        &self.json.extras
    }
}

impl<'a> View<'a> {
    /// Constructs a `View`.
    pub(crate) fn new(root: &'a json::Root, index: usize, json: &'a json::buffer::View) -> Self {
        let parent = root.buffers[json.buffer.value()];
        Self {
            root_json: root,
            index,
            json,
            parent,
        }
    }

    /// Returns the internal JSON index.
    pub fn index(&self) -> usize {
        self.index
    }

    /// Returns the parent `Buffer`.
    pub fn buffer(&self) -> Buffer<'a> {
        self.root_json.buffers[self.json.buffer.value()]
    }

    /// Returns the length of the buffer view in bytes.
    pub fn length(&self) -> usize {
        self.json.byte_length as usize
    }

    /// Returns the offset into the parent buffer in bytes.
    pub fn offset(&self) -> usize {
        self.json.byte_offset.unwrap_or(0) as usize
    }

    /// Returns the stride in bytes between vertex attributes or other interleavable
    /// data. When `None`, data is assumed to be tightly packed.
    pub fn stride(&self) -> Option<usize> {
        self.json.byte_stride.and_then(|x| {
            // Treat byte_stride == 0 same as not specifying stride.
            // This is technically a validation error, but best way we can handle it here
            if x == 0 {
                None
            } else {
                Some(x as usize)
            }
        })
    }

    /// Optional user-defined name for this object.
    pub fn name(&self) -> Option<&'a str> {
        self.json.name.as_deref()
    }

    /// Optional target the buffer should be bound to.
    pub fn target(&self) -> Option<Target> {
        self.json.target.map(|target| target.unwrap())
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &'a json::Extras {
        &self.json.extras
    }
}
