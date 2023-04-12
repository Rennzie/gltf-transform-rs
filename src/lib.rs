// #![deny(missing_docs)]
// #![allow(unknown_lints)]
// #![cfg_attr(docsrs, feature(doc_cfg))]

pub mod document;
pub mod gltf;
pub mod gltf_io;
mod math;
pub mod properties;
pub mod utils;

// ---- Core ---------------------------------------------------------
pub mod prelude {
    pub use crate::document::Document;
    pub use crate::gltf_io::export;
    pub use crate::gltf_io::import;
    pub use crate::properties::accessor::Accessor;
    pub use crate::properties::animation::Animation;
    pub use crate::properties::buffer::Buffer;
    pub use crate::properties::camera::Camera;
    // pub use crate::properties::extension::ExtrasExtensions;
    pub use crate::properties::image::Image;
    pub use crate::properties::material::Material;
    pub use crate::properties::mesh::Mesh;
    pub use crate::properties::node::Node;
    pub use crate::properties::root::Root;
    pub use crate::properties::scene::Scene;
    pub use crate::properties::skin::Skin;
    pub use crate::properties::texture::Texture;
}

// ---- External Crates ---------------------------------------------------------
// #[cfg(test)]
// #[macro_use]
// extern crate approx;
// #[macro_use]
// extern crate lazy_static;
pub extern crate gltf_json as json;
pub extern crate image as image_crate;
