use std::io::prelude;

mod accessor;
mod animation;
mod buffer;
mod camera;
mod extras_extensions;
mod image;
mod material;
mod mesh;
mod node;
mod root;
mod scene;
mod skin;
mod texture;
mod traits;

// ---- Core Properties ---------------------------------------------------------
pub mod prelude {
    pub use crate::accessor::Accessor;
    pub use crate::animation::Animation;
    pub use crate::buffer::Buffer;
    pub use crate::camera::Camera;
    pub use crate::extras_extensions::ExtrasExtensions;
    pub use crate::image::Image;
    pub use crate::material::Material;
    pub use crate::mesh::Mesh;
    pub use crate::node::Node;
    pub use crate::root::Root;
    pub use crate::scene::Scene;
    pub use crate::skin::Skin;
    pub use crate::texture::Texture;
}

// ---- Buffer bits for IO ------------------------------------------------------
pub use crate::buffer::Data as BufferData;
pub use crate::buffer::View as BufferView;
