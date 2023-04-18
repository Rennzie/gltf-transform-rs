use super::extension::Extension;
use crate::gltf_io::UriReader;
use crate::gltf_io::{Error, Result};
pub use json::buffer::Target;
use std::{ops, path::Path};

/// Describes a buffer data source.
#[derive(Clone, Copy, Debug)]
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

/// A buffer points to binary data representing geometry, animations, or skins.
#[derive(Clone, Debug)]
pub struct Buffer<'a> {
    pub name: Option<String>,
    pub source: Source<'a>,
    pub extras: json::Extras,
    pub blob: Blob,
    #[cfg(feature = "extensions")]
    pub extensions: Option<Extension<json::extensions::buffer::Buffer>>,
}

impl Default for Buffer<'_> {
    fn default() -> Self {
        Self {
            name: None,
            source: Source::Bin,
            extras: Default::default(),
            blob: Blob(Vec::new()),
            #[cfg(feature = "extensions")]
            extensions: None,
        }
    }
}

impl<'a> Buffer<'a> {
    pub fn from_json(
        json: &'a json::Buffer, //NOTE: Are we keeping the json around?
        blob: &mut Option<Vec<u8>>,
        index: usize,
        base: Option<&Path>,
    ) -> Result<Self> {
        let byte_length = json.byte_length;
        let source = match json.uri.as_deref() {
            Some(uri) => Source::Uri(uri),
            None => Source::Bin,
        };

        let mut data = match source {
            Source::Uri(uri) => UriReader::read(base, uri),
            Source::Bin => blob.take().ok_or(Error::MissingBlob), // not sure about this. Are we taking the correct amount of blob?
        }?;

        if data.len() < byte_length as usize {
            return Err(Error::BufferLength {
                buffer: index,
                expected: byte_length as usize,
                actual: data.len(),
            });
        }
        while data.len() % 4 != 0 {
            data.push(0);
        }

        Ok(Self {
            name: json.name.clone(),
            source,
            blob: Blob(data),
            extras: json.extras.clone(),
            #[cfg(feature = "extensions")]
            extensions: json
                .extensions
                .as_ref()
                .map(|ext| Extension::from_json(ext.clone(), None, None)),
        })
    }

    /// Returns the blob while consuming the buffer.
    pub fn into_blob(self) -> Blob {
        self.blob
    }

    /// Returns the buffer data source.
    pub fn source(&self) -> &Source<'a> {
        &self.source
    }

    /// Returns a borrowed mutable reference to the buffer data source.
    pub fn source_mut(&mut self) -> &mut Source<'a> {
        &mut self.source
    }

    /// The length of the buffer in bytes.
    pub fn length(&self) -> usize {
        self.blob.0.len()
    }

    /// Optional user-defined name for this object.
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        &self.extras
    }
}
