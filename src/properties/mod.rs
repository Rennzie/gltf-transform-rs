pub mod accessor;
mod extras_extensions;
pub mod root;
mod traits;

pub struct Scene(gltf_json::Scene);

pub struct Animation(gltf_json::Animation);

pub struct Camera(gltf_json::Camera);

pub struct Material(gltf_json::Material);

pub struct Mesh(gltf_json::Mesh);

pub struct Node(gltf_json::Node);

pub struct Skin(gltf_json::Skin);

pub struct Texture(gltf_json::Texture);

pub struct Image(gltf_json::Image);
