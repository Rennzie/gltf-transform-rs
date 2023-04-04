use super::glb_reader::GlbReader;
use super::Result;
use std::{fs, io, ops, path::Path};

/// glTF JSON wrapper plus binary payload.
#[derive(Clone, Debug)]
pub struct GltfReader {
    /// Deserialised glTF JSON
    pub root_json: json::Root,
    /// The glTF binary payload in the case of binary glTF.
    pub blob: Option<Vec<u8>>,
}

impl GltfReader {
    /// Convenience function that loads glTF from the file system.
    pub fn open<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let file = fs::File::open(path)?;
        let reader = io::BufReader::new(file);
        let gltf = Self::from_reader(reader)?;
        Ok(gltf)
    }

    /// Loads glTF from a reader without performing validation checks.
    pub fn from_reader_without_validation<R>(mut reader: R) -> Result<Self>
    where
        R: io::Read + io::Seek,
    {
        let mut magic = [0u8; 4];
        reader.read_exact(&mut magic)?;
        reader.seek(io::SeekFrom::Start(0))?;

        // NOTE: Weird indirection for reading a GLB!

        let (json, blob): (json::Root, Option<Vec<u8>>);
        if magic.starts_with(b"glTF") {
            let mut glb = GlbReader::from_reader(reader)?;
            // TODO: use `json::from_reader` instead of `json::from_slice`
            json = json::deserialize::from_slice(&glb.json)?;
            blob = glb.bin.take().map(|x| x.into_owned());
        } else {
            let json = json::deserialize::from_reader(reader)?;
            blob = None;
        };

        let json = json::deserialize::from_reader(reader)?;
        // let document = Document::from_json_without_validation(json);
        Ok(GltfReader {
            root_json: json,
            blob,
        })
    }

    /// Loads glTF from a reader.
    pub fn from_reader<R>(reader: R) -> Result<Self>
    where
        R: io::Read + io::Seek,
    {
        let gltf = Self::from_reader_without_validation(reader)?;
        // let _ = gltf.root_json.validate();
        Ok(gltf)
    }

    /// Loads glTF from a slice of bytes without performing validation
    /// checks.
    pub fn from_slice_without_validation(slice: &[u8]) -> Result<Self> {
        let (json, blob): (json::Root, Option<Vec<u8>>);
        if slice.starts_with(b"glTF") {
            let mut glb = GlbReader::from_slice(slice)?;
            json = json::deserialize::from_slice(&glb.json)?;
            blob = glb.bin.take().map(|x| x.into_owned());
        } else {
            json = json::deserialize::from_slice(slice)?;
            blob = None;
        };
        // let document = Document::from_json_without_validation(json);
        Ok(GltfReader {
            root_json: json,
            blob,
        })
    }

    /// Loads glTF from a slice of bytes.
    pub fn from_slice(slice: &[u8]) -> Result<Self> {
        let gltf = Self::from_slice_without_validation(slice)?;
        // let _ = gltf.document.validate()?;
        Ok(gltf)
    }
}

impl ops::Deref for GlbReader<'_> {
    type Target = json::Root;
    fn deref(&self) -> &Self::Target {
        &self
    }
}

// impl ops::DerefMut for GltfReader {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self
//     }
// }
