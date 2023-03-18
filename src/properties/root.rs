use super::extras_extensions::ExtrasExtension;

/// Todo: Add docs
pub struct Scene(gltf_json::Scene);
/// Todo: Add docs
pub struct Accessor;
/// Todo: Add docs
pub struct Animation(gltf_json::Animation);
/// Todo: Add docs
pub struct Camera(gltf_json::Camera);
/// Todo: Add docs
pub struct Material(gltf_json::Material);
/// Todo: Add docs
pub struct Mesh(gltf_json::Mesh);
/// Todo: Add docs
pub struct Node(gltf_json::Node);
/// Todo: Add docs
pub struct Skin(gltf_json::Skin);
/// Todo: Add docs
pub struct Texture(gltf_json::Texture);
/// Todo: Add docs
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
    pub fn from_json(json: &gltf_json::Root) -> Self {
        Root {
            asset: json.asset.clone(),
            default_scene: json.scene.map(|scene| scene.value()),
            accessors: json.accessors.iter().map(|_| Accessor).collect(),
            animations: json
                .animations
                .iter()
                .map(|animation| Animation(animation.clone()))
                .collect(),
            cameras: json
                .cameras
                .iter()
                .map(|camera| Camera(camera.clone()))
                .collect(),
            materials: json
                .materials
                .iter()
                .map(|material| Material(material.clone()))
                .collect(),
            meshes: json.meshes.iter().map(|mesh| Mesh(mesh.clone())).collect(),
            nodes: json.nodes.iter().map(|node| Node(node.clone())).collect(),
            scenes: json
                .scenes
                .iter()
                .map(|scene| Scene(scene.clone()))
                .collect(),
            skins: json.skins.iter().map(|skin| Skin(skin.clone())).collect(),
            textures: json
                .textures
                .iter()
                .map(|texture| Texture(texture.clone()))
                .collect(),
            images: json
                .images
                .iter()
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
