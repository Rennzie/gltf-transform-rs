use super::gltf_reader::GltfReader;
use super::{Error, Result};
use crate::gltf::GlTF;
use crate::properties::buffer::{Blob, Buffer};
use std::path::Path;
use std::{fs, io};

pub fn import<P>(path: P) -> Result<GlTF>
where
    P: AsRef<Path>,
{
    let base = path.as_ref().parent().unwrap_or_else(|| Path::new("./"));
    let file = fs::File::open(path).map_err(Error::Io)?;
    let reader = io::BufReader::new(file);

    let reader = GltfReader::from_reader(reader).unwrap();
    let buffers = import_buffer_data(&reader.root_json, Some(base), reader.blob).unwrap();
    // TODO: Import images

    Ok(GlTF::new(reader.root_json, buffers))
}

/// Breaks the buffer_json into Buffer for importing
/// Reads each buffer from the blob or resource URI
/// Returns a list of buffers
pub fn import_buffer_data(
    root_json: &json::Root,
    base: Option<&Path>,
    mut blob: Option<Vec<u8>>,
) -> Result<Vec<Blob>> {
    let mut buffer_blobs = Vec::with_capacity(root_json.buffers.len());

    for (i, json_buffer) in root_json.buffers.iter().enumerate() {
        let buffer = Buffer::from_json(json_buffer, blob, i, base);

        match buffer {
            Ok(buffer) => buffer_blobs.push(buffer.into_blob()),
            Err(err) => return Err(err),
        }
    }

    Ok(buffer_blobs)
}

#[cfg(feature = "image")]
/// Import the image data referenced by a glTF document.
pub fn import_image_data(
    document: &Document,
    base: Option<&Path>,
    buffer_data: &[Blob],
) -> Result<Vec<image::Data>> {
    let mut images = Vec::new();
    // #[cfg(feature = "guess_mime_type")]
    // let guess_format = |encoded_image: &[u8]| match image_crate::guess_format(encoded_image) {
    //     Ok(image_crate::ImageFormat::Png) => Some(Png),
    //     Ok(image_crate::ImageFormat::Jpeg) => Some(Jpeg),
    //     _ => None,
    // };
    // #[cfg(not(feature = "guess_mime_type"))]
    let guess_format = |_encoded_image: &[u8]| None;
    for image in document.images() {
        let decoded_image = match image.source() {
            image::Source::Uri { uri, mime_type } if base.is_some() => {
                match UriReader::parse(uri) {
                    UriReader::Data(Some(annoying_case), base64) => {
                        #[allow(deprecated)]
                        let encoded_image = base64::decode(base64).map_err(Error::Base64)?;
                        let encoded_format = match annoying_case {
                            "image/png" => Png,
                            "image/jpeg" => Jpeg,
                            _ => match guess_format(&encoded_image) {
                                Some(format) => format,
                                None => return Err(Error::UnsupportedImageEncoding),
                            },
                        };

                        image_crate::load_from_memory_with_format(&encoded_image, encoded_format)?
                    }
                    UriReader::Unsupported => return Err(Error::UnsupportedScheme),
                    _ => {
                        let encoded_image = UriReader::read(base, uri)?;
                        let encoded_format = match mime_type {
                            Some("image/png") => Png,
                            Some("image/jpeg") => Jpeg,
                            Some(_) => match guess_format(&encoded_image) {
                                Some(format) => format,
                                None => return Err(Error::UnsupportedImageEncoding),
                            },
                            None => match uri.rsplit('.').next() {
                                Some("png") => Png,
                                Some("jpg") | Some("jpeg") => Jpeg,
                                _ => match guess_format(&encoded_image) {
                                    Some(format) => format,
                                    None => return Err(Error::UnsupportedImageEncoding),
                                },
                            },
                        };
                        image_crate::load_from_memory_with_format(&encoded_image, encoded_format)?
                    }
                }
            }
            image::Source::View { view, mime_type } => {
                let parent_buffer_data = &buffer_data[view.buffer().index()].0;
                let begin = view.offset();
                let end = begin + view.length();
                let encoded_image = &parent_buffer_data[begin..end];
                let encoded_format = match mime_type {
                    "image/png" => Png,
                    "image/jpeg" => Jpeg,
                    _ => match guess_format(encoded_image) {
                        Some(format) => format,
                        None => return Err(Error::UnsupportedImageEncoding),
                    },
                };
                image_crate::load_from_memory_with_format(encoded_image, encoded_format)?
            }
            _ => return Err(Error::ExternalReferenceInSliceImport),
        };

        images.push(image::Data::new(decoded_image)?);
    }

    Ok(images)
}

// fn import_impl(GltfReader { root_json, blob }: GltfReader, base: Option<&Path>) -> Result<Import> {
//     let buffer_data = import_buffer_data(&root_json, base, blob)?;
//     #[cfg(feature = "image")]
//     let image_data = import_image_data(&root_json, base, &buffer_data)?;
//     let import = (root_json, buffer_data);
//     Ok(import)
// }

// pub fn import_slice_impl(slice: &[u8]) -> Result<Import> {
//     import_impl(GltfReader::from_slice(slice)?, None)
// }

// / Import some glTF 2.0 from a slice
// /
// / ```
// / # extern crate gltf;
// / # use std::fs;
// / # use std::io::Read;
// / # fn run() -> Result<(), gltf::Error> {
// / # let path = "examples/Box.glb";
// / # let mut file = fs::File::open(path).map_err(gltf::Error::Io)?;
// / # let mut bytes = Vec::new();
// / # file.read_to_end(&mut bytes).map_err(gltf::Error::Io)?;
// / # #[allow(unused)]
// / let (document, buffers, images) = gltf::import_slice(bytes.as_slice())?;
// / # Ok(())
// / # }
// / # fn main() {
// / #     run().expect("test failure");
// / # }
// / ```
// pub fn import_slice<S>(slice: S) -> Result<Import>
// where
//     S: AsRef<[u8]>,
// {
//     import_slice_impl(slice.as_ref())
// }
