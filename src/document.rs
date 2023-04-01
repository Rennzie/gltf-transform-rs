use crate::properties::prelude::*;
use crate::properties::BufferData;

pub struct Document {
    pub root: Root,
}

impl Document {
    pub fn from_json(root_json: gltf_json::Root, blob: Option<Vec<BufferData>>) -> Self {
        if let Some(buffer) = blob {
            Self {
                root: Root::from_json(root_json, buffer),
            }
        } else {
            // TODO: Fix the empty buffer case
            Self {
                root: Root::from_json(root_json, Vec::new()),
            }
        }
    }

    /// Returns the default scene.
    pub fn default_scene(&self) -> Option<&Scene> {
        self.root.get_default_scene()
    }

    /// Returns the scene with the given index.
    pub fn scene(&self, index: usize) -> Option<&Scene> {
        self.root.scenes[index]
    }

    /// Returns an iterator over the scenes.
    pub fn scenes(&self) -> impl Iterator<Item = &Scene> {
        self.root.scenes.iter()
    }

    /// Returns an iterator over the accessors.
    pub fn accessors(&self) -> impl Iterator<Item = &Accessor> {
        self.root.accessors.iter()
    }

    /// Returns an iterator over the animations.
    pub fn animations(&self) -> impl Iterator<Item = &Animation> {
        self.root.animations.iter()
    }

    /// Returns an iterator over the cameras.
    pub fn cameras(&self) -> impl Iterator<Item = &Camera> {
        self.root.cameras.iter()
    }

    /// Returns an iterator over the materials.
    pub fn materials(&self) -> impl Iterator<Item = &Material> {
        self.root.materials.iter()
    }

    /// Returns an iterator over the meshes.
    pub fn meshes(&self) -> impl Iterator<Item = &Mesh> {
        self.root.meshes.iter()
    }

    /// Returns an iterator over the nodes.
    pub fn nodes(&self) -> impl Iterator<Item = &Node> {
        self.root.nodes.iter()
    }

    /// Returns an iterator over the skins.
    pub fn skins(&self) -> impl Iterator<Item = &Skin> {
        self.root.skins.iter()
    }

    /// Returns an iterator over the textures.
    pub fn textures(&self) -> impl Iterator<Item = &Texture> {
        self.root.textures.iter()
    }

    /// Returns an iterator over the images.
    pub fn images(&self) -> impl Iterator<Item = &Image> {
        self.root.images.iter()
    }
}
