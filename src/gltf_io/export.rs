// use crate::properties::buffer::{Blob, Source};
// use crate::utils::{align_to_multiple_of_four, to_padded_byte_vector};

// use super::glb_reader::{GlbReader, Header};
// use super::{Document, Error, Variant};
// use std::borrow::Cow;
// use std::io::{ErrorKind, Write};
// use std::{fs, path};

// /// Export a glTF document
// /// TODO: support images
// pub fn export(
//     output: Variant,
//     path: &str,
//     document: Document,
//     buffers: Vec<Blob>,
//     // _image_buffers: Vec<image::Data>,
// ) -> Result<(), Error> {
//     match output {
//         Variant::GlTF => export_to_gltf(path, document, buffers),
//         Variant::Glb => export_to_glb(path, document, buffers),
//     }
// }

// /// Export a glTF document
// pub fn export_to_gltf(
//     path: &str,
//     document: Document,
//     buffers: Vec<Blob>,
//     // _image_buffers: Vec<image::Data>,
// ) -> Result<(), Error> {
//     // write each buffer to a file

//     let gltf_path = path::Path::new(&path).with_extension("gltf");
//     let directory = gltf_path.parent();
//     if let Some(dir) = directory {
//         fs::create_dir_all(dir).expect("I/O error");
//     }

//     // write binary files
//     for buffer in document.buffers() {
//         // get the source of the buffer; if it's not a file, then we can't write it;
//         match buffer.source() {
//             Source::Uri(uri) => {
//                 let uri_path = path::Path::new(uri);
//                 let mut writer = if let Some(dir) = directory {
//                     fs::File::create(dir.join(uri_path)).expect("I/O error")
//                 } else {
//                     fs::File::create(uri_path).expect("I/O error")
//                 };

//                 let data = buffers[buffer.index()].clone();
//                 let bin = to_padded_byte_vector(data.0);
//                 writer.write_all(&bin).expect("I/O error");
//                 Ok(())
//             }
//             Source::Bin => Err(Error::Io(std::io::Error::new(
//                 ErrorKind::InvalidData,
//                 "Buffer has no URI set",
//             ))),
//         }
//         .expect("Validation error");
//     }

//     let writer = fs::File::create(gltf_path).expect("I/O error");
//     let root = document.into_json();
//     json::serialize::to_writer_pretty(writer, &root).expect("Serialization error");

//     Ok(())
// }

// /// Todo: need docs
// pub fn export_to_glb(
//     path: &str,
//     document: Document,
//     buffers: Vec<Blob>,
//     // _image_buffers: Vec<image::Data>,
// ) -> Result<(), Error> {
//     let glb_path = path::Path::new(&path).with_extension("glb");
//     let directory = glb_path.parent();
//     if let Some(dir) = directory {
//         fs::create_dir_all(dir).expect("I/O error");
//     }

//     // create a single blob of data from each buffer and image
//     let mut blob = Vec::<u8>::new();
//     for buffer in document.buffers() {
//         let data = buffers[buffer.index()].clone();
//         blob.extend(data.0);
//     }

//     // todo: write the image to the blob
//     // for image in document.images() {
//     //     let data = image_buffers[image.index()].clone();
//     //     blob.extend(data.);
//     // }

//     // serialise the json to a string
//     let root = document.into_json();
//     let json_string = json::serialize::to_string(&root).expect("Serialization error");
//     let mut json_offset = json_string.len() as u32;
//     align_to_multiple_of_four(&mut json_offset);

//     let glb = GlbReader {
//         header: Header {
//             magic: *b"glTF",
//             version: 2,
//             length: json_offset + blob.len() as u32,
//         },
//         bin: Some(Cow::Owned(to_padded_byte_vector(blob))),
//         json: Cow::Owned(json_string.into_bytes()),
//     };

//     let writer = std::fs::File::create(glb_path).expect("I/O error");
//     glb.to_writer(writer).expect("glTF binary output error");
//     Ok(())
// }
