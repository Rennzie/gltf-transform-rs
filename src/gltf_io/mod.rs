pub use crate::prelude::*;

// ---- Export ----------------------------------------------------------------
mod export;
pub use export::export;
pub use export::export_to_glb;
pub use export::export_to_gltf;

// ---- Import ----------------------------------------------------------------
mod glb_reader;
mod gltf_reader;
mod import;
mod uri_reader;
pub use import::import;
pub use uri_reader::UriReader;

use self::glb_reader::BinError;

// ---- Variant ---------------------------------------------------------------

#[derive(Clone, Copy, Debug, Hash)]
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

// ---- Result & Errors ----------------------------------------------------------

/// Result type for convenience.
pub type Result<T> = std::result::Result<T, Error>;

/// Represents a runtime error.
#[derive(Debug)]
pub enum Error {
    /// Base 64 decoding error.
    Base64(base64::DecodeError),

    /// GLB parsing error.
    Binary(glb_reader::BinError),

    InvalidExtension,

    /// Buffer length does not match expected length.
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
    Image(image_crate::ImageError),

    /// The `BIN` chunk of binary glTF is referenced but does not exist.
    MissingBlob,

    /// An external file is referenced in a slice only import without path
    ExternalReferenceInSliceImport,

    /// Unsupported image encoding.
    UnsupportedImageEncoding,

    /// Unsupported image format.
    UnsupportedImageFormat(image_crate::DynamicImage),

    /// Unsupported URI scheme.
    UnsupportedScheme,

    /// glTF validation error.
    Validation(Vec<(json::Path, json::validation::Error)>),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Base64(ref e) => e.fmt(f),
            Error::Binary(ref e) => e.fmt(f),
            Error::BufferLength {
                buffer,
                expected,
                actual,
            } => {
                write!(
                    f,
                    "buffer {}: expected {} bytes but received {} bytes",
                    buffer, expected, actual
                )
            }
            Error::Deserialize(ref e) => e.fmt(f),
            Error::Io(ref e) => e.fmt(f),
            Error::Image(ref e) => e.fmt(f),
            Error::MissingBlob => write!(f, "missing binary portion of binary glTF"),
            Error::ExternalReferenceInSliceImport => {
                write!(f, "external reference in slice only import")
            }
            Error::UnsupportedImageEncoding => write!(f, "unsupported image encoding"),
            Error::UnsupportedImageFormat(image) => {
                write!(f, "unsupported image format: {:?}", image.color())
            }
            Error::UnsupportedScheme => write!(f, "unsupported URI scheme"),
            Error::Validation(ref xs) => {
                write!(f, "invalid glTF:")?;
                for &(ref path, ref error) in xs {
                    write!(f, " {}: {};", path, error)?;
                }
                Ok(())
            }
            Error::InvalidExtension => write!(f, "invalid extension"),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<BinError> for Error {
    fn from(err: BinError) -> Self {
        Error::Binary(err)
    }
}

impl From<json::Error> for Error {
    fn from(err: json::Error) -> Self {
        Error::Deserialize(err)
    }
}

impl From<Vec<(json::Path, json::validation::Error)>> for Error {
    fn from(errs: Vec<(json::Path, json::validation::Error)>) -> Self {
        Error::Validation(errs)
    }
}
