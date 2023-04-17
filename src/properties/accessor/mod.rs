mod element;
mod generic_accessor;

use super::buffer::Blob;
#[cfg(feature = "extensions")]
use super::extension::Extension;
use crate::properties::traits::{FromJson, ToJson};
use element::*;
use generic_accessor::GenericAccessor;
use json::accessor::{ComponentType, Type as ElementType};

// ---- Generic Accessor Aliases -----------
// todo: add default to generic to make these more useful
pub type ScalarU16 = GenericAccessor<u16, Scalar<u16>>;
pub type ScalarF32 = GenericAccessor<f32, Scalar<f32>>;

pub type Vec2U16 = GenericAccessor<u16, Vec2<u16>>;
pub type Vec2F32 = GenericAccessor<f32, Vec2<f32>>;

pub type Vec3U16 = GenericAccessor<u16, Vec3<u16>>;
pub type Vec3F32 = GenericAccessor<f32, Vec3<f32>>;

pub type Vec4Mat2U16 = GenericAccessor<u16, Vec4Mat2<u16>>;
pub type Vec4Mat2F32 = GenericAccessor<f32, Vec4Mat2<f32>>;

pub type Mat3U16 = GenericAccessor<u16, Mat3<u16>>;
pub type Mat3F32 = GenericAccessor<f32, Mat3<f32>>;

pub type Mat4U16 = GenericAccessor<u16, Mat4<u16>>;
pub type Mat4F32 = GenericAccessor<f32, Mat4<f32>>;

// TODO: Add default for each alias
// impl Default for GenericAccessor<u16, Scalar<u16>> {
//     fn default() -> Self {
//         Self {
//             phantom: PhantomData,
//             array: Some(Vec::<Scalar<u16>>::new()),
//             component_type: ComponentType::U16,
//             element_type: ElementType::Scalar,
//             normalized: false,
//             sparse: false,
//             name: None,
//             extension: ExtrasExtension::default(),
//         }
//     }
// }

// ---- Accessor Enum ---------------------
#[derive(Clone, Debug)]
pub enum Accessor {
    ScalarU16(ScalarU16),
    ScalarF32(ScalarF32),

    Vec2U16(Vec2U16),
    Vec2F32(Vec2F32),

    Vec3U16(Vec3U16),
    Vec3F32(Vec3F32),

    Vec4U16(Vec4Mat2U16),
    Vec4F32(Vec4Mat2F32),

    Mat2U16(Vec4Mat2U16),
    Mat2F32(Vec4Mat2F32),

    Mat3U16(Mat3U16),
    Mat3F32(Mat3F32),

    Mat4U16(Mat4U16),
    Mat4F32(Mat4F32),
}

// ----- Utils ---------------------------------------------------------------------

/// Create accessors from the deserialised json struct
pub fn create_accessor_from_json(
    accessor_json: &json::Accessor,
    root: &json::Root,
    buffer: &[Blob],
) -> Accessor {
    let component_type = accessor_json.component_type.unwrap().0;
    match component_type {
        ComponentType::U16 => create_u16_accessor_from_json(accessor_json, root, buffer),
        ComponentType::F32 => create_f32_accessor_from_json(accessor_json, root, buffer),
        _ => todo!(),
    }
}

/// Create accessors with u16 data from the deserialised json struct
fn create_u16_accessor_from_json(
    accessor_json: &json::Accessor,
    root: &json::Root,
    buffer: &[Blob],
) -> Accessor {
    let element_type = accessor_json.type_.unwrap();
    match element_type {
        ElementType::Scalar => {
            Accessor::ScalarU16(ScalarU16::from_json(accessor_json, root, buffer))
        }
        ElementType::Vec2 => Accessor::Vec2U16(Vec2U16::from_json(accessor_json, root, buffer)),
        ElementType::Vec3 => Accessor::Vec3U16(Vec3U16::from_json(accessor_json, root, buffer)),
        ElementType::Vec4 => Accessor::Vec4U16(Vec4Mat2U16::from_json(accessor_json, root, buffer)),
        ElementType::Mat2 => Accessor::Mat2U16(Vec4Mat2U16::from_json(accessor_json, root, buffer)),
        ElementType::Mat3 => Accessor::Mat3U16(Mat3U16::from_json(accessor_json, root, buffer)),
        ElementType::Mat4 => Accessor::Mat4U16(Mat4U16::from_json(accessor_json, root, buffer)),
    }
}

