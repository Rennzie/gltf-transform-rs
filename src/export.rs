use std::borrow::Cow;

use crate::binary;
use crate::buffer;
use buffer::{align_to_multiple_of_four, to_padded_byte_vector, Data};

use crate::{Document, Error};

/// Todo: need docs
pub fn export_to_glb(
    path: &str,
    document: Document,
    buffers: Vec<Data>,
    // _image_buffers: Vec<image::Data>,
) -> Result<(), Error> {
    // create a single blob of data from each buffer and image
    let mut blob = Vec::<u8>::new();
    for buffer in document.buffers() {
        let data = buffers[buffer.index()].clone();
        blob.extend(data.0);
    }

    // todo: write the image to the blob
    // for image in document.images() {
    //     let data = image_buffers[image.index()].clone();
    //     blob.extend(data.);
    // }

    // serialise the json to a string
    let root = document.into_json();
    let json_string = json::serialize::to_string(&root).expect("Serialization error");
    let mut json_offset = json_string.len() as u32;
    align_to_multiple_of_four(&mut json_offset);

    let glb = binary::Glb {
        header: binary::Header {
            magic: *b"glTF",
            version: 2,
            length: json_offset + blob.len() as u32,
        },
        bin: Some(Cow::Owned(to_padded_byte_vector(blob))),
        json: Cow::Owned(json_string.into_bytes()),
    };

    let writer = std::fs::File::create(path).expect("I/O error");
    glb.to_writer(writer).expect("glTF binary output error");
    Ok(())
}
