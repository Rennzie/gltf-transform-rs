use super::accessor::{create_accessor_from_json, Accessor};
use super::extras_extensions::ExtrasExtension;
use crate::buffer::Data;
use rayon::prelude::*;

pub struct Scene(gltf_json::Scene);

pub struct Animation(gltf_json::Animation);

pub struct Camera(gltf_json::Camera);

pub struct Material(gltf_json::Material);

pub struct Mesh(gltf_json::Mesh);

pub struct Node(gltf_json::Node);

pub struct Skin(gltf_json::Skin);

pub struct Texture(gltf_json::Texture);

pub struct Image(gltf_json::Image);

/// The Root of the glTF asset.
/// All properties are directly accessible form the root
pub struct Root {
    /// Metadata about the glTF asset.
    asset: gltf_json::Asset,
    // Reference to the default scene.
    default_scene: Option<usize>,

    /// An array of accessors.
    accessors: Vec<Accessor>,
    // /// An array of buffers.
    //  buffers: Vec<Buffer>,
    // /// An array of buffer views.
    //  buffer_views: Vec<buffer::View>,
    /// An array of keyframe animations.
    animations: Vec<Animation>,

    /// An array of cameras.
    cameras: Vec<Camera>,

    /// An array of materials.
    materials: Vec<Material>,

    /// An array of meshes.
    meshes: Vec<Mesh>,

    /// An array of nodes.
    nodes: Vec<Node>,

    /// An array of scenes.
    scenes: Vec<Scene>,

    /// An array of skins.
    skins: Vec<Skin>,

    /// An array of textures.
    textures: Vec<Texture>,
    // An array of images.
    images: Vec<Image>,

    /// Extras and extensions.
    extras_extensions: Option<ExtrasExtension>,
}

impl Root {
    pub fn from_json(root_json: &gltf_json::Root, buffer: &[Data]) -> Self {
        Root {
            asset: root_json.asset.clone(),
            default_scene: root_json.scene.map(|scene| scene.value()),
            accessors: root_json
                .accessors
                .par_iter()
                .map(|accessor_json| create_accessor_from_json(accessor_json, root_json, buffer))
                .collect(),
            animations: root_json
                .animations
                .par_iter()
                .map(|animation| Animation(animation.clone()))
                .collect(),
            cameras: root_json
                .cameras
                .par_iter()
                .map(|camera| Camera(camera.clone()))
                .collect(),
            materials: root_json
                .materials
                .par_iter()
                .map(|material| Material(material.clone()))
                .collect(),
            meshes: root_json
                .meshes
                .par_iter()
                .map(|mesh| Mesh(mesh.clone()))
                .collect(),
            nodes: root_json
                .nodes
                .par_iter()
                .map(|node| Node(node.clone()))
                .collect(),
            scenes: root_json
                .scenes
                .par_iter()
                .map(|scene| Scene(scene.clone()))
                .collect(),
            skins: root_json
                .skins
                .par_iter()
                .map(|skin| Skin(skin.clone()))
                .collect(),
            textures: root_json
                .textures
                .par_iter()
                .map(|texture| Texture(texture.clone()))
                .collect(),
            images: root_json
                .images
                .par_iter()
                .map(|image| Image(image.clone()))
                .collect(),
            extras_extensions: None,
        }
    }

    /// Returns the metadata about the glTF asset.
    pub fn asset(&self) -> &gltf_json::Asset {
        &self.asset
    }

    /// Returns the default scene.
    pub fn get_default_scene(&self) -> Option<&Scene> {
        match self.default_scene {
            Some(index) => Some(&self.scenes[index]),
            None => None,
        }
    }

    pub fn get_extras_extensions(&self) -> Option<&ExtrasExtension> {
        self.extras_extensions.as_ref()
    }

    // Todo: Move iterators to the Document

    /// Returns an iterator over the scenes.
    pub fn scenes(&self) -> impl Iterator<Item = &Scene> {
        self.scenes.iter()
    }

    /// Returns an iterator over the accessors.
    pub fn accessors(&self) -> impl Iterator<Item = &Accessor> {
        self.accessors.iter()
    }

    /// Returns an iterator over the animations.
    pub fn animations(&self) -> impl Iterator<Item = &Animation> {
        self.animations.iter()
    }

    /// Returns an iterator over the cameras.
    pub fn cameras(&self) -> impl Iterator<Item = &Camera> {
        self.cameras.iter()
    }

    /// Returns an iterator over the materials.
    pub fn materials(&self) -> impl Iterator<Item = &Material> {
        self.materials.iter()
    }

    /// Returns an iterator over the meshes.
    pub fn meshes(&self) -> impl Iterator<Item = &Mesh> {
        self.meshes.iter()
    }

    /// Returns an iterator over the nodes.
    pub fn nodes(&self) -> impl Iterator<Item = &Node> {
        self.nodes.iter()
    }

    /// Returns an iterator over the skins.
    pub fn skins(&self) -> impl Iterator<Item = &Skin> {
        self.skins.iter()
    }

    /// Returns an iterator over the textures.
    pub fn textures(&self) -> impl Iterator<Item = &Texture> {
        self.textures.iter()
    }

    /// Returns an iterator over the images.
    pub fn images(&self) -> impl Iterator<Item = &Image> {
        self.images.iter()
    }
}

// TODO: revisit the root implementation
impl Default for Root {
    fn default() -> Self {
        Root {
            asset: gltf_json::Asset {
                version: "2.0".to_string(),
                ..Default::default()
            },
            default_scene: None,
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
            extras_extensions: None,
        }
    }
}
