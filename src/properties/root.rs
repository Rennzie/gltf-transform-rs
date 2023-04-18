#[cfg(feature = "extensions")]
use super::extension::Extension;
use super::{accessor::create_accessor_from_json, buffer::Blob};
use crate::prelude::*;
use rayon::prelude::*;

/// The Root of the glTF asset.
/// All properties are directly accessible form the root
#[derive(Debug, Clone)]
pub struct Root<'a> {
    /// The a buffer where accessor data will be written at export.
    /// Set the buffers Source to determine how that buffer is written.
    /// By default the root has one buffer which all accessors will reference after import.
    buffers: Vec<Buffer<'a>>,

    /// Metadata about the glTF asset.
    pub asset: json::Asset,
    // Reference to the default scene.
    pub default_scene: Option<usize>,

    /// An array of accessors.
    pub accessors: Vec<Accessor>,

    /// An array of keyframe animations.
    pub animations: Vec<Animation>,

    /// An array of cameras.
    pub cameras: Vec<Camera>,

    /// An array of materials.
    pub materials: Vec<Material>,

    /// An array of meshes.
    pub meshes: Vec<Mesh>,

    /// An array of nodes.
    pub nodes: Vec<Node>,

    /// An array of scenes.
    pub scenes: Vec<Scene>,

    /// An array of skins.
    pub skins: Vec<Skin>,

    /// An array of textures.
    pub textures: Vec<Texture>,
    // An array of images.
    pub images: Vec<Image>,

    pub extras: json::Extras,

    #[cfg(feature = "extensions")]
    /// Extras and extensions.
    extension: Option<Extension<json::extensions::Root>>,
}

impl Root<'_> {
    pub fn from_json(root_json: json::Root, buffer: Vec<Blob>) -> Self {
        Root {
            buffers: vec![Buffer::default()],
            asset: root_json.asset.clone(),
            default_scene: root_json.scene.map(|scene| scene.value()),
            accessors: root_json
                .accessors
                .par_iter()
                .map(|accessor_json| create_accessor_from_json(accessor_json, &root_json, &buffer))
                .collect(),
            animations: root_json
                .animations
                .par_iter()
                .map(Animation::new)
                .collect(),
            cameras: root_json
                .cameras
                .par_iter()
                .map(Camera::from_json)
                .collect(),
            materials: root_json.materials.par_iter().map(Material::new).collect(),
            meshes: root_json.meshes.par_iter().map(Mesh::new).collect(),
            nodes: root_json.nodes.par_iter().map(Node::new).collect(),
            scenes: root_json.scenes.par_iter().map(Scene::new).collect(),
            skins: root_json.skins.par_iter().map(Skin::new).collect(),
            textures: root_json.textures.par_iter().map(Texture::new).collect(),
            images: root_json.images.par_iter().map(Image::new).collect(),
            extras: root_json.extras,
            #[cfg(feature = "extensions")]
            extension: None,
        }
    }

    /// Returns the metadata about the glTF asset.
    pub fn asset(&self) -> &json::Asset {
        &self.asset
    }

    /// Returns the default scene.
    pub fn get_default_scene(&self) -> Option<&Scene> {
        match self.default_scene {
            Some(index) => Some(&self.scenes[index]),
            None => None,
        }
    }

    #[cfg(feature = "extensions")]
    pub fn get_extension(&self) -> Option<&Extension<json::extensions::Root>> {
        self.extension.as_ref()
    }
}

// TODO: revisit the root implementation
impl Default for Root<'_> {
    fn default() -> Self {
        Root {
            asset: json::Asset {
                version: "2.0".to_string(),
                ..Default::default()
            },
            default_scene: None,
            buffers: vec![Buffer::default()],
            accessors: Vec::new(),
            animations: Vec::new(),
            cameras: Vec::new(),
            materials: Vec::new(),
            meshes: Vec::new(),
            nodes: Vec::new(),
            scenes: Vec::new(),
            skins: Vec::new(),
            textures: Vec::new(),
            images: Vec::new(),
            extras: json::Extras::default(),
            #[cfg(feature = "extensions")]
            extension: None,
        }
    }
}
