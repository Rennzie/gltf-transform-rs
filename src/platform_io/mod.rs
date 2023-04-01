#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
/// The type of output to write.
pub enum Variant {
    /// Output standard glTF.
    /// Can only be used if the imported glTF has buffer uri's pre-set.
    GlTF,
    /// Output binary glTF.
    Glb,
}

impl Variant {
    pub fn from_ext(ext: &str) -> Self {
        match ext {
            "gltf" => Variant::GlTF,
            "glb" => Variant::Glb,
            _ => panic!("Unknown extension"),
        }
    }

    /// Returns the file extension for the variant.
    pub fn ext(self) -> &'static str {
        match self {
            Variant::GlTF => "gltf",
            Variant::Glb => "glb",
        }
    }
}

// ---- Export ----------------------------------------------------------------
mod export;
use std::result;

pub use export::export;
pub use export::export_to_glb;
pub use export::export_to_gltf;

// ---- Import ----------------------------------------------------------------
mod glb_reader;
mod gltf_reader;
mod import;
pub use import::import;

// ---- Result & Errors ----------------------------------------------------------

/// Result type for convenience.
pub type Result<T> = result::Result<T, Error>;

/// Represents a runtime error.
#[derive(Debug)]
pub enum Error {
    /// Base 64 decoding error.
    #[cfg(feature = "import")]
    #[cfg_attr(docsrs, doc(cfg(feature = "import")))]
    Base64(base64::DecodeError),

    /// GLB parsing error.
    Binary(glb_reader::BinError),

    InvalidExtension,

    /// Buffer length does not match expected length.
    #[cfg(feature = "import")]
    #[cfg_attr(docsrs, doc(cfg(feature = "import")))]
    BufferLength {
        /// The index of the offending buffer.
        buffer: usize,

        /// The expected buffer length in bytes.
        expected: usize,

        /// The number of bytes actually available.
        actual: usize,
    },

    /// JSON deserialization error.
    Deserialize(json::Error),

    /// Standard I/O error.
    Io(std::io::Error),

    /// Image decoding error.
    #[cfg(feature = "import")]
    #[cfg_attr(docsrs, doc(cfg(feature = "import")))]
    Image(image_crate::ImageError),

    /// The `BIN` chunk of binary glTF is referenced but does not exist.
    #[cfg(feature = "import")]
    #[cfg_attr(docsrs, doc(cfg(feature = "import")))]
    MissingBlob,

    /// An external file is referenced in a slice only import without path
    #[cfg(feature = "import")]
    #[cfg_attr(docsrs, doc(cfg(feature = "import")))]
    ExternalReferenceInSliceImport,

    /// Unsupported image encoding.
    #[cfg(feature = "import")]
    #[cfg_attr(docsrs, doc(cfg(feature = "import")))]
    UnsupportedImageEncoding,

    /// Unsupported image format.
    #[cfg(feature = "import")]
    #[cfg_attr(docsrs, doc(cfg(feature = "import")))]
    UnsupportedImageFormat(image_crate::DynamicImage),

    /// Unsupported URI scheme.
    #[cfg(feature = "import")]
    #[cfg_attr(docsrs, doc(cfg(feature = "import")))]
    UnsupportedScheme,

    /// glTF validation error.
    Validation(Vec<(json::Path, json::validation::Error)>),
}