/// Create accessors with f32 data from the deserialised json struct
fn create_f32_accessor_from_json(
    accessor_json: &json::Accessor,
    root: &json::Root,
    buffer: &[Blob],
) -> Accessor {
    let element_type = accessor_json.type_.unwrap();
    match element_type {
        ElementType::Scalar => {
            Accessor::ScalarF32(ScalarF32::from_json(accessor_json, root, buffer))
        }
        ElementType::Vec2 => Accessor::Vec2F32(Vec2F32::from_json(accessor_json, root, buffer)),
        ElementType::Vec3 => Accessor::Vec3F32(Vec3F32::from_json(accessor_json, root, buffer)),
        ElementType::Vec4 => Accessor::Vec4F32(Vec4Mat2F32::from_json(accessor_json, root, buffer)),
        ElementType::Mat2 => Accessor::Mat2F32(Vec4Mat2F32::from_json(accessor_json, root, buffer)),
        ElementType::Mat3 => Accessor::Mat3F32(Mat3F32::from_json(accessor_json, root, buffer)),
        ElementType::Mat4 => Accessor::Mat4F32(Mat4F32::from_json(accessor_json, root, buffer)),
    }
}

// ----- Tests ---------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gltf_io::import;

    #[test]
    fn test_round_trip() {
        let gltf = import("/Users/seanrennie/Development/PERSONAL/gltf-transform-rs/tests/minimal_accessor_min_max.gltf").unwrap();
        let root = gltf.root;
        let json_accessor = &root.accessors[1];
        let accessor = create_accessor_from_json(json_accessor, &root, &gltf.buffers);

        let (accessor_to_json, _view_to_json) = match accessor {
            Accessor::ScalarU16(accessor) => accessor.to_json(0),
            Accessor::Vec3F32(accessor) => accessor.to_json(1),
            _ => todo!(),
        };

        assert_eq!(accessor_to_json.buffer_view, json_accessor.buffer_view);
        assert_eq!(accessor_to_json.byte_offset, json_accessor.byte_offset);
        assert_eq!(
            accessor_to_json.component_type.unwrap().0,
            json_accessor.component_type.unwrap().0
        );
        assert_eq!(accessor_to_json.count, json_accessor.count);
        assert_eq!(accessor_to_json.type_, json_accessor.type_);
        // assert_eq!(accessor_to_json.min, json_accessor.min);
        // assert_eq!(accessor_to_json.max, json_accessor.max);
    }

    #[test]
    fn test_scalar_u16_accessor() {
        let gltf = import("/Users/seanrennie/Development/PERSONAL/gltf-transform-rs/tests/minimal_accessor_min_max.gltf").unwrap();
        let root = gltf.root;
        let json_accessor = &root.accessors[0];
        let accessor = create_accessor_from_json(json_accessor, &root, &gltf.buffers);

        match accessor {
            Accessor::ScalarU16(accessor) => {
                assert_eq!(accessor.get_count(), 3);
                assert_eq!(accessor.get_type(), ElementType::Scalar);
                assert_eq!(accessor.get_element_size(), 1);
                assert_eq!(accessor.get_component_size(), 2);
                assert_eq!(accessor.get_byte_length(), 6);
                let expected_element: &[u16; 1] = &[0];
                assert_eq!(accessor.get_element(0), Some(expected_element));
            }
            _ => unimplemented!(),
        }
    }

    #[test]
    fn test_vec3_f32_accessor() {
        let gltf = import("/Users/seanrennie/Development/PERSONAL/gltf-transform-rs/tests/minimal_accessor_min_max.gltf").unwrap();
        let root = gltf.root;
        let json_accessor = &root.accessors[1];
        let accessor = create_accessor_from_json(json_accessor, &root, &gltf.buffers);

        match accessor {
            Accessor::Vec3F32(accessor) => {
                assert_eq!(accessor.get_count(), 3);
                assert_eq!(accessor.get_type(), ElementType::Vec3);
                assert_eq!(accessor.get_element_size(), 3);
                assert_eq!(accessor.get_component_size(), 4);
                assert_eq!(accessor.get_byte_length(), 12);
                let expected_element: &[f32; 3] = &[0.0, 0.0, 0.0];
                assert_eq!(accessor.get_element(0), Some(expected_element));
            }
            _ => unimplemented!(),
        }
    }
}
